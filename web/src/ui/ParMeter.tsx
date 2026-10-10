import type { VerdictKind } from "../core/classify.ts";

/** How a used slot is drawn: by verdict, or plainly (e.g. a best record). */
export type Shot = VerdictKind | "done";

function shotClass(shot: Shot): string {
  switch (shot) {
    case "AC":
      return "ac";
    case "CE":
      return "ce";
    case "done":
      return "done";
    case "IE":
      return "ie";
    case "WA":
    case "RE":
    case "UB":
    case "TLE":
      return "miss";
  }
}

interface Props {
  par: number;
  /** Submissions made so far, in order. */
  shots?: readonly Shot[];
  label: string;
}

/**
 * Par as a row of slots, one per expected submission. Each submission fills a slot
 * in its verdict color; submissions beyond par add slots marked as over par.
 */
export function ParMeter({ par, shots = [], label }: Props) {
  const slots = Math.max(par, shots.length);
  return (
    <span class="par-meter" title={label} aria-label={label} role="img">
      {Array.from({ length: slots }, (_, i) => {
        const shot = shots[i];
        const over = i >= par ? " over" : "";
        return (
          <span key={i} class={`slot${shot === undefined ? "" : ` ${shotClass(shot)}`}${over}`} />
        );
      })}
    </span>
  );
}

/** The meter with a "Par" label, as a pill. */
export function ParChip(props: Props) {
  return (
    <span class="par-chip">
      <span class="par-label">Par</span>
      <ParMeter {...props} />
    </span>
  );
}
