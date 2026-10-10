// The result image offered for sharing: its content and the layout of the verdict row.
import type { Messages } from "./i18n.ts";
import { type ShareInput, titleFor, type VerdictTone, verdictTone } from "./scoring.ts";

export const SITE_NAME = "oneshotrust.dev";

export interface ShareCard {
  /** e.g. "Birdie!", or the out-of-lives message. */
  headline: string;
  solved: boolean;
  problemTitle: string;
  /** e.g. "3 tries · Par 2 · Standard" */
  detail: string;
  shots: { label: string; tone: VerdictTone }[];
}

export function shareCard(
  input: ShareInput & { problemTitle: string },
  messages: Messages,
): ShareCard {
  const n = input.verdicts.length;
  return {
    headline: input.solved ? `${titleFor(n, input.par)}!` : messages.outOfLives,
    solved: input.solved,
    problemTitle: input.problemTitle,
    detail: [messages.shareTries(n), `Par ${input.par}`, messages.modes[input.mode]].join(" · "),
    shots: input.verdicts.map((kind) => ({ label: kind, tone: verdictTone(kind) })),
  };
}

export interface ShotRow {
  /** Side of each square. */
  size: number;
  /** Left edge of each square, relative to the start of the row. */
  xs: number[];
}

/**
 * Lays out `count` squares in a row of at most `width`: squares are `maxSize` wide with
 * gaps of a quarter of their size, and shrink to fit when there are many.
 */
export function layoutShots(count: number, width: number, maxSize: number): ShotRow {
  // n squares and (n - 1) gaps of size / 4 fill `width` exactly when size = 4w / (5n - 1).
  const size = count === 0 ? maxSize : Math.min(maxSize, (4 * width) / (5 * count - 1));
  const step = size * 1.25;
  return { size, xs: Array.from({ length: count }, (_, i) => i * step) };
}
