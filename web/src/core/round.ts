// A round: one problem of each difficulty, from easy to hard, scored by the total number of
// submissions. Every round has the same make-up, so scores can be compared.
import type { VerdictKind } from "./classify.ts";
import type { Messages } from "./i18n.ts";
import type { Score } from "./marks.ts";
import { type Mode, PLAY_MODE } from "./mode.ts";
import { DIFFICULTIES, type Difficulty, type Problem } from "./problem.ts";
import { type Run, verdictsOf } from "./run.ts";
import { verdictEmoji } from "./scoring.ts";

/** The difficulties of the problems of a round, in order. */
export const ROUND_LADDER: readonly Difficulty[] = [1, 2, 3];

export const ROUND_LENGTH = ROUND_LADDER.length;

export interface RoundEntry {
  problemId: string;
  runId: string;
  solved: boolean;
  verdicts: VerdictKind[];
}

export interface Round {
  id: string;
  /**
   * The difficulty all the problems were drawn from, in rounds of older versions (players
   * chose a course then). Rounds now climb the ladder instead.
   */
  course?: Difficulty;
  /** Always `PLAY_MODE` now; rounds of older versions may have another mode. */
  mode: Mode;
  problemIds: string[];
  startedAt: number;
  entries: RoundEntry[];
  finishedAt?: number;
  /** Set when the round was abandoned before all of its problems were played. */
  abandoned?: true;
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

/** What the player has done so far, which decides the problems of the next round. */
export interface PlayedProblems {
  /** Problems with a best record. */
  solved: ReadonlySet<string>;
  /** Problems shown in earlier rounds. */
  seen: ReadonlySet<string>;
}

type Candidate = Pick<Problem, "id" | "difficulty">;

/**
 * Orders problems for picking: those not seen yet first, then those seen but not solved
 * (the player may just have read their model solution), then solved ones, each group in
 * random order.
 */
function rankByStaleness<C extends Candidate>(
  candidates: readonly C[],
  played: PlayedProblems,
  random: () => number,
): C[] {
  const staleness = (p: C) => (played.solved.has(p.id) ? 2 : played.seen.has(p.id) ? 1 : 0);
  // Sorting is stable, so the shuffled order remains within each group.
  return shuffle(candidates, random).sort((a, b) => staleness(a) - staleness(b));
}

/**
 * Picks the problems of a new round, one per step of the ladder, ordered from easy to hard.
 * Within a difficulty, the stalest problems come last (see `rankByStaleness`). A difficulty
 * without enough problems is made up for from the others.
 */
export function pickRound(
  problems: readonly Candidate[],
  played: PlayedProblems,
  random: () => number,
): string[] {
  const ranked = (candidates: readonly Candidate[]) => rankByStaleness(candidates, played, random);
  const steps = (d: Difficulty) => ROUND_LADDER.filter((step) => step === d).length;
  const picked = DIFFICULTIES.flatMap((d) =>
    ranked(problems.filter((p) => p.difficulty === d)).slice(0, steps(d)),
  );
  const pickedIds = new Set(picked.map((p) => p.id));
  const makeUp = ranked(problems.filter((p) => !pickedIds.has(p.id))).slice(
    0,
    ROUND_LENGTH - picked.length,
  );
  return [...picked, ...makeUp].sort((a, b) => a.difficulty - b.difficulty).map((p) => p.id);
}

/**
 * Picks one problem of the given difficulty to play on its own, the stalest last as for
 * rounds, or `undefined` when there is no problem of that difficulty.
 */
export function pickProblem(
  problems: readonly Candidate[],
  played: PlayedProblems,
  difficulty: Difficulty,
  random: () => number,
): string | undefined {
  return rankByStaleness(
    problems.filter((p) => p.difficulty === difficulty),
    played,
    random,
  )[0]?.id;
}

export function newRound(params: { id: string; problemIds: string[]; now: number }): Round {
  const { now, ...rest } = params;
  return { ...rest, mode: PLAY_MODE, startedAt: now, entries: [] };
}

/** The round in progress, if any (rounds are given newest first). */
export function activeRound(rounds: readonly Round[]): Round | undefined {
  return rounds.find((r) => r.finishedAt === undefined);
}

/** The problem currently being played, or `undefined` when the round is over. */
export function currentProblemId(round: Round): string | undefined {
  return round.problemIds[round.entries.length];
}

/** e.g. "2 / 3" for the problem at `index` (0-based). */
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

/** Whether every problem of the round was solved: only then does it have a total. */
export function isComplete(round: Round): boolean {
  return solvedCount(round) === round.problemIds.length;
}

/**
 * The total as it is marked on the card: the number of submissions against par (one per
 * problem) when every problem was solved. A problem given up has no count, so a round with
 * one has no total either, only a cross: as in golf, a hole not finished means no score for
 * the round. Summing the submissions made before giving up would reward giving up early.
 */
export function totalScore(round: Round): Score {
  return isComplete(round)
    ? { kind: "solved", shots: totalSubmissions(round), par: round.problemIds.length }
    : { kind: "unsolved" };
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
  return isComplete(round)
    ? messages.roundOverPerfect(totalSubmissions(round) - count)
    : messages.roundSolved(solved, count);
}

/** e.g. "Total 7 · +2 over perfect", or just "2/5 solved" when there is no total. */
export function roundSummary(round: Round, messages: Messages): string {
  const result = roundResult(round, messages);
  return isComplete(round)
    ? [messages.roundTotal(totalSubmissions(round)), result].join(" · ")
    : result;
}

/** Wordle-style result text: one row of squares per problem. Problems are not named. */
export function roundShareText(round: Round, messages: Messages): string {
  const rows = round.entries.map(
    (e) => `${e.verdicts.map(verdictEmoji).join("") || "—"}${e.solved ? "" : " ✗"}`,
  );
  return ["OneShot Rust 🦀", ...rows, roundSummary(round, messages)].join("\n");
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
