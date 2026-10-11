// When the model solution may be shown. It is a learning aid, so it appears once the player
// is done with the problem: on the play screen when the run ends (solved or given up), and
// in the history once any run has ended. Seeing it turns later runs into assisted ones.
import type { Run } from "./run.ts";

/** Whether any run of the problem has ended, in a round or in practice. */
export function hasFinishedRun(runs: readonly Run[]): boolean {
  return runs.some((r) => r.status !== "playing");
}
