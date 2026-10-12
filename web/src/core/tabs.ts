/**
 * Which arrow keys move between the options of a tab list or a radio group:
 * a radio group takes both pairs.
 */
export type Orientation = "horizontal" | "vertical" | "both";

const STEPS: Record<Orientation, Record<string, 1 | -1>> = {
  horizontal: { ArrowRight: 1, ArrowLeft: -1 },
  vertical: { ArrowDown: 1, ArrowUp: -1 },
  both: { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 },
};

/**
 * The option a key moves to from `current` among `count`, wrapping around at the ends;
 * `undefined` when the key does not move.
 */
export function keyTarget(
  key: string,
  current: number,
  count: number,
  orientation: Orientation,
): number | undefined {
  if (count === 0) return undefined;
  switch (key) {
    case "Home":
      return 0;
    case "End":
      return count - 1;
    default: {
      const step = STEPS[orientation][key];
      return step === undefined ? undefined : (current + step + count) % count;
    }
  }
}
