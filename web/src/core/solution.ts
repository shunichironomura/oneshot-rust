// When the model solution may be shown. It is a learning aid, so it opens up once the
// player is done with the problem, and seeing it turns later runs into assisted ones.
import type { Run } from "./run.ts";

export type SolutionAccess =
  /** The problem has not been finished yet; giving up reveals it. */
  | "locked"
  /** Finished (solved, out of lives or given up), but not seen yet. */
  | "unlocked"
  /** Already seen. */
  | "revealed";

/** Whether any run of the problem has ended, in a round or in practice. */
export function hasFinishedRun(runs: readonly Run[]): boolean {
  return runs.some((r) => r.status !== "playing");
}

export function solutionAccess(params: { revealed: boolean; finished: boolean }): SolutionAccess {
  if (params.revealed) return "revealed";
  return params.finished ? "unlocked" : "locked";
}
