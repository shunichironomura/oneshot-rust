import { describe, expect, test } from "vite-plus/test";
import { parseStdout, runProgress, unescapeMessage } from "./protocol.ts";

const N = "abc123";
const line = (body: string) => `\x01J:${N}:${body}\n`;

describe("parseStdout", () => {
  test("parses a complete run", () => {
    const out =
      line("START") +
      line("CASE:sample-1:PASS") +
      line("CASE:hidden-1:FAIL:expected 1, got 2") +
      line("END");
    expect(parseStdout(out, N)).toEqual({
      started: true,
      ended: true,
      cases: [
        { id: "sample-1", pass: true },
        { id: "hidden-1", pass: false, message: "expected 1, got 2" },
      ],
      userOutput: "",
    });
  });

  test("separates the player's output from protocol lines", () => {
    const out = line("START") + "hello\n" + "no newline" + line("CASE:a:PASS") + "tail";
    const parsed = parseStdout(out, N);
    expect(parsed.userOutput).toBe("hello\nno newline\ntail");
    expect(parsed.cases).toEqual([{ id: "a", pass: true }]);
  });

  test("ignores lines with a forged nonce", () => {
    const out = line("START") + "\x01J:wrong:CASE:a:PASS\n";
    const parsed = parseStdout(out, N);
    expect(parsed.cases).toEqual([]);
    expect(parsed.userOutput).toBe("\x01J:wrong:CASE:a:PASS\n");
  });

  test("hides an incomplete protocol line while streaming", () => {
    const parsed = parseStdout(`out\n\x01J:${N}:CASE:a:PA`, N);
    expect(parsed.userOutput).toBe("out\n");
    expect(parsed.cases).toEqual([]);
  });

  test("unescapes failure messages", () => {
    const parsed = parseStdout(line("CASE:a:FAIL:line1\\nline2 \\\\n"), N);
    expect(parsed.cases[0].message).toBe("line1\nline2 \\n");
  });
});

test.each([
  ["a\\nb", "a\nb"],
  ["a\\\\nb", "a\\nb"],
  ["\\r", "\r"],
  ["plain", "plain"],
])("unescapeMessage(%j)", (input, expected) => {
  expect(unescapeMessage(input)).toBe(expected);
});

test("runProgress", () => {
  expect(runProgress("", N, 3)).toEqual({ phase: "compiling" });
  expect(runProgress(line("START") + line("CASE:a:PASS"), N, 3)).toEqual({
    phase: "running",
    done: 1,
    total: 3,
  });
});
