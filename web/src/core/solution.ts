// When the model solution may be shown. It is a learning aid, so it opens up once the
// player is done with the problem, and seeing it turns later runs into practice.
import type { Run } from "./run.ts";

export type SolutionAccess =
  /** Not offered at all (inside a 連続チャレンジ). */
  | "hidden"
  /** The problem has not been finished yet; giving up reveals it. */
  | "locked"
  /** Finished (solved, out of lives or given up), but not seen yet. */
  | "unlocked"
  /** Already seen. */
  | "revealed";

/** Whether any run of the problem has ended, in free play or in a challenge. */
export function hasFinishedRun(runs: readonly Run[]): boolean {
  return runs.some((r) => r.status !== "playing");
}

export function solutionAccess(params: {
  inChallenge: boolean;
  revealed: boolean;
  finished: boolean;
}): SolutionAccess {
  if (params.inChallenge) return "hidden";
  if (params.revealed) return "revealed";
  return params.finished ? "unlocked" : "locked";
}
