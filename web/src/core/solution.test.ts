import { expect, test } from "vite-plus/test";
import { newRun, type Run, type RunStatus } from "./run.ts";
import { hasFinishedRun, solutionAccess } from "./solution.ts";

const run = (status: RunStatus): Run => ({
  ...newRun({ id: status, problemId: "p", mode: "standard", lives: null, now: 0 }),
  status,
});

test.each([
  [[], false],
  [[run("playing")], false],
  [[run("playing"), run("solved")], true],
  [[run("failed")], true],
])("hasFinishedRun(%j) = %s", (runs, expected) => {
  expect(hasFinishedRun(runs)).toBe(expected);
});

test.each([
  [{ inChallenge: true, revealed: true, finished: true }, "hidden"],
  [{ inChallenge: false, revealed: true, finished: false }, "revealed"],
  [{ inChallenge: false, revealed: false, finished: true }, "unlocked"],
  [{ inChallenge: false, revealed: false, finished: false }, "locked"],
] as const)("solutionAccess(%j) = %s", (params, expected) => {
  expect(solutionAccess(params)).toBe(expected);
});
