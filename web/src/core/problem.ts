import type { Localized } from "./i18n.ts";

export type ProblemKind = "implement" | "repair" | "signature";
export type MiriChecks = "full" | "fast";

/** Coarse difficulty tier. A round climbs from 1 to 3 (see `ROUND_LADDER`). */
export type Difficulty = 1 | 2 | 3;

export const DIFFICULTIES: readonly Difficulty[] = [1, 2, 3];

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
  difficulty: Difficulty;
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

/** The model solution, loaded on demand (see `virtual:oneshot-solutions`). */
export interface ModelSolution {
  code: string;
  /** Optional commentary, rendered from Markdown at build time. */
  editorialHtml?: Localized;
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
export const EXAMPLE_CASE_PREFIX = "example-";

export function isExampleCase(id: string): boolean {
  return id.startsWith(EXAMPLE_CASE_PREFIX);
}
