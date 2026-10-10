// 連続チャレンジ: play several problems in a row and compete on the total number of submissions.
import type { VerdictKind } from "./classify.ts";
import type { Messages } from "./i18n.ts";
import type { Mode } from "./mode.ts";
import { type Run, verdictsOf } from "./run.ts";
import { verdictEmoji } from "./scoring.ts";

export const CHALLENGE_LENGTH = 5;

export interface ChallengeEntry {
  problemId: string;
  runId: string;
  solved: boolean;
  verdicts: VerdictKind[];
}

export interface Challenge {
  id: string;
  mode: Mode;
  lives: number | null;
  problemIds: string[];
  startedAt: number;
  entries: ChallengeEntry[];
  finishedAt?: number;
}

/** Picks `count` distinct problems uniformly at random (Fisher–Yates with the given RNG). */
export function pickProblems(
  ids: readonly string[],
  count: number,
  random: () => number,
): string[] {
  const pool = [...ids];
  for (let i = pool.length - 1; i > 0; i--) {
    const j = Math.floor(random() * (i + 1));
    [pool[i], pool[j]] = [pool[j], pool[i]];
  }
  return pool.slice(0, count);
}

export function newChallenge(params: {
  id: string;
  mode: Mode;
  lives: number | null;
  problemIds: string[];
  now: number;
}): Challenge {
  return { ...params, startedAt: params.now, entries: [] };
}

/** The problem currently being played, or `undefined` when the challenge is over. */
export function currentProblemId(challenge: Challenge): string | undefined {
  return challenge.problemIds[challenge.entries.length];
}

/** Records a finished run of the current problem. Other runs leave the challenge unchanged. */
export function recordRun(challenge: Challenge, run: Run): Challenge {
  if (run.status === "playing" || run.problemId !== currentProblemId(challenge)) return challenge;
  if (challenge.entries.some((e) => e.runId === run.id)) return challenge;
  const entries = [
    ...challenge.entries,
    {
      problemId: run.problemId,
      runId: run.id,
      solved: run.status === "solved",
      verdicts: verdictsOf(run),
    },
  ];
  const done = entries.length === challenge.problemIds.length;
  return { ...challenge, entries, finishedAt: done ? run.finishedAt : undefined };
}

export function totalSubmissions(challenge: Challenge): number {
  return challenge.entries.reduce((n, e) => n + e.verdicts.length, 0);
}

export function solvedCount(challenge: Challenge): number {
  return challenge.entries.filter((e) => e.solved).length;
}

/** Wordle-style result text: one row of squares per problem. */
export function challengeShareText(challenge: Challenge, messages: Messages): string {
  const rows = challenge.entries.map(
    (e) => `${e.verdicts.map(verdictEmoji).join("") || "—"}${e.solved ? "" : " ✗"}  ${e.problemId}`,
  );
  return [
    messages.challengeShareHeader(messages.modes[challenge.mode]),
    ...rows,
    messages.challengeTotal(
      totalSubmissions(challenge),
      solvedCount(challenge),
      challenge.problemIds.length,
    ),
  ].join("\n");
}
