// Imperative shell of the play screen: drafts, submissions and the current run.
import { useCallback, useEffect, useRef, useState } from "preact/hooks";
import { classify, type Judgement } from "../core/classify.ts";
import { buildSubmission, precheck } from "../core/harness.ts";
import type { JudgeSources, Problem } from "../core/problem.ts";
import { parseStdout, type RunProgress, runProgress } from "../core/protocol.ts";
import { bestOverall, improvedBest, newRun, recordAttempt, type Run } from "../core/run.ts";
import type { RunnerPool } from "../runner/pool.ts";
import { draftKey, type Storage } from "../storage/db.ts";

export type SubmitPhase =
  | { kind: "idle" }
  /** Submitted before the compiler finished loading. */
  | { kind: "waiting" }
  | { kind: "running"; progress: RunProgress; output: string };

/** What the result panel shows: the latest judgement, or a rejected pre-check. */
export type LastResult =
  /** `scored` is false for a trial run after the run has ended: it is not recorded. */
  | { kind: "judged"; judgement: Judgement; scored: boolean }
  | { kind: "rejected"; forbiddenMacro: string }
  | { kind: "error"; message: string };

export interface PlayState {
  loaded: boolean;
  code: string;
  /** Incremented whenever `code` is replaced programmatically (the editor must reload it). */
  codeRevision: number;
  run: Run | null;
  phase: SubmitPhase;
  last: LastResult | null;
  /**
   * Set when the run has just ended, to report it. `beatBest` is set when it beat an
   * earlier best record (a first solve sets a best, but does not beat one).
   */
  finished: { run: Run; beatBest: boolean } | null;
  /** The model solution has been seen: runs from then on are assisted. */
  revealed: boolean;
}

interface Params {
  problem: Problem;
  judgeSources: JudgeSources;
  storage: Storage;
  pool: Promise<RunnerPool>;
  /** The round being played; runs and drafts of a round are kept apart from practice. */
  roundId?: string;
  onRunFinished?: (run: Run) => void;
}

const DRAFT_SAVE_DELAY_MS = 500;

function randomNonce(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(8));
  return Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
}

function failed(e: unknown): LastResult {
  return { kind: "error", message: e instanceof Error ? e.message : String(e) };
}

