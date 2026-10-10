import type { JudgeConfig, JudgeSources, Problem } from "./problem.ts";

/** Directory layout inside the runner's virtual filesystem. */
export const SRC_DIR = "/src";
export const SYSROOT_DIR = "/sysroot";
/** Writable scratch directory; rustc writes its (unused) output metadata here. */
export const TMP_DIR = "/tmp";
export const INTERPRETED_TARGET = "x86_64-unknown-linux-gnu";

export interface SubmissionFiles {
  "main.rs": string;
  "user.rs": string;
  "judge.rs": string;
  "tests.rs": string;
}

export interface Submission {
  files: SubmissionFiles;
  /** Arguments to Miri, excluding `argv[0]`. */
  args: string[];
  stdoutLimitBytes: number;
  timeLimitMs: number;
}

const USER_ATTRS_PLACEHOLDER = "/*@USER_ATTRS@*/";
const NONCE_PLACEHOLDER = "__NONCE__";

export function renderMain(template: string, judge: JudgeConfig): string {
  const attrs = judge.forbidUnsafe ? "#[forbid(unsafe_code)]" : "";
  return template.replace(USER_ATTRS_PLACEHOLDER, attrs);
}

export function renderJudge(template: string, nonce: string): string {
  if (!/^[0-9a-f]+$/.test(nonce)) throw new Error(`invalid nonce: ${nonce}`);
  return template.replace(NONCE_PLACEHOLDER, nonce);
}

export function miriArgs(judge: JudgeConfig): string[] {
  const fast =
    judge.miriChecks === "fast"
      ? ["-Zmiri-disable-stacked-borrows", "-Zmiri-disable-validation"]
      : [];
  return [
    `${SRC_DIR}/main.rs`,
    "--edition=2024",
    "--crate-type=bin",
    `--sysroot=${SYSROOT_DIR}`,
    `--target=${INTERPRETED_TARGET}`,
    `--out-dir=${TMP_DIR}`,
    "--error-format=json",
    "-Zmiri-seed=0",
    ...fast,
  ];
}

export function buildSubmission(
  problem: Problem,
  judgeSources: JudgeSources,
  userCode: string,
  nonce: string,
): Submission {
  return {
    files: {
      "main.rs": renderMain(judgeSources.main, problem.judge),
      "user.rs": userCode,
      "judge.rs": renderJudge(judgeSources.lib, nonce),
      "tests.rs": problem.tests,
    },
    args: miriArgs(problem.judge),
    stdoutLimitBytes: problem.judge.stdoutLimitBytes,
    timeLimitMs: problem.judge.timeLimitMs,
  };
}

/** Removes comments and string/char literals so that keyword scans do not see them. */
export function stripCommentsAndStrings(code: string): string {
  return code.replace(
    /\/\/[^\n]*|\/\*[\s\S]*?\*\/|r(#*)"[\s\S]*?"\1|b?"(?:\\[\s\S]|[^"\\])*"|b?'(?:\\.|[^'\\\n])'/g,
    " ",
  );
}

const FORBIDDEN_MACROS = ["include_str", "include_bytes", "include", "env", "option_env"];

/**
 * Pre-submission check run before compiling. These macros can read the judge's
 * nonce from the virtual filesystem or the environment, so they are rejected.
 * A rejected submission is not counted.
 */
export function precheck(userCode: string): string | null {
  const code = stripCommentsAndStrings(userCode);
  const found = FORBIDDEN_MACROS.find((m) => new RegExp(`\\b${m}\\s*!`).test(code));
  return found === undefined ? null : `\`${found}!\` は使用できません（判定の改ざん防止のため）`;
}
