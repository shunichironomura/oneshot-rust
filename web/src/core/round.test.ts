import { describe, expect, test } from "vite-plus/test";
import type { Judgement, Verdict } from "./classify.ts";
import { MESSAGES } from "./i18n.ts";
import {
  abandonRound,
  activeRound,
  currentProblemId,
  isComplete,
  newRound,
  pickProblem,
  pickRound,
  recordRun,
  type Round,
  roundShareText,
  roundSummary,
  seenProblemIds,
  skipCurrent,
  totalScore,
  totalSubmissions,
} from "./round.ts";
import type { Run, RunStatus } from "./run.ts";

function run(id: string, problemId: string, status: RunStatus, kinds: Verdict["kind"][]): Run {
  const attempts = kinds.map((kind) => {
    const verdict = (kind === "CE" ? { kind, diagnostics: [] } : { kind, cases: [] }) as Verdict;
    const judgement: Judgement = { verdict, output: "", stderr: "", warnings: [] };
    return { submittedAt: 5, code: "", judgement };
  });
  return {
    id,
    problemId,
    mode: "standard",
    startedAt: 0,
    attempts,
    status,
    finishedAt: 5,
  };
}

const start = (problemIds = ["a", "b"]) => newRound({ id: "r", problemIds, now: 0 });

let seed = 1;
const random = () => ((seed = (seed * 16807) % 2147483647) - 1) / 2147483646;

describe("pickRound", () => {
  /** `count` problems of each difficulty, named like "2c" (difficulty 2, third problem). */
  const problems = (count: number) =>
    ([1, 2, 3] as const).flatMap((difficulty) =>
      Array.from({ length: count }, (_, i) => ({
        id: `${difficulty}${String.fromCharCode(97 + i)}`,
        difficulty,
      })),
    );
  const nothing = { solved: new Set<string>(), seen: new Set<string>() };
  const difficulty = (id: string) => Number(id[0]);

  test("climbs the ladder with distinct problems", () => {
    const picked = pickRound(problems(4), nothing, random);
    expect(picked.map(difficulty)).toEqual([1, 2, 3]);
    expect(new Set(picked).size).toBe(3);
  });

  test("prefers problems not seen yet, then unsolved ones, then solved ones", () => {
    const played = {
      solved: new Set(["1a", "2a"]),
      seen: new Set(["1a", "1b", "2a", "3a"]),
    };
    for (let i = 0; i < 10; i++) {
      const picked = pickRound(problems(3), played, random);
      expect(picked[0]).toBe("1c");
      expect(["2b", "2c"]).toContain(picked[1]);
      expect(["3b", "3c"]).toContain(picked[2]);
    }
  });

  test("makes up for a missing difficulty with the others", () => {
    const picked = pickRound(
      problems(4).filter((p) => p.difficulty < 3),
      nothing,
      random,
    );
    const difficulties = picked.map(difficulty);
    expect(difficulties).toHaveLength(3);
    expect(difficulties).toEqual([...difficulties].sort((a, b) => a - b));
    expect(difficulties).toContain(1);
    expect(difficulties).toContain(2);
  });

  test("returns every problem when there are fewer than a round needs", () => {
    expect(pickRound(problems(1), nothing, random)).toEqual(["1a", "2a", "3a"]);
  });
});

describe("pickProblem", () => {
  const problems = ([1, 2, 3] as const).flatMap((difficulty) =>
    ["a", "b", "c"].map((name) => ({ id: `${difficulty}${name}`, difficulty })),
  );

  test("picks a problem of the given difficulty", () => {
    const nothing = { solved: new Set<string>(), seen: new Set<string>() };
    for (let i = 0; i < 10; i++) {
      expect(["2a", "2b", "2c"]).toContain(pickProblem(problems, nothing, 2, random));
    }
  });

  test("prefers problems not seen yet, then unsolved ones, then solved ones", () => {
    for (let i = 0; i < 10; i++) {
      expect(
        pickProblem(problems, { solved: new Set(["1a"]), seen: new Set(["1a", "1b"]) }, 1, random),
      ).toBe("1c");
      expect(
        pickProblem(
          problems,
          { solved: new Set(["1a", "1c"]), seen: new Set(["1a", "1b", "1c"]) },
          1,
          random,
        ),
      ).toBe("1b");
    }
  });

  test("returns undefined when there is no problem of the difficulty", () => {
    const easy = problems.filter((p) => p.difficulty === 1);
    expect(pickProblem(easy, { solved: new Set(), seen: new Set() }, 3, random)).toBeUndefined();
  });
});

test("activeRound finds the unfinished round", () => {
  const done: Round = { ...start(), finishedAt: 1 };
  const playing = { ...start(), id: "p" };
  expect(activeRound([done, playing])).toBe(playing);
  expect(activeRound([done])).toBeUndefined();
});

describe("recordRun", () => {
  test("advances through the problems and finishes", () => {
    let r = start();
    expect(currentProblemId(r)).toBe("a");
    r = recordRun(r, run("r1", "a", "solved", ["CE", "AC"]));
    expect(currentProblemId(r)).toBe("b");
    expect(r.finishedAt).toBeUndefined();
    r = recordRun(r, run("r2", "b", "failed", ["WA", "WA", "WA"]));
    expect(currentProblemId(r)).toBeUndefined();
    expect(r.finishedAt).toBe(5);
    expect(totalSubmissions(r)).toBe(5);
  });

  test("ignores unfinished runs, other problems and duplicates", () => {
    const r = start();
    expect(recordRun(r, run("r1", "a", "playing", ["CE"]))).toBe(r);
    expect(recordRun(r, run("r1", "b", "solved", ["AC"]))).toBe(r);
    const once = recordRun(r, run("r1", "a", "solved", ["AC"]));
    expect(recordRun(once, run("r1", "a", "solved", ["AC"]))).toBe(once);
  });
});

