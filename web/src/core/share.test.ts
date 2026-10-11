import { expect, test } from "vite-plus/test";
import type { Judgement, Verdict } from "./classify.ts";
import { MESSAGES } from "./i18n.ts";
import { newRound, recordRun } from "./round.ts";
import type { Run } from "./run.ts";
import { roundShareCard, shareCard } from "./share.ts";

test("a solved run", () => {
  const card = shareCard(
    {
      problemId: "lifetime-01",
      problemTitle: "Longest word",
      verdicts: ["CE", "WA", "AC"],
      solved: true,
    },
    MESSAGES.en,
  );
  expect(card).toEqual({
    headline: "Solved!",
    solved: true,
    title: "Longest word",
    detail: "3 tries",
    rows: [
      {
        shots: [
          { label: "CE", tone: "ce" },
          { label: "WA", tone: "miss" },
          { label: "AC", tone: "ac" },
        ],
        score: { kind: "solved", shots: 3 },
        seed: "lifetime-01",
      },
    ],
  });
});

test("a hole in one", () => {
  const card = shareCard(
    {
      problemId: "iter-01",
      problemTitle: "Sum of even squares",
      verdicts: ["AC"],
      solved: true,
    },
    MESSAGES.en,
  );
  expect(card.headline).toBe("Hole in One!");
  expect(card.detail).toBe("1 try");
});

test("a failed run", () => {
  const card = shareCard(
    {
      problemId: "lifetime-01",
      problemTitle: "最長の単語",
      verdicts: ["CE", "UB"],
      solved: false,
    },
    MESSAGES.ja,
  );
  expect(card.headline).toBe(MESSAGES.ja.unsolvedHeadline);
  expect(card.detail).toBe("2回");
  expect(card.rows[0].score).toEqual({ kind: "unsolved" });
});

/** A finished run whose submissions got `kinds`, solved when the last one is `AC`. */
function run(id: string, problemId: string, kinds: Verdict["kind"][]): Run {
  const attempts = kinds.map((kind) => {
    const verdict = (kind === "CE" ? { kind, diagnostics: [] } : { kind, cases: [] }) as Verdict;
    const judgement: Judgement = { verdict, output: "", stderr: "", warnings: [] };
    return { submittedAt: 1, code: "", judgement };
  });
  return {
    id,
    problemId,
    mode: "standard",
    startedAt: 0,
    attempts,
    status: kinds.at(-1) === "AC" ? "solved" : "failed",
    finishedAt: 1,
  };
}

test("a round with every problem solved has a total", () => {
  let round = newRound({ id: "r", problemIds: ["a", "b"], now: 0 });
  round = recordRun(round, run("1", "a", ["CE", "AC"]));
  round = recordRun(round, run("2", "b", ["AC"]));
  expect(roundShareCard(round, "Oct 11, 2026", MESSAGES.en)).toEqual({
    headline: "Total 3",
    solved: true,
    title: "Oct 11, 2026",
    detail: "+1 over perfect",
    rows: [
      {
        shots: [
          { label: "CE", tone: "ce" },
          { label: "AC", tone: "ac" },
        ],
        score: { kind: "solved", shots: 2 },
        seed: "r:0",
      },
      { shots: [{ label: "AC", tone: "ac" }], score: { kind: "solved", shots: 1 }, seed: "r:1" },
    ],
  });
});

test("a round with a problem given up has no total, only how far it got", () => {
  let round = newRound({ id: "r", problemIds: ["a", "b"], now: 0 });
  round = recordRun(round, run("1", "a", ["AC"]));
  round = recordRun(round, run("2", "b", []));
  expect(roundShareCard(round, "Oct 11, 2026", MESSAGES.en)).toEqual({
    headline: "1/2 solved",
    solved: false,
    title: "Oct 11, 2026",
    detail: "",
    rows: [
      { shots: [{ label: "AC", tone: "ac" }], score: { kind: "solved", shots: 1 }, seed: "r:0" },
      { shots: [], score: { kind: "unsolved" }, seed: "r:1" },
    ],
  });
});
