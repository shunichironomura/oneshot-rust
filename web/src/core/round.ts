// A round: a few problems from one course in a row, scored by the total number of submissions.
import type { VerdictKind } from "./classify.ts";
import type { Messages } from "./i18n.ts";
import type { Mode } from "./mode.ts";
import type { Difficulty, Problem } from "./problem.ts";
import { type Run, verdictsOf } from "./run.ts";
import { verdictEmoji } from "./scoring.ts";

export const ROUND_LENGTH = 5;

export interface RoundEntry {
  problemId: string;
  runId: string;
  solved: boolean;
  verdicts: VerdictKind[];
}

export interface Round {
  id: string;
  /** The course the problems were drawn from. Missing for rounds from before courses existed. */
  course?: Difficulty;
  mode: Mode;
  problemIds: string[];
  startedAt: number;
  entries: RoundEntry[];
  finishedAt?: number;
  /** Set when the round was abandoned before all of its problems were played. */
  abandoned?: true;
}

/** The problems of a course, in the given order. */
export function courseProblemIds(
  problems: readonly Pick<Problem, "id" | "difficulty">[],
  course: Difficulty,
): string[] {
  return problems.filter((p) => p.difficulty === course).map((p) => p.id);
}

/** Fisher–Yates shuffle with the given RNG. */
function shuffle<T>(items: readonly T[], random: () => number): T[] {
  const pool = [...items];
  for (let i = pool.length - 1; i > 0; i--) {
    const j = Math.floor(random() * (i + 1));
    [pool[i], pool[j]] = [pool[j], pool[i]];
  }
  return pool;
}

/**
 * Picks up to `count` distinct problems at random, unsolved ones first, so that the same
 * course can be played again and again.
 */
export function pickProblems(
  ids: readonly string[],
  solved: ReadonlySet<string>,
  count: number,
  random: () => number,
): string[] {
  const unsolved = shuffle(
    ids.filter((id) => !solved.has(id)),
    random,
  );
  const rest = shuffle(
    ids.filter((id) => solved.has(id)),
    random,
  );
  return [...unsolved, ...rest].slice(0, count);
}

export function newRound(params: {
  id: string;
  course: Difficulty;
  mode: Mode;
  problemIds: string[];
  now: number;
}): Round {
  const { now, ...rest } = params;
  return { ...rest, startedAt: now, entries: [] };
}

/** The round in progress, if any (rounds are given newest first). */
export function activeRound(rounds: readonly Round[]): Round | undefined {
  return rounds.find((r) => r.finishedAt === undefined);
}

/** The problem currently being played, or `undefined` when the round is over. */
export function currentProblemId(round: Round): string | undefined {
  return round.problemIds[round.entries.length];
}

/** e.g. "3 / 5" for the problem at `index` (0-based). */
export function roundPosition(round: Round, index: number): string {
  return `${index + 1} / ${round.problemIds.length}`;
}

function addEntry(round: Round, entry: RoundEntry, now: number): Round {
  const entries = [...round.entries, entry];
  const done = entries.length === round.problemIds.length;
  return { ...round, entries, finishedAt: done ? now : undefined };
}

/** Records a finished run of the current problem. Other runs leave the round unchanged. */
export function recordRun(round: Round, run: Run): Round {
  if (run.status === "playing" || run.problemId !== currentProblemId(round)) return round;
  if (round.entries.some((e) => e.runId === run.id)) return round;
  return addEntry(
    round,
    {
      problemId: run.problemId,
      runId: run.id,
      solved: run.status === "solved",
      verdicts: verdictsOf(run),
    },
    run.finishedAt ?? run.startedAt,
  );
}

/** Records the current problem as unsolved without a run (it no longer exists). */
export function skipCurrent(round: Round, now: number): Round {
  const problemId = currentProblemId(round);
  if (problemId === undefined) return round;
  return addEntry(round, { problemId, runId: "", solved: false, verdicts: [] }, now);
}

/**
 * Ends the round early. The run of the current problem (ended unsolved by the caller), if
 * it had submissions, is recorded; the problems not played are left without entries.
 */
export function abandonRound(round: Round, run: Run | undefined, now: number): Round {
  if (round.finishedAt !== undefined) return round;
  const recorded = run === undefined || run.attempts.length === 0 ? round : recordRun(round, run);
  return { ...recorded, finishedAt: recorded.finishedAt ?? now, abandoned: true };
}

export function totalSubmissions(round: Round): number {
  return round.entries.reduce((n, e) => n + e.verdicts.length, 0);
}

export function solvedCount(round: Round): number {
  return round.entries.filter((e) => e.solved).length;
}

/**
 * How the round went: "+2 over perfect" when every problem was solved (a perfect round
 * takes one submission per problem), "2/5 solved" otherwise, and "Abandoned (2/5 solved)"
 * when it was ended early.
 */
export function roundResult(round: Round, messages: Messages): string {
  const solved = solvedCount(round);
  const count = round.problemIds.length;
  if (round.abandoned === true) return messages.roundAbandoned(solved, count);
  return solved === count
    ? messages.roundOverPerfect(totalSubmissions(round) - count)
    : messages.roundSolved(solved, count);
}

/** e.g. "Total 7 · +2 over perfect" */
export function roundSummary(round: Round, messages: Messages): string {
  return [messages.roundTotal(totalSubmissions(round)), roundResult(round, messages)].join(" · ");
}

/** The course name, or the fallback for rounds from before courses existed. */
export function courseName(course: Difficulty | undefined, messages: Messages): string {
  return course === undefined ? messages.courseUnknown : messages.difficulties[course];
}

/** Wordle-style result text: one row of squares per problem. Problems are not named. */
export function roundShareText(round: Round, messages: Messages): string {
  const rows = round.entries.map(
    (e) => `${e.verdicts.map(verdictEmoji).join("") || "—"}${e.solved ? "" : " ✗"}`,
  );
  return [
    messages.roundShareHeader(courseName(round.course, messages), messages.modes[round.mode]),
    ...rows,
    roundSummary(round, messages),
  ].join("\n");
}

/**
 * The problems the player has been shown in rounds: every one with an entry, and the
 * current problem of a round in progress. Problems a round never reached are not included.
 */
export function seenProblemIds(rounds: readonly Round[]): Set<string> {
  return new Set(
    rounds.flatMap((r) => [
      ...r.entries.map((e) => e.problemId),
      ...(r.finishedAt === undefined ? [currentProblemId(r)].filter((id) => id !== undefined) : []),
    ]),
  );
}
