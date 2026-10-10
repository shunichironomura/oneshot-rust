// Judges a single source file against a problem and prints what the player would see.
//
// Usage: node scripts/judge.ts <problem-id> <user.rs> [--mode beginner|standard|advanced|oni]
import { readFile } from "node:fs/promises";
import { parseArgs } from "node:util";
import { loadJudgeSources, loadProblems } from "../content/load.ts";
import { MODES, type Mode } from "../src/core/mode.ts";
import { present } from "../src/core/present.ts";
import { judge, loadToolchain } from "./node-runner.ts";

const { positionals, values } = parseArgs({
  allowPositionals: true,
  options: { mode: { type: "string", default: "standard" } },
});
const [problemId, file] = positionals;
const mode = MODES.find((m) => m === values.mode);
if (problemId === undefined || file === undefined || mode === undefined) {
  console.error("usage: node scripts/judge.ts <problem-id> <user.rs> [--mode <mode>]");
  process.exit(2);
}

const [toolchain, problems, judgeSources, code] = await Promise.all([
  loadToolchain(),
  loadProblems(),
  loadJudgeSources(),
  readFile(file, "utf8"),
]);
const entry = problems.find((p) => p.problem.id === problemId);
if (entry === undefined) {
  console.error(`unknown problem: ${problemId}`);
  process.exit(2);
}
const { problem } = entry;
const { judgement, elapsedMs } = await judge(toolchain, problem, judgeSources, code);
const view = present(judgement, {
  mode: mode satisfies Mode,
  harnessVisible: problem.judge.harnessVisible,
});
console.log(
  JSON.stringify(
    {
      elapsedMs: Math.round(elapsedMs),
      ...view,
      output: judgement.output,
      stderr: judgement.stderr,
    },
    null,
    2,
  ),
);
for (const d of view.diagnostics) console.log(`\n${d}`);
