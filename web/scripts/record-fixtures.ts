// Records raw Miri outputs used as test fixtures for `classify` (src/core/__fixtures__/).
//
// Usage: node scripts/record-fixtures.ts
import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { loadJudgeSources, loadProblems } from "../content/load.ts";
import { judge, loadToolchain } from "./node-runner.ts";

const FIXTURES_DIR = path.resolve(import.meta.dirname, "../src/core/__fixtures__");

const LONGEST_WORD_CE = `pub fn longest_word(s: &str) -> &str {
    let words: Vec<String> = s.split_whitespace().map(|w| w.to_string()).collect();
    let best = words.iter().max_by_key(|w| w.len()).unwrap();
    &best
}
`;
const LONGEST_WORD_WRONG_SIGNATURE = `pub fn longest_word(s: String) -> String {
    s
}
`;
const LONGEST_WORD_EXIT = `pub fn longest_word(s: &str) -> &str {
    if s.is_empty() {
        std::process::exit(3)
    }
    s
}
`;

/** [fixture name, problem id, code (or "solution"/"template")] */
const CASES: [string, string, string][] = [
  ["ac", "lifetime-01", "solution"],
  ["wa-todo", "lifetime-01", "template"],
  ["ce-borrow", "lifetime-01", LONGEST_WORD_CE],
  ["ce-harness", "lifetime-01", LONGEST_WORD_WRONG_SIGNATURE],
  ["re-exit", "lifetime-01", LONGEST_WORD_EXIT],
  ["ub-dangling", "unsafe-01", "template"],
];

const [toolchain, problems, judgeSources] = await Promise.all([
  loadToolchain(),
  loadProblems(),
  loadJudgeSources(),
]);
await mkdir(FIXTURES_DIR, { recursive: true });
for (const [name, problemId, code] of CASES) {
  const entry = problems.find((p) => p.problem.id === problemId);
  if (entry === undefined) throw new Error(`unknown problem ${problemId}`);
  const source =
    code === "solution" ? entry.solution : code === "template" ? entry.problem.template : code;
  const { output, nonce, judgement } = await judge(toolchain, entry.problem, judgeSources, source);
  const fixture = { problemId, caseIds: entry.problem.caseIds, nonce, output };
  await writeFile(path.join(FIXTURES_DIR, `${name}.json`), `${JSON.stringify(fixture, null, 2)}\n`);
  console.log(`${name}: ${judgement.verdict.kind}`);
}
