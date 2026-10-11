import type { VerdictKind } from "../core/classify.ts";
import { verdictTone } from "../core/scoring.ts";

/** How a used slot is drawn: by verdict, or plainly (e.g. a best record). */
export type Shot = VerdictKind | "done";

function shotClass(shot: Shot): string {
  return shot === "done" ? "done" : verdictTone(shot);
}

interface Props {
  /** Submissions made so far, in order. */
  shots?: readonly Shot[];
  label: string;
}

/**
 * Submissions as a row of slots in their verdict colors. The goal is always one shot,
 * so an empty slot stands for it before the first submission, and every slot after the
 * first is marked as over.
 */
export function ShotMeter({ shots = [], label }: Props) {
  const slots = Math.max(1, shots.length);
  return (
    <span class="shot-meter" title={label} aria-label={label} role="img">
      {Array.from({ length: slots }, (_, i) => {
        const shot = shots[i];
        const over = i >= 1 ? " over" : "";
        return (
          <span key={i} class={`slot${shot === undefined ? "" : ` ${shotClass(shot)}`}${over}`} />
        );
      })}
    </span>
  );
}

/**
 * The meter with the number of submissions, as a pill. Nothing before the first
 * submission: a lone empty slot says nothing yet.
 */
export function ShotChip(props: Props) {
  const count = props.shots?.length ?? 0;
  if (count === 0) return null;
  return (
    <span class="shot-chip">
      <ShotMeter {...props} />
      <span class="shot-count">{count}</span>
    </span>
  );
}
