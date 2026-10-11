// The result images offered for sharing: their content.
import type { Messages } from "./i18n.ts";
import type { Score } from "./marks.ts";
import { isComplete, type Round, roundResult, totalSubmissions } from "./round.ts";
import { type ShareInput, titleFor, type VerdictTone, verdictTone } from "./scoring.ts";

export const SITE_NAME = "oneshotrust.dev";

export interface ShareShot {
  label: string;
  tone: VerdictTone;
}

/** One hole of the card. */
export interface ShareRow {
  /** The verdicts, in order. */
  shots: ShareShot[];
  /** The count as it is marked on the card. */
  score: Score;
  /** Decides the wobble of the mark's lines. */
  seed: string;
}

export interface ShareCard {
  /** e.g. "Hole in One!", "Unsolved" or "Total 7". */
  headline: string;
  /** Whether the headline is a success (every problem solved). */
  solved: boolean;
  /** The problem title, or when the round was played. */
  title: string;
  /** e.g. "3 tries" or "+2 over perfect" */
  detail: string;
  /** One per problem. */
  rows: ShareRow[];
}

function row(verdicts: ShareInput["verdicts"], solved: boolean, seed: string): ShareRow {
  return {
    shots: verdicts.map((kind) => ({ label: kind, tone: verdictTone(kind) })),
    score: solved ? { kind: "solved", shots: verdicts.length } : { kind: "unsolved" },
    seed,
  };
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
    detail: messages.shareTries(n),
    rows: [row(input.verdicts, input.solved, input.problemId)],
  };
}

/**
 * The scorecard of a finished round, with the date it was played (already formatted). A
 * round with a problem not solved has no total (see `totalScore`): the headline is how far
 * it got instead.
 */
export function roundShareCard(round: Round, date: string, messages: Messages): ShareCard {
  const complete = isComplete(round);
  const result = roundResult(round, messages);
  return {
    headline: complete ? messages.roundTotal(totalSubmissions(round)) : result,
    solved: complete,
    title: date,
    detail: complete ? result : "",
    rows: round.entries.map((e, i) => row(e.verdicts, e.solved, `${round.id}:${i}`)),
  };
}
