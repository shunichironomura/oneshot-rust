import { describe, expect, test } from "vite-plus/test";
import ceBorrow from "./__fixtures__/ce-borrow.json";
import ceHarness from "./__fixtures__/ce-harness.json";
import ubDangling from "./__fixtures__/ub-dangling.json";
import waTodo from "./__fixtures__/wa-todo.json";
import { classify, type RunOutput } from "./classify.ts";
import { MESSAGES } from "./i18n.ts";
import { cleanPaths, present } from "./present.ts";

interface Fixture {
  caseIds: string[];
  nonce: string;
  output: RunOutput;
}

function view(fixture: Fixture, harnessVisible = false) {
  const judgement = classify(fixture.output, {
    nonce: fixture.nonce,
    caseIds: fixture.caseIds,
    stdoutLimitBytes: 1024,
  });
  return present(judgement, { harnessVisible, messages: MESSAGES.ja });
}

describe("CE", () => {
  test("shows the code, message and snippet but no help", () => {
    const v = view(ceBorrow);
    expect(v.diagnostics[0]).toMatch(
      /^error\[E0515\]: cannot return value referencing local variable `words`/,
    );
    expect(v.diagnostics[0]).toContain("--> user.rs:4:5");
    expect(v.diagnostics[0]).toContain("4 |     &best");
    expect(v.diagnostics[0]).not.toContain("help");
  });

  test("marks the lines of the player's code in the editor", () => {
    expect(view(ceBorrow).markers).toEqual([
      { line: 4, message: "cannot return value referencing local variable `words`" },
    ]);
  });

  test("hidden harness code is never shown nor marked", () => {
    const v = view(ceHarness);
    const text = v.diagnostics.join("\n");
    expect(text).toContain("--> tests.rs （ハーネス側でエラー。コードは非公開）");
    expect(text).not.toContain("check_eq(");
    expect(v.markers).toEqual([]);
  });

  test("a visible harness is shown as is", () => {
    expect(view(ceHarness, true).diagnostics.join("\n")).toContain("check_eq(");
  });
});

test("cases show the messages of public cases only", () => {
  const cases = view(waTodo).cases ?? [];
  expect(cases.find((c) => c.id === "sample-1")?.message).toBe(
    "panicked: not yet implemented at user.rs:2:5",
  );
  expect(cases.find((c) => c.id === "hidden-1")?.message).toBeUndefined();
});

test("a visible harness makes every case public", () => {
  const cases = view(waTodo, true).cases ?? [];
  expect(cases.every((c) => c.visible)).toBe(true);
  expect(cases.find((c) => c.id === "hidden-1")?.message).toBe(
    "panicked: not yet implemented at user.rs:2:5",
  );
});

test("UB drops help but keeps the location", () => {
  const v = view(ubDangling);
  expect(v.kind).toBe("UB");
  expect(v.diagnostics[0]).toMatch(/^error: Undefined Behavior/);
  expect(v.diagnostics[0]).toContain("--> user.rs:");
  expect(v.diagnostics[0]).not.toContain("help:");
  expect(v.markers.length).toBeGreaterThan(0);
});

test("cleanPaths", () => {
  expect(cleanPaths("panicked at /src/user.rs:3:1")).toBe("panicked at user.rs:3:1");
});
