// The result images offered for sharing: their content and the layout of the verdict grid.
import type { Messages } from "./i18n.ts";
import { courseName, type Round, roundResult, solvedCount, totalSubmissions } from "./round.ts";
import { type ShareInput, titleFor, type VerdictTone, verdictTone } from "./scoring.ts";

export const SITE_NAME = "oneshotrust.dev";

export interface ShareShot {
  label: string;
  tone: VerdictTone;
}

export interface ShareCard {
  /** e.g. "Hole in One!", "Unsolved" or "Total 7". */
  headline: string;
  /** Whether the headline is a success (every problem solved). */
  solved: boolean;
  /** The problem title, or the course of a round. */
  title: string;
  /** e.g. "3 tries · Standard" */
  detail: string;
  /** One row of squares per problem. */
  rows: ShareShot[][];
}

function shots(verdicts: ShareInput["verdicts"]): ShareShot[] {
  return verdicts.map((kind) => ({ label: kind, tone: verdictTone(kind) }));
}

/** The card for one run of a problem. */
export function shareCard(
  input: ShareInput & { problemTitle: string },
  messages: Messages,
): ShareCard {
  const n = input.verdicts.length;
  return {
    headline: input.solved
      ? `${titleFor(n) ?? messages.solvedHeadline}!`
      : messages.unsolvedHeadline,
    solved: input.solved,
    title: input.problemTitle,
    detail: [messages.shareTries(n), messages.modes[input.mode]].join(" · "),
    rows: [shots(input.verdicts)],
  };
}

/** The scorecard of a finished round. */
export function roundShareCard(round: Round, messages: Messages): ShareCard {
  return {
    headline: messages.roundTotal(totalSubmissions(round)),
    solved: solvedCount(round) === round.problemIds.length,
    title: messages.courseTitle(courseName(round.course, messages)),
    detail: [roundResult(round, messages), messages.modes[round.mode]].join(" · "),
    rows: round.entries.map((e) => shots(e.verdicts)),
  };
}

export interface ShotGrid {
  /** Side of each square. */
  size: number;
  /** Distance between the left (or top) edges of neighboring squares. */
  step: number;
}

/**
 * Lays out rows of squares (`counts[i]` squares in row `i`) in a box of `width` × `height`:
 * all squares share one size, at most `maxSize`, with gaps of a quarter of it.
 */
export function layoutShots(
  counts: readonly number[],
  width: number,
  height: number,
  maxSize: number,
): ShotGrid {
  // n squares and (n - 1) gaps of size / 4 fill `length` exactly when size = 4l / (5n - 1).
  const fit = (length: number, n: number) => (n === 0 ? maxSize : (4 * length) / (5 * n - 1));
  const size = Math.min(maxSize, fit(width, Math.max(0, ...counts)), fit(height, counts.length));
  return { size, step: size * 1.25 };
}
