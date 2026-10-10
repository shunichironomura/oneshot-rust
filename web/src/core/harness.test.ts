import { describe, expect, test } from "vite-plus/test";
import { miriArgs, precheck, renderJudge, renderMain, stripCommentsAndStrings } from "./harness.ts";
import type { JudgeConfig } from "./problem.ts";

const judge: JudgeConfig = {
  forbidUnsafe: true,
  miriChecks: "full",
  timeLimitMs: 15000,
  stdoutLimitBytes: 65536,
  harnessVisible: false,
};

test("renderMain adds forbid(unsafe_code) only when configured", () => {
  const template = "/*@USER_ATTRS@*/\nmod user;";
  expect(renderMain(template, judge)).toBe("#[forbid(unsafe_code)]\nmod user;");
  expect(renderMain(template, { ...judge, forbidUnsafe: false })).toBe("\nmod user;");
});

test("renderJudge embeds the nonce and rejects unsafe nonces", () => {
  expect(renderJudge('const NONCE: &str = "__NONCE__";', "00ff")).toBe(
    'const NONCE: &str = "00ff";',
  );
  expect(() => renderJudge("", '"; evil')).toThrow();
});

test("miriArgs", () => {
  expect(miriArgs(judge)).toContain("--edition=2024");
  expect(miriArgs(judge)).not.toContain("-Zmiri-disable-validation");
  expect(miriArgs({ ...judge, miriChecks: "fast" })).toContain("-Zmiri-disable-validation");
});

describe("precheck", () => {
  test.each([
    ['const S: &str = include_str!("judge.rs");', "include_str"],
    ['let n = env!("X");', "env"],
    ['include ! ("x.rs");', "include"],
  ])("rejects %s", (code, macro) => {
    expect(precheck(code)).toContain(`${macro}!`);
  });

  test.each([
    '// include_str!("judge.rs")',
    "/* env!() */ fn f() {}",
    'let s = "include_str!(x)";',
    "let environment = 1; let included = environment;",
    "fn f<'a>(x: &'a str) -> &'a str { x }",
  ])("accepts %s", (code) => {
    expect(precheck(code)).toBeNull();
  });
});

test("stripCommentsAndStrings keeps lifetimes", () => {
  expect(stripCommentsAndStrings("fn f<'a>(x: &'a str) { 'c'; \"s\" }")).toBe(
    "fn f<'a>(x: &'a str) {  ;   }",
  );
});
