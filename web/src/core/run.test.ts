import { describe, expect, test } from "vite-plus/test";
import type { Judgement, Verdict } from "./classify.ts";
import {
  type Attempt,
  type Best,
  bestOverall,
  bestsByProblem,
  bestVerdicts,
  ceCount,
  improvedBest,
  newRun,
  recordAttempt,
  type Run,
} from "./run.ts";

function attempt(kind: Verdict["kind"], at = 1): Attempt {
  const verdict = (
    kind === "IE"
      ? { kind, detail: "" }
      : kind === "CE"
        ? { kind, diagnostics: [] }
        : { kind, cases: [] }
  ) as Verdict;
  const judgement: Judgement = { verdict, output: "", stderr: "", warnings: [] };
  return { submittedAt: at, code: "", judgement };
}

const fresh = (): Run => newRun({ id: "r", problemId: "p", mode: "standard", now: 0 });

describe("recordAttempt", () => {
  test("AC solves the run", () => {
    const run = [attempt("CE", 1), attempt("WA", 2), attempt("AC", 3)].reduce(
      recordAttempt,
      fresh(),
    );
    expect(run.status).toBe("solved");
    expect(run.finishedAt).toBe(3);
    expect(run.attempts).toHaveLength(3);
    expect(ceCount(run)).toBe(1);
  });

  test("internal errors are not counted", () => {
    const run = recordAttempt(fresh(), attempt("IE"));
    expect(run.attempts).toHaveLength(0);
  });

  test("there is no limit on the number of submissions", () => {
    const run = Array.from({ length: 10 }, () => attempt("WA")).reduce(recordAttempt, fresh());
    expect(run.status).toBe("playing");
    expect(run.attempts).toHaveLength(10);
  });

  test("a finished run ignores further submissions", () => {
    const solved = recordAttempt(fresh(), attempt("AC"));
    expect(recordAttempt(solved, attempt("CE"))).toBe(solved);
  });
});

describe("improvedBest", () => {
  const solvedIn = (kinds: Verdict["kind"][]) =>
    kinds.map((k) => attempt(k, 10)).reduce(recordAttempt, fresh());
  const best = (submissions: number, ceCount: number): Best => ({
    problemId: "p",
    mode: "standard",
    submissions,
    ceCount,
    achievedAt: 0,
    runId: "old",
  });

  test("an assisted run is never a best", () => {
    const assisted = ["AC"]
      .map((k) => attempt(k as Verdict["kind"], 10))
      .reduce(
        recordAttempt,
        newRun({ id: "r", problemId: "p", mode: "standard", now: 0, assisted: true }),
      );
    expect(assisted.status).toBe("solved");
    expect(improvedBest(undefined, assisted)).toBeUndefined();
  });

  test("first solve is a best", () => {
    expect(improvedBest(undefined, solvedIn(["AC"]))).toMatchObject({
      submissions: 1,
      ceCount: 0,
      runId: "r",
    });
  });

  test("fewer submissions improve, more do not", () => {
    expect(improvedBest(best(3, 0), solvedIn(["CE", "AC"]))).toBeDefined();
    expect(improvedBest(best(1, 0), solvedIn(["CE", "AC"]))).toBeUndefined();
  });

  test("ties are broken by fewer compile errors", () => {
    expect(improvedBest(best(2, 1), solvedIn(["WA", "AC"]))).toBeDefined();
    expect(improvedBest(best(2, 0), solvedIn(["CE", "AC"]))).toBeUndefined();
  });

  test("unsolved runs are never a best", () => {
    expect(improvedBest(undefined, fresh())).toBeUndefined();
  });
});

describe("bestVerdicts", () => {
  const solved = [attempt("CE", 1), attempt("WA", 2), attempt("AC", 3)].reduce(
    recordAttempt,
    fresh(),
  );
  const best: Best = {
    problemId: "p",
    mode: "standard",
    submissions: 3,
    ceCount: 1,
    achievedAt: 3,
    runId: "r",
  };

  test("returns the verdicts of the best run", () => {
    expect(bestVerdicts(best, solved)).toEqual(["CE", "WA", "AC"]);
  });

  test.each([
    ["the run is missing", best, undefined],
    ["the run is another one", { ...best, runId: "other" }, solved],
    ["the run does not match the record", { ...best, submissions: 2 }, solved],
  ])("is undefined when %s", (_, b, run) => {
    expect(bestVerdicts(b, run)).toBeUndefined();
  });
});

describe("bestOverall", () => {
  const best = (mode: Best["mode"], submissions: number, achievedAt: number, problemId = "p") => ({
    problemId,
    mode,
    submissions,
    ceCount: 0,
    achievedAt,
    runId: `${mode}-${submissions}-${achievedAt}`,
  });

  test("prefers fewer submissions", () => {
    expect(bestOverall([best("oni", 3, 1), best("beginner", 2, 1)])?.mode).toBe("beginner");
  });

  test("breaks ties by the harder mode, then the newer record", () => {
    expect(bestOverall([best("standard", 2, 1), best("advanced", 2, 1)])?.mode).toBe("advanced");
    expect(bestOverall([best("standard", 2, 1), best("standard", 2, 5)])?.achievedAt).toBe(5);
  });

  test("no records", () => {
    expect(bestOverall([])).toBeUndefined();
  });

  test("bestsByProblem picks one record per problem", () => {
    const map = bestsByProblem([
      best("standard", 2, 1, "a"),
      best("oni", 1, 1, "a"),
      best("standard", 4, 1, "b"),
    ]);
    expect(map.get("a")?.mode).toBe("oni");
    expect(map.get("b")?.submissions).toBe(4);
  });
});
