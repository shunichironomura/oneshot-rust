// A "run" is one attempt at solving a problem: submissions until AC (or out of lives).
import { isCounted, type Judgement, type VerdictKind } from "./classify.ts";
import { MODES, type Mode } from "./mode.ts";

export interface Attempt {
  submittedAt: number;
  code: string;
  judgement: Judgement;
}

export type RunStatus = "playing" | "solved" | "failed";

export interface Run {
  id: string;
  problemId: string;
  mode: Mode;
  /** Maximum number of submissions, or `null` for unlimited. */
  lives: number | null;
  startedAt: number;
  attempts: Attempt[];
  status: RunStatus;
  finishedAt?: number;
  /**
   * The round the run is part of; unset for practice. (Stored under the name it had
   * when rounds were called challenges, so that older runs keep their round.)
   */
  challengeId?: string;
  /** Set when the model solution was seen before or during the run: it cannot set a best record. */
  assisted?: boolean;
}

export interface Best {
  problemId: string;
  mode: Mode;
  submissions: number;
  ceCount: number;
  achievedAt: number;
  runId: string;
}

export function newRun(params: {
  id: string;
  problemId: string;
  mode: Mode;
  lives: number | null;
  now: number;
  roundId?: string;
  assisted?: boolean;
}): Run {
  return {
    id: params.id,
    problemId: params.problemId,
    mode: params.mode,
    lives: params.lives,
    startedAt: params.now,
    attempts: [],
    status: "playing",
    challengeId: params.roundId,
    ...(params.assisted === true && { assisted: true }),
  };
}

export function verdictsOf(run: Run): VerdictKind[] {
  return run.attempts.map((a) => a.judgement.verdict.kind);
}

/**
 * The verdicts of the run that set `best`, or `undefined` when that run is not available
 * (e.g. it was dropped by a storage upgrade that kept the best records).
 */
export function bestVerdicts(best: Best, run: Run | undefined): VerdictKind[] | undefined {
  if (run === undefined || run.id !== best.runId) return undefined;
  const verdicts = verdictsOf(run);
  return verdicts.length === best.submissions ? verdicts : undefined;
}

export function ceCount(run: Run): number {
  return verdictsOf(run).filter((k) => k === "CE").length;
}

export function remainingLives(run: Run): number | null {
  return run.lives === null ? null : run.lives - run.attempts.length;
}

/**
 * Records a submission. Uncounted verdicts (internal errors) leave the run unchanged,
 * as do submissions to a finished run.
 */
export function recordAttempt(run: Run, attempt: Attempt): Run {
  if (run.status !== "playing" || !isCounted(attempt.judgement.verdict.kind)) return run;
  const attempts = [...run.attempts, attempt];
  const solved = attempt.judgement.verdict.kind === "AC";
  const outOfLives = run.lives !== null && attempts.length >= run.lives;
  const status: RunStatus = solved ? "solved" : outOfLives ? "failed" : "playing";
  return {
    ...run,
    attempts,
    status,
    finishedAt: status === "playing" ? undefined : attempt.submittedAt,
  };
}

/** Returns the new best record if `run` improves on `prev`. Assisted runs never do. */
export function improvedBest(prev: Best | undefined, run: Run): Best | undefined {
  if (run.status !== "solved" || run.finishedAt === undefined || run.assisted === true) {
    return undefined;
  }
  const candidate: Best = {
    problemId: run.problemId,
    mode: run.mode,
    submissions: run.attempts.length,
    ceCount: ceCount(run),
    achievedAt: run.finishedAt,
    runId: run.id,
  };
  if (prev === undefined) return candidate;
  const better =
    candidate.submissions < prev.submissions ||
    (candidate.submissions === prev.submissions && candidate.ceCount < prev.ceCount);
  return better ? candidate : undefined;
}

/**
 * The best of the per-mode records of a problem: fewest submissions, then the harder
 * mode, then the newer one.
 */
export function bestOverall(bests: readonly Best[]): Best | undefined {
  const rank = (b: Best) => MODES.indexOf(b.mode);
  return bests.reduce<Best | undefined>((best, b) => {
    if (best === undefined) return b;
    const better =
      b.submissions - best.submissions || rank(best) - rank(b) || best.achievedAt - b.achievedAt;
    return better < 0 ? b : best;
  }, undefined);
}

/** The best record of each problem across modes. */
export function bestsByProblem(bests: readonly Best[]): Map<string, Best> {
  return bests.reduce((map, b) => {
    const best = bestOverall([map.get(b.problemId), b].filter((x) => x !== undefined));
    return best === undefined ? map : map.set(b.problemId, best);
  }, new Map<string, Best>());
}
