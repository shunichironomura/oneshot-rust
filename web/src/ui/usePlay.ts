// Imperative shell of the play screen: drafts, submissions and the current run.
import { useCallback, useEffect, useRef, useState } from "preact/hooks";
import { classify, type Judgement } from "../core/classify.ts";
import { buildSubmission, precheck } from "../core/harness.ts";
import type { Mode } from "../core/mode.ts";
import type { JudgeSources, Problem } from "../core/problem.ts";
import { parseStdout, type RunProgress, runProgress } from "../core/protocol.ts";
import { type Best, improvedBest, newRun, recordAttempt, type Run } from "../core/run.ts";
import type { Settings } from "../core/settings.ts";
import type { RunnerPool } from "../runner/pool.ts";
import type { Storage } from "../storage/db.ts";

export type SubmitPhase =
  | { kind: "idle" }
  | { kind: "running"; progress: RunProgress; output: string };

/** What the result panel shows: the latest judgement, or a rejected pre-check. */
export type LastResult =
  | { kind: "judged"; judgement: Judgement; mode: Mode }
  | { kind: "rejected"; message: string }
  | { kind: "error"; message: string };

export interface PlayState {
  loaded: boolean;
  code: string;
  /** Incremented whenever `code` is replaced programmatically (the editor must reload it). */
  codeRevision: number;
  run: Run | null;
  phase: SubmitPhase;
  last: LastResult | null;
  /** Set when the latest submission finished the run (solved or out of lives). */
  finished: { run: Run; newBest: Best | undefined } | null;
}

interface Params {
  problem: Problem;
  judgeSources: JudgeSources;
  storage: Storage;
  settings: Settings;
  pool: RunnerPool | undefined;
}

const DRAFT_SAVE_DELAY_MS = 500;

function randomNonce(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(8));
  return Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
}

function errorMessage(e: unknown): string {
  return e instanceof Error ? e.message : String(e);
}

export function usePlay({ problem, judgeSources, storage, settings, pool }: Params) {
  const [state, setState] = useState<PlayState>({
    loaded: false,
    code: problem.template,
    codeRevision: 0,
    run: null,
    phase: { kind: "idle" },
    last: null,
    finished: null,
  });
  const saveTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  // Load the draft and resume an unfinished run.
  useEffect(() => {
    let cancelled = false;
    void Promise.all([storage.loadDraft(problem.id), storage.runsOf(problem.id)]).then(
      ([draft, runs]) => {
        if (cancelled) return;
        const playing = runs.find((r) => r.status === "playing") ?? null;
        const lastAttempt = playing?.attempts.at(-1);
        setState((s) => ({
          loaded: true,
          code: draft ?? problem.template,
          codeRevision: s.codeRevision + 1,
          run: playing,
          phase: { kind: "idle" },
          last:
            playing !== null && lastAttempt !== undefined
              ? { kind: "judged", judgement: lastAttempt.judgement, mode: playing.mode }
              : null,
          finished: null,
        }));
      },
    );
    return () => {
      cancelled = true;
      clearTimeout(saveTimer.current);
    };
  }, [problem, storage]);

  const setCode = useCallback(
    (code: string) => {
      setState((s) => ({ ...s, code }));
      clearTimeout(saveTimer.current);
      saveTimer.current = setTimeout(
        () => void storage.saveDraft(problem.id, code),
        DRAFT_SAVE_DELAY_MS,
      );
    },
    [problem, storage],
  );

  const submit = useCallback(async () => {
    if (pool === undefined || state.phase.kind === "running" || !state.loaded) return;
    const code = state.code;
    const rejected = precheck(code);
    if (rejected !== null) {
      setState((s) => ({ ...s, last: { kind: "rejected", message: rejected } }));
      return;
    }

    const run =
      state.run !== null && state.run.status === "playing"
        ? state.run
        : newRun({
            id: crypto.randomUUID(),
            problemId: problem.id,
            mode: settings.mode,
            lives: settings.lives,
            now: Date.now(),
          });
    const previousRun = state.run;
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
    update();

    try {
      const out = await pool.run(submission, {
        onStdout: (chunk) => {
          stdout += chunk;
          update();
        },
        onStderr: () => {},
      });
      const judgement = classify(out, {
        nonce,
        caseIds: problem.caseIds,
        stdoutLimitBytes: problem.judge.stdoutLimitBytes,
      });
      const submittedAt = Date.now();
      const next = recordAttempt(run, { submittedAt, code, judgement });
      let newBest: Best | undefined;
      if (next !== run) {
        await storage.saveRun(next);
        newBest = improvedBest(await storage.bestOf(problem.id, next.mode), next);
        if (newBest !== undefined) await storage.saveBest(newBest);
      }
      setState((s) => ({
        ...s,
        // An uncounted first submission does not start a run.
        run: next.attempts.length > 0 ? next : previousRun,
        phase: { kind: "idle" },
        last: { kind: "judged", judgement, mode: run.mode },
        finished: next.status !== "playing" ? { run: next, newBest } : null,
      }));
    } catch (e) {
      setState((s) => ({
        ...s,
        phase: { kind: "idle" },
        last: { kind: "error", message: errorMessage(e) },
      }));
    }
  }, [pool, state, problem, judgeSources, settings, storage]);

  /** Abandons the current run (if any) and starts over with the template. */
  const restart = useCallback(async () => {
    if (state.run?.status === "playing") {
      await storage.saveRun({ ...state.run, status: "failed", finishedAt: Date.now() });
    }
    clearTimeout(saveTimer.current);
    await storage.deleteDraft(problem.id);
    setState((s) => ({
      ...s,
      code: problem.template,
      codeRevision: s.codeRevision + 1,
      run: null,
      last: null,
      finished: null,
    }));
  }, [state.run, storage, problem]);

  const dismissFinished = useCallback(() => setState((s) => ({ ...s, finished: null })), []);

  return { state, setCode, submit, restart, dismissFinished };
}