test("skipCurrent records the problem as unsolved", () => {
  let r = skipCurrent(start(), 3);
  expect(currentProblemId(r)).toBe("b");
  expect(r.entries[0]).toMatchObject({ problemId: "a", solved: false, verdicts: [] });
  r = skipCurrent(r, 4);
  expect(r.finishedAt).toBe(4);
  expect(skipCurrent(r, 5)).toBe(r);
});

describe("abandonRound", () => {
  test("records the current run and ends the round", () => {
    let r = recordRun(start(["a", "b", "c"]), run("r1", "a", "solved", ["AC"]));
    r = abandonRound(r, run("r2", "b", "failed", ["WA", "CE"]), 9);
    expect(r.finishedAt).toBe(9);
    expect(r.abandoned).toBe(true);
    expect(r.entries.map((e) => e.problemId)).toEqual(["a", "b"]);
    expect(activeRound([r])).toBeUndefined();
    expect(roundSummary(r, MESSAGES.en)).toBe("Abandoned (1/3 solved)");
    expect(roundSummary(r, MESSAGES.ja)).toBe("棄権（1/3 問正解）");
  });

  test("leaves a problem without submissions unrecorded", () => {
    const r = abandonRound(start(), run("r1", "a", "failed", []), 9);
    expect(r.entries).toEqual([]);
    expect(abandonRound(start(), undefined, 9).entries).toEqual([]);
  });

  test("leaves a finished round unchanged", () => {
    const done: Round = { ...start(), finishedAt: 1 };
    expect(abandonRound(done, undefined, 9)).toBe(done);
  });
});

describe("roundSummary", () => {
  test("measures a fully solved round against perfect", () => {
    let r = start();
    r = recordRun(r, run("r1", "a", "solved", ["CE", "AC"]));
    r = recordRun(r, run("r2", "b", "solved", ["WA", "AC"]));
    expect(roundSummary(r, MESSAGES.en)).toBe("Total 4 · +2 over perfect");
    expect(roundSummary(r, MESSAGES.ja)).toBe("合計 4 · 完璧まで +2");
  });

  test("a perfect round", () => {
    let r = start();
    r = recordRun(r, run("r1", "a", "solved", ["AC"]));
    r = recordRun(r, run("r2", "b", "solved", ["AC"]));
    expect(roundSummary(r, MESSAGES.en)).toBe("Total 2 · Perfect!");
  });
});

describe("totalScore", () => {
  test("is the submissions against par when every problem was solved", () => {
    let r = start();
    r = recordRun(r, run("r1", "a", "solved", ["CE", "AC"]));
    r = recordRun(r, run("r2", "b", "solved", ["AC"]));
    expect(isComplete(r)).toBe(true);
    expect(totalScore(r)).toEqual({ kind: "solved", shots: 3, par: 2 });
    expect(roundSummary(r, MESSAGES.en)).toBe("Total 3 · +1 over perfect");
  });

  test("is a cross when a problem was given up, however few submissions it took", () => {
    let r = start();
    r = recordRun(r, run("r1", "a", "solved", ["AC"]));
    r = recordRun(r, run("r2", "b", "failed", []));
    expect(isComplete(r)).toBe(false);
    expect(totalScore(r)).toEqual({ kind: "unsolved" });
    expect(roundSummary(r, MESSAGES.en)).toBe("1/2 solved");
  });

  test("is a cross when the round was abandoned", () => {
    let r = start();
    r = recordRun(r, run("r1", "a", "solved", ["AC"]));
    r = abandonRound(r, undefined, 9);
    expect(totalScore(r)).toEqual({ kind: "unsolved" });
    expect(roundSummary(r, MESSAGES.en)).toBe("Abandoned (1/2 solved)");
  });
});

test("roundShareText", () => {
  let r = start();
  r = recordRun(r, run("r1", "a", "solved", ["CE", "AC"]));
  r = recordRun(r, run("r2", "b", "failed", []));
  expect(roundShareText(r, MESSAGES.ja)).toBe("OneShot Rust 🦀\n🟥🟩\n— ✗\n1/2 問正解");
  expect(roundShareText(r, MESSAGES.en)).toBe("OneShot Rust 🦀\n🟥🟩\n— ✗\n1/2 solved");
  r = start();
  r = recordRun(r, run("r1", "a", "solved", ["CE", "AC"]));
  r = recordRun(r, run("r2", "b", "solved", ["AC"]));
  expect(roundShareText(r, MESSAGES.en)).toBe(
    "OneShot Rust 🦀\n🟥🟩\n🟩\nTotal 3 · +1 over perfect",
  );
});

describe("seenProblemIds", () => {
  test("is empty without rounds", () => {
    expect(seenProblemIds([])).toEqual(new Set());
  });

  test("includes played problems and the current one, but not those not reached", () => {
    const inProgress = recordRun(start(["a", "b", "c"]), run("1", "a", "solved", ["AC"]));
    expect(seenProblemIds([inProgress])).toEqual(new Set(["a", "b"]));
  });

  test("leaves out the problems an abandoned round did not reach", () => {
    const abandoned = abandonRound(start(["a", "b", "c"]), run("1", "a", "failed", ["WA"]), 9);
    expect(seenProblemIds([abandoned])).toEqual(new Set(["a"]));
  });

  test("merges the problems of several rounds", () => {
    const first = skipCurrent(skipCurrent(start(["a", "b"]), 1), 2);
    const second = start(["c", "a"]);
    expect(seenProblemIds([first, second])).toEqual(new Set(["a", "b", "c"]));
  });
});
