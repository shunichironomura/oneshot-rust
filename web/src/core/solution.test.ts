import { expect, test } from "vite-plus/test";
import { newRun, type Run, type RunStatus } from "./run.ts";
import { hasFinishedRun } from "./solution.ts";

const run = (status: RunStatus): Run => ({
  ...newRun({ id: status, problemId: "p", now: 0 }),
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
