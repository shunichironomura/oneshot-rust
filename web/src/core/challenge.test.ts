import { describe, expect, test } from "vite-plus/test";
import {
  challengeShareText,
  currentProblemId,
  newChallenge,
  pickProblems,
  recordRun,
  totalSubmissions,
} from "./challenge.ts";
import type { Judgement, Verdict } from "./classify.ts";
import { MESSAGES } from "./i18n.ts";
import type { Run, RunStatus } from "./run.ts";

function run(id: string, problemId: string, status: RunStatus, kinds: Verdict["kind"][]): Run {
  const attempts = kinds.map((kind) => {
    const verdict = (kind === "CE" ? { kind, diagnostics: [] } : { kind, cases: [] }) as Verdict;
    const judgement: Judgement = { verdict, output: "", stderr: "", warnings: [] };
    return { submittedAt: 5, code: "", judgement };
  });
  return {
    id,
    problemId,
    mode: "standard",
    lives: null,
    startedAt: 0,
    attempts,
    status,
    finishedAt: 5,
  };
}

const start = () =>
  newChallenge({ id: "c", mode: "standard", lives: null, problemIds: ["a", "b"], now: 0 });

test("pickProblems returns distinct problems", () => {
  let seed = 1;
  const random = () => ((seed = (seed * 16807) % 2147483647) - 1) / 2147483646;
  const picked = pickProblems(["a", "b", "c", "d", "e", "f"], 5, random);
  expect(picked).toHaveLength(5);
  expect(new Set(picked).size).toBe(5);
});

test("pickProblems with fewer problems than requested returns all", () => {
  expect(pickProblems(["a", "b"], 5, () => 0).sort()).toEqual(["a", "b"]);
});

describe("recordRun", () => {
  test("advances through the problems and finishes", () => {
    let c = start();
    expect(currentProblemId(c)).toBe("a");
    c = recordRun(c, run("r1", "a", "solved", ["CE", "AC"]));
    expect(currentProblemId(c)).toBe("b");
    expect(c.finishedAt).toBeUndefined();
    c = recordRun(c, run("r2", "b", "failed", ["WA", "WA", "WA"]));
    expect(currentProblemId(c)).toBeUndefined();
    expect(c.finishedAt).toBe(5);
    expect(totalSubmissions(c)).toBe(5);
  });

  test("ignores unfinished runs, other problems and duplicates", () => {
    const c = start();
    expect(recordRun(c, run("r1", "a", "playing", ["CE"]))).toBe(c);
    expect(recordRun(c, run("r1", "b", "solved", ["AC"]))).toBe(c);
    const once = recordRun(c, run("r1", "a", "solved", ["AC"]));
    expect(recordRun(once, run("r1", "a", "solved", ["AC"]))).toBe(once);
  });
});

test("challengeShareText", () => {
  let c = start();
  c = recordRun(c, run("r1", "a", "solved", ["CE", "AC"]));
  c = recordRun(c, run("r2", "b", "failed", []));
  expect(challengeShareText(c, MESSAGES.ja)).toBe(
    "OneShot Rust 🦀 連続チャレンジ [標準]\n🟥🟩  a\n— ✗  b\n合計 2 回 ・ 1/2 問正解",
  );
  expect(challengeShareText(c, MESSAGES.en)).toBe(
    "OneShot Rust 🦀 Challenge [Standard]\n🟥🟩  a\n— ✗  b\n2 submissions in total · 1/2 solved",
  );
});
