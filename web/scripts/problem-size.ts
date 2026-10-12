// Measures the size of every problem against the limits of the sizing guideline
// (problems/README.md, "規模の基準") and lists the ones over a limit. Advisory only:
// the exit code is always 0.
//
// Usage: node scripts/problem-size.ts [problem-id ...]
import { parseArgs } from "node:util";
import { loadProblemFiles } from "../content/load.ts";
import { parseProblem } from "../content/parse.ts";
import { exceedances, measure, SIZE_LIMITS, type SizeMetrics } from "../content/size.ts";

const LABELS = {
  writtenLines: "written lines",
  hunks: "hunks",
  statementWords: "statement words",
  cases: "cases",
} as const;

function cell(value: number | undefined): string {
  return (value === undefined ? "-" : String(value)).padStart(5);
}

function row(id: string, difficulty: number, kind: string, m: SizeMetrics, over: string): string {
  return [
    String(difficulty),
    id.padEnd(14),
    kind.padEnd(9),
    cell(m.writtenLines),
    cell(m.hunks),
    cell(m.todos),
    cell(m.statementWords),
    cell(m.cases),
    over,
  ].join(" ");
}

async function main() {
  const { positionals } = parseArgs({ allowPositionals: true });
  const all = await loadProblemFiles();
  const selected =
    positionals.length === 0 ? all : all.filter((f) => positionals.includes(f.dirName));
  const measured = selected
    .map((files) => {
      const { problem } = parseProblem(files);
      const metrics = measure(problem.kind, files);
      const over = exceedances(metrics, SIZE_LIMITS[problem.difficulty]);
      return { problem, metrics, over };
    })
    .sort(
      (a, b) =>
        a.problem.difficulty - b.problem.difficulty || a.problem.id.localeCompare(b.problem.id),
    );

  const header = ["d", "problem".padEnd(14), "kind".padEnd(9)]
    .concat(["lines", "hunks", "todos", "words", "cases"].map((h) => h.padStart(5)))
    .concat(["over"])
    .join(" ");
  console.log(header);
  for (const { problem, metrics, over } of measured) {
    const flags = over.map((e) => `${LABELS[e.metric]} ${e.value} > ${e.limit}`).join(", ");
    console.log(row(problem.id, problem.difficulty, problem.kind, metrics, flags));
  }
  const flagged = measured.filter((p) => p.over.length > 0);
  console.log(
    `\n${flagged.length} of ${measured.length} problems over a limit (lines: implement only; hunks: repair and signature, a reference value)`,
  );
}

await main();
