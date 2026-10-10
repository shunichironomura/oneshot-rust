import { expect, test } from "vite-plus/test";
import { extractCaseIds, parseProblem, type ProblemFiles } from "./parse.ts";

const files = (toml: string, tests = 'case("sample-1", || Ok(()));'): ProblemFiles => ({
  dirName: "p-01",
  problemToml: toml,
  statementMd: "# hi",
  templateRs: "pub fn f() {}",
  testsRs: tests,
  solutionRs: "pub fn f() {}",
});

const MINIMAL = `id = "p-01"
title = "T"
difficulty = 1
par = 1
concepts = ["x"]
kind = "implement"
`;

test("parseProblem applies judge defaults and renders the statement", () => {
  const { problem, solution } = parseProblem(files(MINIMAL));
  expect(problem.judge).toEqual({
    forbidUnsafe: true,
    miriChecks: "full",
    timeLimitMs: 15000,
    stdoutLimitBytes: 65536,
    harnessVisible: false,
  });
  expect(problem.statementHtml).toBe("<h1>hi</h1>\n");
  expect(problem.caseIds).toEqual(["sample-1"]);
  expect(solution).toBe("pub fn f() {}");
});

test.each([
  [MINIMAL.replace('id = "p-01"', 'id = "other"'), /must match the directory name/],
  [MINIMAL.replace("difficulty = 1", "difficulty = 9"), /difficulty/],
  [MINIMAL.replace('kind = "implement"', 'kind = "golf"'), /kind/],
  [MINIMAL.replace('kind = "implement"', 'kind = "signature"'), /harness_visible/],
  [`${MINIMAL}\n[judge]\nmiri_checks = "slow"\n`, /judge\.miri_checks/],
  ["not toml =", /invalid problem\.toml/],
])("parseProblem rejects invalid problem.toml (%#)", (toml, error) => {
  expect(() => parseProblem(files(toml))).toThrow(error);
});

test("parseProblem rejects duplicate or missing cases", () => {
  expect(() => parseProblem(files(MINIMAL, ""))).toThrow(/at least one case/);
  expect(() => parseProblem(files(MINIMAL, 'case("a", f); case("a", g);'))).toThrow(
    /duplicate case id "a"/,
  );
});

test("extractCaseIds", () => {
  expect(extractCaseIds('case("sample-1", || x);\n    case(\n "hidden-1", || y);')).toEqual([
    "sample-1",
    "hidden-1",
  ]);
});
