import { describe, expect, test } from "vite-plus/test";
import ceBorrow from "./__fixtures__/ce-borrow.json";
import ceHarness from "./__fixtures__/ce-harness.json";
import ubDangling from "./__fixtures__/ub-dangling.json";
import waTodo from "./__fixtures__/wa-todo.json";
import { classify, type RunOutput } from "./classify.ts";
import type { Mode } from "./mode.ts";
import { cleanPaths, present } from "./present.ts";

interface Fixture {
  caseIds: string[];
  nonce: string;
  output: RunOutput;
}

function view(fixture: Fixture, mode: Mode, harnessVisible = false) {
  const judgement = classify(fixture.output, {
    nonce: fixture.nonce,
    caseIds: fixture.caseIds,
    stdoutLimitBytes: 1024,
  });
  return present(judgement, { mode, harnessVisible });
}

describe("CE", () => {
  test("beginner shows rustc's rendering (with help) and editor markers", () => {
    const v = view(ceBorrow, "beginner");
    expect(v.diagnostics[0]).toContain("--> user.rs:4:5");
    expect(v.markers).toEqual([
      { line: 4, message: "cannot return value referencing local variable `words`" },
    ]);
  });

  test("standard shows code, message and snippet but no help", () => {
    const v = view(ceBorrow, "standard");
    expect(v.diagnostics[0]).toMatch(
      /^error\[E0515\]: cannot return value referencing local variable `words`/,
    );
    expect(v.diagnostics[0]).toContain("4 |     &best");
    expect(v.diagnostics[0]).not.toContain("help");
    expect(v.markers).toEqual([]);
  });

  test("advanced shows only the count and lines", () => {
    const v = view(ceBorrow, "advanced");
    expect(v.summary).toBe("エラー 1 件（user.rs:4）");
    expect(v.diagnostics).toEqual([]);
  });

  test("oni shows nothing", () => {
    const v = view(ceBorrow, "oni");
    expect(v.summary).toBeUndefined();
    expect(v.diagnostics).toEqual([]);
  });

  test("hidden harness code is never shown", () => {
    for (const mode of ["beginner", "standard"] as const) {
      const text = view(ceHarness, mode).diagnostics.join("\n");
      expect(text).toContain("--> tests.rs （ハーネス側でエラー。コードは非公開）");
      expect(text).not.toContain("check_eq(");
    }
  });

  test("a visible harness is shown as is", () => {
    expect(view(ceHarness, "standard", true).diagnostics.join("\n")).toContain("check_eq(");
  });
});

describe("cases", () => {
  test("standard hides messages of hidden cases", () => {
    const cases = view(waTodo, "standard").cases ?? [];
    expect(cases.find((c) => c.id === "sample-1")?.message).toBe(
      "panicked: not yet implemented at user.rs:2:5",
    );
    expect(cases.find((c) => c.id === "hidden-1")?.message).toBeUndefined();
  });

  test("beginner shows every message", () => {
    const cases = view(waTodo, "beginner").cases ?? [];
    expect(cases.find((c) => c.id === "hidden-1")?.message).toBeDefined();
  });

  test("advanced shows only statuses; oni hides cases", () => {
    expect((view(waTodo, "advanced").cases ?? []).every((c) => c.message === undefined)).toBe(true);
    expect(view(waTodo, "oni").cases).toBeNull();
  });
});

test("UB in standard mode drops help but keeps the location", () => {
  const v = view(ubDangling, "standard");
  expect(v.kind).toBe("UB");
  expect(v.diagnostics[0]).toMatch(/^error: Undefined Behavior/);
  expect(v.diagnostics[0]).toContain("--> user.rs:");
  expect(v.diagnostics[0]).not.toContain("help:");
});

test("cleanPaths", () => {
  expect(cleanPaths("panicked at /src/user.rs:3:1")).toBe("panicked at user.rs:3:1");
});
