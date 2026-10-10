import { expect, test } from "vite-plus/test";
import { errorsOf, parseStderr, warningsOf } from "./diagnostics.ts";

const diag = (level: string, message: string, spans: unknown[] = []) =>
  JSON.stringify({
    $message_type: "diagnostic",
    message,
    code: null,
    level,
    spans,
    children: [],
    rendered: null,
  });

test("parseStderr separates diagnostics from plain text", () => {
  const stderr = [diag("error", "boom"), "panicked: x", diag("warning", "unused"), ""].join("\n");
  const parsed = parseStderr(stderr);
  expect(parsed.diagnostics.map((d) => d.message)).toEqual(["boom", "unused"]);
  expect(parsed.text).toBe("panicked: x");
});

test("errorsOf drops the trailing summary; warningsOf drops the count", () => {
  const { diagnostics } = parseStderr(
    [
      diag("error", "boom"),
      diag("error", "aborting due to 1 previous error"),
      diag("warning", "1 warning emitted"),
    ].join("\n"),
  );
  expect(errorsOf(diagnostics).map((d) => d.message)).toEqual(["boom"]);
  expect(warningsOf(diagnostics)).toEqual([]);
});

test("a user line that merely looks like JSON stays text", () => {
  expect(parseStderr('{"not": "a diagnostic"}').text).toBe('{"not": "a diagnostic"}');
});
