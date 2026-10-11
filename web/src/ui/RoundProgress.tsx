import type { Round } from "../core/round.ts";

interface Props {
  round: Round;
  /** The problem being played (0-based), outlined. */
  current?: number;
  /** Accessible name; also shown as a tooltip. */
  label: string;
}

/**
 * One square per problem of a round, like the holes of a golf scorecard: a solved problem
 * shows its number of submissions, an unsolved one a cross, and the rest are empty.
 */
export function RoundProgress({ round, current, label }: Props) {
  return (
    <span class="round-progress" role="img" aria-label={label} title={label}>
      {round.problemIds.map((id, i) => {
        const entry = round.entries[i];
        const state = entry === undefined ? "" : entry.solved ? " solved" : " unsolved";
        return (
          <span key={`${i}:${id}`} class={`hole${state}${i === current ? " current" : ""}`}>
            {entry === undefined ? "" : entry.solved ? entry.verdicts.length : "✗"}
          </span>
        );
      })}
    </span>
  );
}