export function usePlay({ problem, judgeSources, storage, pool, roundId, onRunFinished }: Params) {
  const draft = draftKey(problem.id, roundId);
  const [state, setState] = useState<PlayState>({
    loaded: false,
    code: problem.template,
    codeRevision: 0,
    run: null,
    phase: { kind: "idle" },
    last: null,
    finished: null,
    revealed: false,
  });
  const saveTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  // Load the draft and resume an unfinished run.
  useEffect(() => {
    let cancelled = false;
    void Promise.all([
      storage.loadDraft(draft),
      storage.runsOf(problem.id),
      storage.isRevealed(problem.id),
    ]).then(([savedCode, runs, revealed]) => {
      if (cancelled) return;
      const playing = runs.find((r) => r.status === "playing" && r.challengeId === roundId) ?? null;
      const lastAttempt = playing?.attempts.at(-1);
      setState((s) => ({
        loaded: true,
        code: savedCode ?? problem.template,
        codeRevision: s.codeRevision + 1,
        run: playing,
        phase: { kind: "idle" },
        last:
          playing !== null && lastAttempt !== undefined
            ? { kind: "judged", judgement: lastAttempt.judgement, scored: true }
            : null,
        finished: null,
        revealed,
      }));
    });
    return () => {
      cancelled = true;
      clearTimeout(saveTimer.current);
    };
  }, [problem, storage, draft, roundId]);

  const setCode = useCallback(
    (code: string) => {
      setState((s) => ({ ...s, code }));
      clearTimeout(saveTimer.current);
      saveTimer.current = setTimeout(
        () => void storage.saveDraft(draft, code),
        DRAFT_SAVE_DELAY_MS,
      );
    },
    [storage, draft],
  );

  /**
   * Judges `code`, showing the progress meanwhile. Rejects code that fails the pre-check
   * (and shows why) by returning `null`. The caller shows a failure.
   */
  const judge = useCallback(
    async (code: string): Promise<Judgement | null> => {
      const forbiddenMacro = precheck(code);
      if (forbiddenMacro !== null) {
        setState((s) => ({ ...s, last: { kind: "rejected", forbiddenMacro } }));
        return null;
      }
      const nonce = randomNonce();
      const submission = buildSubmission(problem, judgeSources, code, nonce);
      let stdout = "";
      const update = () =>
        setState((s) => ({
          ...s,
          finished: null,
          phase: {
            kind: "running",
            output: parseStdout(stdout, nonce).userOutput,
            progress: runProgress(stdout, nonce, problem.caseIds.length),
          },
        }));
      setState((s) => ({ ...s, finished: null, phase: { kind: "waiting" } }));
      const runner = await pool;
      update();
      const out = await runner.run(submission, {
        onStdout: (chunk) => {
          stdout += chunk;
          update();
        },
        onStderr: () => {},
      });
      return classify(out, {
        nonce,
        caseIds: problem.caseIds,
        stdoutLimitBytes: problem.judge.stdoutLimitBytes,
      });
    },
    [pool, problem, judgeSources],
  );

  const submit = useCallback(async () => {
    if (state.phase.kind !== "idle" || !state.loaded) return;
    const code = state.code;
    const run =
      state.run !== null && state.run.status === "playing"
        ? state.run
        : newRun({
            id: crypto.randomUUID(),
            problemId: problem.id,
            now: Date.now(),
            roundId,
            assisted: state.revealed,
          });
    const previousRun = state.run;

    try {
      const judgement = await judge(code);
      if (judgement === null) return;
      const submittedAt = Date.now();
      const next = recordAttempt(run, { submittedAt, code, judgement });
      let beatBest = false;
      if (next !== run) {
        await storage.saveRun(next);
        const previous = bestOverall(await storage.bestsOf(problem.id));
        const newBest = improvedBest(previous, next);
        if (newBest !== undefined) await storage.saveBest(newBest);
        beatBest = previous !== undefined && newBest !== undefined;
      }
      setState((s) => ({
        ...s,
        // An uncounted first submission does not start a run.
        run: next.attempts.length > 0 ? next : previousRun,
        phase: { kind: "idle" },
        last: { kind: "judged", judgement, scored: true },
        finished: next.status !== "playing" ? { run: next, beatBest } : null,
      }));
      if (next.status !== "playing") onRunFinished?.(next);
    } catch (e) {
      setState((s) => ({ ...s, phase: { kind: "idle" }, last: failed(e) }));
    }
  }, [state, judge, problem, storage, roundId, onRunFinished]);

  /**
   * Runs the code without scoring (once the run has ended): to learn from the model
   * solution by trying changes. Nothing is recorded.
   */
  const tryRun = useCallback(async () => {
    if (state.phase.kind !== "idle" || !state.loaded) return;
    if (state.run === null || state.run.status === "playing") return;
    try {
      const judgement = await judge(state.code);
      if (judgement === null) return;
      setState((s) => ({
        ...s,
        phase: { kind: "idle" },
        last: { kind: "judged", judgement, scored: false },
      }));
    } catch (e) {
      setState((s) => ({ ...s, phase: { kind: "idle" }, last: failed(e) }));
    }
  }, [state, judge]);

  /** Practice: starts over with the template (after the run has ended). */
  const restart = useCallback(async () => {
    if (state.run?.status === "playing") {
      await storage.saveRun({ ...state.run, status: "failed", finishedAt: Date.now() });
    }
    clearTimeout(saveTimer.current);
    await storage.deleteDraft(draft);
    setState((s) => ({
      ...s,
      code: problem.template,
      codeRevision: s.codeRevision + 1,
      run: null,
      last: null,
      finished: null,
    }));
  }, [state.run, storage, problem, draft]);

  /**
   * Ends the problem unsolved, even before the first submission, and reports it like a
   * finished run.
   */
  const giveUp = useCallback(async () => {
    const now = Date.now();
    const current =
      state.run?.status === "playing"
        ? state.run
        : newRun({
            id: crypto.randomUUID(),
            problemId: problem.id,
            now,
            roundId,
            assisted: state.revealed,
          });
    const abandoned: Run = { ...current, status: "failed", finishedAt: now };
    await storage.saveRun(abandoned);
    setState((s) => ({
      ...s,
      run: abandoned,
      finished: { run: abandoned, beatBest: false },
    }));
    onRunFinished?.(abandoned);
  }, [state.run, state.revealed, storage, problem, roundId, onRunFinished]);

  /** Marks the model solution as seen (once the run has ended): later runs are assisted. */
  const revealSolution = useCallback(async () => {
    await storage.saveReveal(problem.id, Date.now());
    setState((s) => ({ ...s, revealed: true }));
  }, [storage, problem]);

  const dismissFinished = useCallback(() => setState((s) => ({ ...s, finished: null })), []);

  return { state, setCode, submit, tryRun, restart, giveUp, dismissFinished, revealSolution };
}
