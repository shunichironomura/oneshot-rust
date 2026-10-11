import { describe, expect, test } from "vite-plus/test";
import type { Judgement, Verdict } from "./classify.ts";
import { MESSAGES } from "./i18n.ts";
import {
  abandonRound,
  activeRound,
  courseProblemIds,
  currentProblemId,
  newRound,
  pickProblems,
  recordRun,
  type Round,
  roundShareText,
  roundSummary,
  seenProblemIds,
  skipCurrent,
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

const start = (problemIds = ["a", "b"]) =>
  newRound({ id: "r", course: 1, mode: "standard", problemIds, now: 0 });

let seed = 1;
const random = () => ((seed = (seed * 16807) % 2147483647) - 1) / 2147483646;

test("courseProblemIds keeps the problems of one difficulty", () => {
  const problems = [
    { id: "a", difficulty: 1 as const },
    { id: "b", difficulty: 2 as const },
    { id: "c", difficulty: 1 as const },
  ];
  expect(courseProblemIds(problems, 1)).toEqual(["a", "c"]);
});

test("pickProblems returns distinct problems", () => {
  const picked = pickProblems(["a", "b", "c", "d", "e", "f"], new Set(), 5, random);
  expect(picked).toHaveLength(5);
  expect(new Set(picked).size).toBe(5);
});

test("pickProblems with fewer problems than requested returns all", () => {
  expect(pickProblems(["a", "b"], new Set(), 5, () => 0).sort()).toEqual(["a", "b"]);
});

test("pickProblems puts unsolved problems first", () => {
  const picked = pickProblems(["a", "b", "c", "d"], new Set(["a", "c"]), 3, random);
  expect(picked.slice(0, 2).sort()).toEqual(["b", "d"]);
  expect(["a", "c"]).toContain(picked[2]);
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
    expect(roundSummary(r, MESSAGES.en)).toBe("Total 3 · Abandoned (1/3 solved)");
    expect(roundSummary(r, MESSAGES.ja)).toBe("合計 3 · 棄権（1/3 問正解）");
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

test("roundShareText", () => {
  let r = start();
  r = recordRun(r, run("r1", "a", "solved", ["CE", "AC"]));
  r = recordRun(r, run("r2", "b", "failed", []));
  expect(roundShareText(r, MESSAGES.ja)).toBe(
    "OneShot Rust 🦀 初級 [標準]\n🟥🟩\n— ✗\n合計 2 · 1/2 問正解",
  );
  expect(roundShareText(r, MESSAGES.en)).toBe(
    "OneShot Rust 🦀 Beginner [Standard]\n🟥🟩\n— ✗\nTotal 2 · 1/2 solved",
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
