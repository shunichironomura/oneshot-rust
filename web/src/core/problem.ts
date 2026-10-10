import type { Localized } from "./i18n.ts";

export type ProblemKind = "implement" | "repair" | "signature";
export type MiriChecks = "full" | "fast";

export interface JudgeConfig {
  forbidUnsafe: boolean;
  miriChecks: MiriChecks;
  timeLimitMs: number;
  stdoutLimitBytes: number;
  harnessVisible: boolean;
}

export interface Problem {
  id: string;
  title: Localized;
  par: number;
  concepts: string[];
  kind: ProblemKind;
  /** Problem statement, already rendered from Markdown to HTML at build time. */
  statementHtml: Localized;
  template: string;
  tests: string;
  /** Test case ids in the order they appear in `tests.rs`. */
  caseIds: string[];
  judge: JudgeConfig;
}

/** Sources of the fixed judge harness (`judge/` in the repository). */
export interface JudgeSources {
  main: string;
  lib: string;
}

export interface Content {
  problems: Problem[];
  judge: JudgeSources;
}

/** Test cases whose id starts with this prefix are public. */
export const SAMPLE_CASE_PREFIX = "sample-";

export function isSampleCase(id: string): boolean {
  return id.startsWith(SAMPLE_CASE_PREFIX);
}
