// Size metrics of a problem and the per-difficulty limits of the sizing guideline
// (problems/README.md, "規模の基準"). Pure: the IO is in scripts/problem-size.ts.
import { structuredPatch } from "diff";
import type { Difficulty, ProblemKind } from "../src/core/problem.ts";
import { extractCaseIds, type ProblemFiles } from "./parse.ts";

export interface SizeMetrics {
  /** `implement` only: lines the player writes (solution − template + `todo!()` calls). */
  writtenLines?: number;
  /**
   * `repair` and `signature` only: hunks of the template → solution diff. One fix often spans
   * several hunks, so this over-counts; the real number of fixes is counted by hand.
   */
  hunks?: number;
  /** Words of the English statement. */
  statementWords: number;
  /** Test cases in `tests.rs`. */
  cases: number;
  /** `todo!()` calls in the template. */
  todos: number;
}

export interface SizeLimits {
  writtenLines: number;
  hunks: number;
  statementWords: number;
  cases: number;
}

/** Upper limits per difficulty. Exceeding one is a signal to trim or split, not an error. */
export const SIZE_LIMITS: Record<Difficulty, SizeLimits> = {
  1: { writtenLines: 20, hunks: 3, statementWords: 400, cases: 10 },
  2: { writtenLines: 30, hunks: 4, statementWords: 500, cases: 10 },
  3: { writtenLines: 40, hunks: 5, statementWords: 600, cases: 10 },
};

type Files = Pick<ProblemFiles, "statementMd" | "templateRs" | "solutionRs" | "testsRs">;

/** Lines as `wc -l` counts them: a trailing newline does not start another line. */
export function countLines(text: string): number {
  return text === "" ? 0 : text.split("\n").length - (text.endsWith("\n") ? 1 : 0);
}

export function countWords(text: string): number {
  return text.split(/\s+/).filter((w) => w !== "").length;
}

function countTodos(code: string): number {
  return code.match(/\btodo!\(/g)?.length ?? 0;
}

function countHunks(from: string, to: string): number {
  return structuredPatch("template.rs", "solution.rs", from, to, undefined, undefined, {
    context: 0,
  }).hunks.length;
}

export function measure(kind: ProblemKind, files: Files): SizeMetrics {
  const todos = countTodos(files.templateRs);
  const common = {
    statementWords: countWords(files.statementMd),
    cases: extractCaseIds(files.testsRs).length,
    todos,
  };
  return kind === "implement"
    ? {
        ...common,
        writtenLines: Math.max(
          0,
          countLines(files.solutionRs) - countLines(files.templateRs) + todos,
        ),
      }
    : { ...common, hunks: countHunks(files.templateRs, files.solutionRs) };
}

export interface Exceedance {
  metric: keyof SizeLimits;
  value: number;
  limit: number;
}

/** The metrics of `m` that are over `limits`, in the order of the limits table. */
export function exceedances(m: SizeMetrics, limits: SizeLimits): Exceedance[] {
  const metrics: (keyof SizeLimits)[] = ["writtenLines", "hunks", "statementWords", "cases"];
  return metrics.flatMap((metric) => {
    const value = m[metric];
    return value !== undefined && value > limits[metric]
      ? [{ metric, value, limit: limits[metric] }]
      : [];
  });
}
