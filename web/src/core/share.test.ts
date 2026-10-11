import { expect, test } from "vite-plus/test";
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

test("a round", () => {
  const run = (id: string, problemId: string, solved: boolean): Run => ({
    ...newRunStub(id, problemId),
    status: solved ? "solved" : "failed",
  });
  let round = newRound({ id: "r", problemIds: ["a", "b"], now: 0 });
  round = recordRun(round, run("1", "a", true));
  round = recordRun(round, run("2", "b", false));
  expect(roundShareCard(round, "Oct 11, 2026", MESSAGES.en)).toEqual({
    headline: "Total 0",
    solved: false,
    title: "Oct 11, 2026",
    detail: "1/2 solved",
    rows: [
      { shots: [], score: { kind: "solved", shots: 0 }, seed: "r:0" },
      { shots: [], score: { kind: "unsolved" }, seed: "r:1" },
    ],
  });
});

function newRunStub(id: string, problemId: string): Run {
  return {
    id,
    problemId,
    mode: "standard",
    startedAt: 0,
    attempts: [],
    status: "playing",
    finishedAt: 1,
  };
}
