// Build-time IO: reads problems and the judge harness from the repository.
import { readdir, readFile } from "node:fs/promises";
import path from "node:path";
import type { JudgeSources } from "../src/core/problem.ts";
import {
  compareProblems,
  parseProblem,
  type ProblemFiles,
  type ProblemWithSolution,
} from "./parse.ts";

export const REPO_ROOT = path.resolve(import.meta.dirname, "../..");
export const PROBLEMS_DIR = path.join(REPO_ROOT, "problems");
export const JUDGE_DIR = path.join(REPO_ROOT, "judge");

/** Reads the raw files of `problems/<dirName>/` without parsing them. */
export async function readProblemFiles(dirName: string): Promise<ProblemFiles> {
  const read = (file: string) => readFile(path.join(PROBLEMS_DIR, dirName, file), "utf8");
  const readOptional = (file: string) =>
    read(file).catch((e: unknown) => {
      if (e instanceof Error && "code" in e && e.code === "ENOENT") return undefined;
      throw e;
    });
  const [
    problemToml,
    statementMd,
    statementJaMd,
    templateRs,
    testsRs,
    solutionRs,
    editorialMd,
    editorialJaMd,
  ] = await Promise.all([
    read("problem.toml"),
    read("statement.md"),
    readOptional("statement.ja.md"),
    read("template.rs"),
    read("tests.rs"),
    read("solution.rs"),
    readOptional("editorial.md"),
    readOptional("editorial.ja.md"),
  ]);
  return {
    dirName,
    problemToml,
    statementMd,
    statementJaMd,
    templateRs,
    testsRs,
    solutionRs,
    editorialMd,
    editorialJaMd,
  };
}

export async function readProblemDir(dirName: string): Promise<ProblemWithSolution> {
  return parseProblem(await readProblemFiles(dirName));
}

async function problemDirNames(): Promise<string[]> {
  const entries = await readdir(PROBLEMS_DIR, { withFileTypes: true });
  return entries.filter((e) => e.isDirectory()).map((e) => e.name);
}

/** The raw files of every problem, in directory order. */
export async function loadProblemFiles(): Promise<ProblemFiles[]> {
  return Promise.all((await problemDirNames()).map(readProblemFiles));
}

export async function loadProblems(): Promise<ProblemWithSolution[]> {
  const problems = await Promise.all((await problemDirNames()).map(readProblemDir));
  return problems.sort((a, b) => compareProblems(a.problem, b.problem));
}

export async function loadJudgeSources(): Promise<JudgeSources> {
  const [main, lib] = await Promise.all([
    readFile(path.join(JUDGE_DIR, "main.rs"), "utf8"),
    readFile(path.join(JUDGE_DIR, "judge.rs"), "utf8"),
  ]);
  return { main, lib };
}
