// Verifies every problem with the built miri.wasm (same runner code as the browser):
//   1. solution.rs is AC, within half of the time limit.
//   2. template.rs is not AC. It compiles for `implement` problems, and is CE for
//      `repair` and `signature` problems (the harness needs the right signature).
//
// Usage: node scripts/verify-problems.ts [problem-id ...]
import { parseArgs } from "node:util";
import { loadJudgeSources, loadProblems } from "../content/load.ts";
import type { Judgement } from "../src/core/classify.ts";
import { MESSAGES } from "../src/core/i18n.ts";
import { cleanPaths, present } from "../src/core/present.ts";
import type { Problem } from "../src/core/problem.ts";
import { judge, loadToolchain } from "./node-runner.ts";

function describe(judgement: Judgement, problem: Problem): string {
  const p = present(judgement, { harnessVisible: true, messages: MESSAGES.en });
  const v = judgement.verdict;
  // rustc's own rendering, help included: authors get more than players do.
  const diagnostics =
    "diagnostics" in v ? v.diagnostics.map((d) => cleanPaths(d.rendered ?? d.message)) : [];
  // Messages of hidden cases too, which players do not get.
  const cases = ("cases" in v ? v.cases : [])
    .filter((c) => c.status !== "pass")
    .map((c) => `    ${c.id}: ${c.status}${c.message ? ` (${cleanPaths(c.message)})` : ""}`);
  return [
    `  verdict: ${p.kind}${p.summary ? ` — ${p.summary}` : ""}`,
    ...diagnostics.map((d) => d.trimEnd().replace(/^/gm, "    ")),
    ...cases,
    judgement.stderr ? `  stderr: ${judgement.stderr}` : "",
    `  (problem ${problem.id})`,
  ]
    .filter((l) => l !== "")
    .join("\n");
}

async function main() {
  const { positionals } = parseArgs({ allowPositionals: true });
  const [toolchain, all, judgeSources] = await Promise.all([
    loadToolchain(),
    loadProblems(),
    loadJudgeSources(),
  ]);
  const selected =
    positionals.length === 0 ? all : all.filter((p) => positionals.includes(p.problem.id));
  let failures = 0;

  for (const { problem, solution } of selected) {
    const sol = await judge(toolchain, problem, judgeSources, solution.code);
    const tpl = await judge(toolchain, problem, judgeSources, problem.template);
    const errors: string[] = [];
    if (sol.judgement.verdict.kind !== "AC") {
      errors.push(`solution.rs is not AC\n${describe(sol.judgement, problem)}`);
    }
    if (sol.elapsedMs > problem.judge.timeLimitMs / 2) {
      errors.push(`solution.rs took ${sol.elapsedMs.toFixed(0)} ms (> half of time_limit_ms)`);
    }
    const tplKind = tpl.judgement.verdict.kind;
    if (tplKind === "AC") errors.push("template.rs is already AC");
    const mustBeCe = problem.kind === "repair" || problem.kind === "signature";
    if (mustBeCe && tplKind !== "CE") {
      errors.push(
        `template.rs of a ${problem.kind} problem must be CE\n${describe(tpl.judgement, problem)}`,
      );
    }
    if (!mustBeCe && (tplKind === "CE" || tplKind === "IE")) {
      errors.push(`template.rs must compile\n${describe(tpl.judgement, problem)}`);
    }

    const status = errors.length === 0 ? "ok  " : "FAIL";
    console.log(
      `${status} ${problem.id.padEnd(16)} solution ${sol.elapsedMs.toFixed(0).padStart(6)} ms, template ${tplKind}`,
    );
    for (const e of errors) console.log(`  - ${e.replace(/\n/g, "\n    ")}`);
    failures += errors.length === 0 ? 0 : 1;
  }

  console.log(`\n${selected.length - failures}/${selected.length} problems OK`);
  process.exitCode = failures === 0 ? 0 : 1;
}

await main();
