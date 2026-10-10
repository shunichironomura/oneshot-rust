import {
  type Diagnostic,
  errorsOf,
  isInternalCompilerError,
  isUndefinedBehavior,
  parseStderr,
  warningsOf,
} from "./diagnostics.ts";
import { isSampleCase } from "./problem.ts";
import { parseStdout } from "./protocol.ts";

export type CaseStatus = "pass" | "fail" | "not-run";

export interface CaseResult {
  id: string;
  visible: boolean;
  status: CaseStatus;
  message?: string;
}

export type Verdict =
  | { kind: "CE"; diagnostics: Diagnostic[] }
  | { kind: "AC"; cases: CaseResult[] }
  | { kind: "WA"; cases: CaseResult[] }
  | { kind: "UB"; diagnostics: Diagnostic[]; cases: CaseResult[] }
  | { kind: "RE"; reason: RuntimeErrorReason; diagnostics: Diagnostic[]; cases: CaseResult[] }
  | { kind: "TLE"; cases: CaseResult[]; runningCase?: string }
  | { kind: "IE"; reason: InternalErrorReason };

/** Why a run ended abnormally. Messages are localized by the UI. */
export type RuntimeErrorReason =
  | { type: "diagnostic"; message: string }
  | { type: "panic"; message: string }
  | { type: "exit-code"; code: number }
  | { type: "abnormal-exit" }
  | { type: "output-limit"; limitBytes: number };

export type InternalErrorReason =
  | { type: "ice"; message: string }
  | { type: "crash"; message: string }
  | { type: "compile-timeout" }
  | { type: "no-start"; exitCode: number | null };

export type VerdictKind = Verdict["kind"];

/** Raw result of one Miri invocation, as collected by the runner. */
export interface RunOutput {
  stdout: string;
  stderr: string;
  /** `null` when the process did not exit normally (timeout, crash, output limit). */
  exitCode: number | null;
  timedOut: boolean;
  /** Set when stdout exceeded the problem's limit and the run was aborted. */
  outputLimitExceeded: boolean;
  /** Set when Miri itself crashed (Wasm trap, out of memory, ...). */
  crash?: string;
}

export interface ClassifyContext {
  nonce: string;
  caseIds: string[];
  stdoutLimitBytes: number;
}

export interface Judgement {
  verdict: Verdict;
  /** The player's own stdout. */
  output: string;
  /** Non-diagnostic stderr (the player's `eprintln!`, panics outside of cases). */
  stderr: string;
  warnings: Diagnostic[];
}

/** Whether a verdict counts as a submission. Internal errors are not the player's fault. */
export function isCounted(kind: VerdictKind): boolean {
  return kind !== "IE";
}

function caseResults(
  caseIds: string[],
  outcomes: { id: string; pass: boolean; message?: string }[],
) {
  const byId = new Map(outcomes.map((o) => [o.id, o]));
  return caseIds.map((id): CaseResult => {
    const o = byId.get(id);
    const visible = isSampleCase(id);
    if (o === undefined) return { id, visible, status: "not-run" };
    return o.pass
      ? { id, visible, status: "pass" }
      : { id, visible, status: "fail", message: o.message };
  });
}

function runtimeErrorReason(
  errors: Diagnostic[],
  panic: string | undefined,
  exitCode: number | null,
): RuntimeErrorReason {
  if (errors.length > 0) return { type: "diagnostic", message: errors[0].message };
  if (panic !== undefined) return { type: "panic", message: panic };
  return exitCode === null ? { type: "abnormal-exit" } : { type: "exit-code", code: exitCode };
}

export function classify(out: RunOutput, ctx: ClassifyContext): Judgement {
  const stdout = parseStdout(out.stdout, ctx.nonce);
  const stderr = parseStderr(out.stderr);
  const errors = errorsOf(stderr.diagnostics);
  const base = {
    output: stdout.userOutput,
    stderr: stderr.text,
    warnings: warningsOf(stderr.diagnostics),
  };
  const judged = (verdict: Verdict): Judgement => ({ verdict, ...base });
  const cases = caseResults(ctx.caseIds, stdout.cases);

  const ice = stderr.diagnostics.find(isInternalCompilerError);
  if (ice !== undefined) {
    return judged({ kind: "IE", reason: { type: "ice", message: ice.message } });
  }
  // Note: on WASI, rustc is built with panic=abort, so even an ordinary compile error
  // ends in a trap (`crash`). The output decides the verdict; a crash only matters
  // when nothing else explains how the run ended.
  const crashed: Verdict | undefined =
    out.crash === undefined
      ? undefined
      : { kind: "IE", reason: { type: "crash", message: out.crash } };

  if (!stdout.started) {
    if (errors.length > 0) return judged({ kind: "CE", diagnostics: errors });
    if (out.timedOut) return judged({ kind: "IE", reason: { type: "compile-timeout" } });
    if (crashed !== undefined) return judged(crashed);
    return judged({ kind: "IE", reason: { type: "no-start", exitCode: out.exitCode } });
  }

  if (out.timedOut) {
    const runningCase = cases.find((c) => c.status === "not-run")?.id;
    return judged({ kind: "TLE", cases, runningCase });
  }
  if (out.outputLimitExceeded) {
    return judged({
      kind: "RE",
      reason: { type: "output-limit", limitBytes: ctx.stdoutLimitBytes },
      diagnostics: [],
      cases,
    });
  }

  // `END` without a clean exit happens when Miri reports e.g. a memory leak after `main`.
  if (!stdout.ended || out.exitCode !== 0) {
    if (errors.some(isUndefinedBehavior)) return judged({ kind: "UB", diagnostics: errors, cases });
    const panic = stderr.text.split("\n").find((l) => l.startsWith("panicked: "));
    if (crashed !== undefined && errors.length === 0 && panic === undefined) return judged(crashed);
    return judged({
      kind: "RE",
      reason: runtimeErrorReason(errors, panic, out.exitCode),
      diagnostics: errors,
      cases,
    });
  }

  return judged(
    cases.every((c) => c.status === "pass") ? { kind: "AC", cases } : { kind: "WA", cases },
  );
}
