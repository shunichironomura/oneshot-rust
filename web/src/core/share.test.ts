import { expect, test } from "vite-plus/test";
import { MESSAGES } from "./i18n.ts";
import { newRound, recordRun } from "./round.ts";
import type { Run } from "./run.ts";
import { layoutShots, roundShareCard, shareCard } from "./share.ts";

test("a solved run", () => {
  const card = shareCard(
    {
      problemId: "lifetime-01",
      problemTitle: "Longest word",
      mode: "standard",
      verdicts: ["CE", "WA", "AC"],
      solved: true,
    },
    MESSAGES.en,
  );
  expect(card).toEqual({
    headline: "Solved!",
    solved: true,
    title: "Longest word",
    detail: "3 tries · Standard",
    rows: [
      [
        { label: "CE", tone: "ce" },
        { label: "WA", tone: "miss" },
        { label: "AC", tone: "ac" },
      ],
    ],
  });
});

test("a hole in one", () => {
  const card = shareCard(
    {
      problemId: "iter-01",
      problemTitle: "Sum of even squares",
      mode: "standard",
      verdicts: ["AC"],
      solved: true,
    },
    MESSAGES.en,
  );
  expect(card.headline).toBe("Hole in One!");
  expect(card.detail).toBe("1 try · Standard");
});

test("a failed run", () => {
  const card = shareCard(
    {
      problemId: "lifetime-01",
      problemTitle: "最長の単語",
      mode: "oni",
      verdicts: ["CE", "UB"],
      solved: false,
    },
    MESSAGES.ja,
  );
  expect(card.headline).toBe(MESSAGES.ja.unsolvedHeadline);
  expect(card.detail).toBe("2回 · 鬼");
});

test("a round", () => {
  const run = (id: string, problemId: string, solved: boolean): Run => ({
    ...newRunStub(id, problemId),
    status: solved ? "solved" : "failed",
  });
  let round = newRound({
    id: "r",
    course: 2,
    mode: "standard",
    problemIds: ["a", "b"],
    now: 0,
  });
  round = recordRun(round, run("1", "a", true));
  round = recordRun(round, run("2", "b", false));
  expect(roundShareCard(round, MESSAGES.en)).toEqual({
    headline: "Total 0",
    solved: false,
    title: "Intermediate course",
    detail: "1/2 solved · Standard",
    rows: [[], []],
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

test("few squares keep their maximum size", () => {
  expect(layoutShots([3], 1000, 1000, 80)).toEqual({ size: 80, step: 100 });
});

test("many squares shrink to fit the width", () => {
  const grid = layoutShots([20, 2], 990, 1000, 80);
  expect(grid.size).toBeCloseTo(40);
  expect(19 * grid.step + grid.size).toBeCloseTo(990);
});

test("many rows shrink to fit the height", () => {
  const grid = layoutShots([1, 1, 1, 1, 1], 1000, 195, 96);
  expect(4 * grid.step + grid.size).toBeCloseTo(195);
});

test("no squares", () => {
  expect(layoutShots([], 1000, 1000, 80).size).toBe(80);
});
