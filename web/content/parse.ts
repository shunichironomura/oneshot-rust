import { marked } from "marked";
import { parse as parseToml } from "smol-toml";
import type { JudgeConfig, MiriChecks, Problem, ProblemKind } from "../src/core/problem.ts";

/** Raw files of one problem directory (`problems/<id>/`). */
export interface ProblemFiles {
  dirName: string;
  problemToml: string;
  statementMd: string;
  templateRs: string;
  testsRs: string;
  solutionRs: string;
}

/** A problem plus the data that is used for verification only and never shipped. */
export interface ProblemWithSolution {
  problem: Problem;
  solution: string;
}

export class ProblemFormatError extends Error {
  constructor(dirName: string, message: string) {
    super(`problems/${dirName}: ${message}`);
    this.name = "ProblemFormatError";
  }
}

const KINDS: readonly ProblemKind[] = ["implement", "repair", "signature"];
const MIRI_CHECKS: readonly MiriChecks[] = ["full", "fast"];

const JUDGE_DEFAULTS: JudgeConfig = {
  forbidUnsafe: true,
  miriChecks: "full",
  timeLimitMs: 15000,
  stdoutLimitBytes: 65536,
  harnessVisible: false,
};

type Table = Record<string, unknown>;

function isTable(value: unknown): value is Table {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

class FieldReader {
  private readonly dirName: string;
  private readonly table: Table;
  private readonly prefix: string;

  constructor(dirName: string, table: Table, prefix: string) {
    this.dirName = dirName;
    this.table = table;
    this.prefix = prefix;
  }

  private fail(key: string, expected: string): never {
    throw new ProblemFormatError(this.dirName, `\`${this.prefix}${key}\` must be ${expected}`);
  }

  string(key: string): string {
    const v = this.table[key];
    return typeof v === "string" && v.length > 0 ? v : this.fail(key, "a non-empty string");
  }

  int(key: string, min: number, max: number, fallback?: number): number {
    const v = this.table[key] ?? fallback;
    return typeof v === "number" && Number.isInteger(v) && v >= min && v <= max
      ? v
      : this.fail(key, `an integer in ${min}..=${max}`);
  }

  bool(key: string, fallback: boolean): boolean {
    const v = this.table[key] ?? fallback;
    return typeof v === "boolean" ? v : this.fail(key, "a boolean");
  }

  stringArray(key: string): string[] {
    const v = this.table[key];
    return Array.isArray(v) && v.every((x) => typeof x === "string")
      ? v
      : this.fail(key, "an array of strings");
  }

  oneOf<T extends string>(key: string, options: readonly T[], fallback?: T): T {
    const v = this.table[key] ?? fallback;
    return options.find((o) => o === v) ?? this.fail(key, `one of ${options.join(", ")}`);
  }
}

/** Extracts test case ids from `case("<id>", ...)` calls in `tests.rs`. */
export function extractCaseIds(testsRs: string): string[] {
  return [...testsRs.matchAll(/\bcase\(\s*"([^"]+)"/g)].map((m) => m[1]);
}

function parseJudge(dirName: string, value: unknown): JudgeConfig {
  if (value === undefined) return JUDGE_DEFAULTS;
  if (!isTable(value)) throw new ProblemFormatError(dirName, "`[judge]` must be a table");
  const r = new FieldReader(dirName, value, "judge.");
  return {
    forbidUnsafe: r.bool("forbid_unsafe", JUDGE_DEFAULTS.forbidUnsafe),
    miriChecks: r.oneOf("miri_checks", MIRI_CHECKS, JUDGE_DEFAULTS.miriChecks),
    timeLimitMs: r.int("time_limit_ms", 1000, 600_000, JUDGE_DEFAULTS.timeLimitMs),
    stdoutLimitBytes: r.int("stdout_limit_bytes", 1024, 16 << 20, JUDGE_DEFAULTS.stdoutLimitBytes),
    harnessVisible: r.bool("harness_visible", JUDGE_DEFAULTS.harnessVisible),
  };
}

export function parseProblem(files: ProblemFiles): ProblemWithSolution {
  const { dirName } = files;
  let toml: Table;
  try {
    toml = parseToml(files.problemToml);
  } catch (e) {
    throw new ProblemFormatError(dirName, `invalid problem.toml: ${String(e)}`);
  }
  const r = new FieldReader(dirName, toml, "");
  const id = r.string("id");
  if (id !== dirName) {
    throw new ProblemFormatError(dirName, `id "${id}" must match the directory name`);
  }
  const caseIds = extractCaseIds(files.testsRs);
  if (caseIds.length === 0) {
    throw new ProblemFormatError(dirName, "tests.rs must contain at least one case(...)");
  }
  const duplicate = caseIds.find((c, i) => caseIds.indexOf(c) !== i);
  if (duplicate !== undefined) {
    throw new ProblemFormatError(dirName, `duplicate case id "${duplicate}"`);
  }
  const problem: Problem = {
    id,
    title: r.string("title"),
    difficulty: r.int("difficulty", 1, 5),
    par: r.int("par", 1, 20),
    concepts: r.stringArray("concepts"),
    kind: r.oneOf("kind", KINDS),
    statementHtml: marked.parse(files.statementMd, { async: false }),
    template: files.templateRs,
    tests: files.testsRs,
    caseIds,
    judge: parseJudge(dirName, toml["judge"]),
  };
  if (problem.kind === "signature" && !problem.judge.harnessVisible) {
    throw new ProblemFormatError(dirName, "signature problems require harness_visible = true");
  }
  return { problem, solution: files.solutionRs };
}

/** Problems are ordered by difficulty, then by id. */
export function compareProblems(a: Problem, b: Problem): number {
  return a.difficulty - b.difficulty || a.id.localeCompare(b.id);
}
