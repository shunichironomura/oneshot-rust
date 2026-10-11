import { BOX, digitOf, marksFor, type Score } from "../core/marks.ts";

export type { Score } from "../core/marks.ts";

interface Props {
  score: Score;
  /** Decides the wobble of the lines, so that a mark keeps its shape from render to render. */
  seed: string;
  /** Just written: the lines are drawn in front of the viewer. */
  fresh?: boolean;
}

/** A shot count written by hand, in pencil, and marked against par (see `core/marks.ts`). */
export function ScoreMark({ score, seed, fresh = false }: Props) {
  const marks = marksFor(score, seed);
  const digit = digitOf(score);
  return (
    <span class={`score-mark${fresh ? " fresh" : ""}`} aria-hidden="true">
      {digit !== undefined && <span class="score-digit">{digit}</span>}
      {marks.length > 0 && (
        <svg viewBox={`0 0 ${BOX} ${BOX}`}>
          {marks.map((d, i) => (
            <path key={i} d={d} pathLength={1} />
          ))}
        </svg>
      )}
    </span>
  );
}
