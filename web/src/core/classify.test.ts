import { describe, expect, test } from "vite-plus/test";
import ac from "./__fixtures__/ac.json";
import ceBorrow from "./__fixtures__/ce-borrow.json";
import ceHarness from "./__fixtures__/ce-harness.json";
import reExit from "./__fixtures__/re-exit.json";
import ubDangling from "./__fixtures__/ub-dangling.json";
import waTodo from "./__fixtures__/wa-todo.json";
import { classify, isCounted, type RunOutput } from "./classify.ts";

interface Fixture {
  caseIds: string[];
  nonce: string;
  output: RunOutput;
}

function run(fixture: Fixture, patch: Partial<RunOutput> = {}) {
  return classify(
    { ...fixture.output, ...patch },
    { nonce: fixture.nonce, caseIds: fixture.caseIds, stdoutLimitBytes: 1024 },
  );
}

describe("classify (recorded Miri output)", () => {
  test("AC", () => {
    const v = run(ac).verdict;
    expect(v.kind).toBe("AC");
  });

  test("WA from a panicking todo!()", () => {
    const v = run(waTodo).verdict;
    expect(v.kind).toBe("WA");
    if (v.kind !== "WA") return;
    expect(v.cases[0]).toMatchObject({ id: "example-1", visible: true, status: "fail" });
    expect(v.cases[0].message).toMatch(/^panicked: not yet implemented at \/src\/user\.rs:2:5/);
  });

  test("CE ends in a trap on WASI but is still CE", () => {
    expect(ceBorrow.output.exitCode).toBeNull();
    const v = run(ceBorrow).verdict;
    expect(v.kind).toBe("CE");
    if (v.kind !== "CE") return;
    expect(v.diagnostics.map((d) => d.code?.code)).toEqual(["E0515"]);
  });

  test("CE located in the hidden harness", () => {
    const v = run(ceHarness).verdict;
    expect(v.kind).toBe("CE");
    if (v.kind !== "CE") return;
    expect(v.diagnostics.every((d) => d.spans.some((s) => s.file_name === "/src/tests.rs"))).toBe(
      true,
    );
  });

  test("UB", () => {
    const v = run(ubDangling).verdict;
    expect(v.kind).toBe("UB");
    if (v.kind !== "UB") return;
    expect(v.diagnostics[0].message).toMatch(/^Undefined Behavior/);
  });

  test("RE when the program exits early", () => {
    const v = run(reExit).verdict;
    expect(v).toMatchObject({ kind: "RE", reason: { type: "exit-code", code: 3 } });
  });

  test("TLE reports the running case", () => {
    const v = run(waTodo, {
      timedOut: true,
      exitCode: null,
      stdout: waTodo.output.stdout.split("\n").slice(0, 2).join("\n") + "\n",
    });
    expect(v.verdict).toMatchObject({ kind: "TLE", runningCase: "example-2" });
  });

  test("output limit is a runtime error", () => {
    expect(run(ac, { outputLimitExceeded: true, exitCode: null }).verdict.kind).toBe("RE");
  });
});

describe("classify (edge cases)", () => {
  const empty: RunOutput = {
    stdout: "",
    stderr: "",
    exitCode: null,
    timedOut: false,
    outputLimitExceeded: false,
  };
  const ctx = { nonce: "n", caseIds: ["a"], stdoutLimitBytes: 1024 };

  test("a crash before START without diagnostics is an internal error", () => {
    expect(classify({ ...empty, crash: "RuntimeError: unreachable" }, ctx).verdict.kind).toBe("IE");
  });

  test("a timeout while compiling is an internal error", () => {
    expect(classify({ ...empty, timedOut: true }, ctx).verdict.kind).toBe("IE");
  });

  test("a crash after START without explanation is an internal error", () => {
    const stdout = "\x01J:n:START\n";
    expect(
      classify({ ...empty, stdout, crash: "RuntimeError: unreachable" }, ctx).verdict.kind,
    ).toBe("IE");
  });

  test("a panic outside of a case is a runtime error", () => {
    const out = {
      ...empty,
      stdout: "\x01J:n:START\n",
      stderr: "panicked: boom at /src/tests.rs:1:1\n",
      exitCode: 101,
    };
    expect(classify(out, ctx).verdict).toMatchObject({
      kind: "RE",
      reason: { type: "panic", message: "panicked: boom at /src/tests.rs:1:1" },
    });
  });
});

test.each([
  ["IE", false],
  ["CE", true],
  ["AC", true],
] as const)("isCounted(%s) = %s", (kind, counted) => {
  expect(isCounted(kind)).toBe(counted);
});
