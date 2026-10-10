import { expect, test } from "vite-plus/test";
import { lineDiff } from "./diff.ts";
import { parseRoute, routeHref } from "./route.ts";
import { DEFAULT_SETTINGS, normalizeSettings, notepadEditor } from "./settings.ts";

test.each([
  ["", { page: "list" }],
  ["#/", { page: "list" }],
  ["#/p/lifetime-01", { page: "play", problemId: "lifetime-01" }],
  ["#/p/lifetime-01/history", { page: "history", problemId: "lifetime-01" }],
  ["#/settings", { page: "settings" }],
  ["#/challenge", { page: "challenge" }],
  ["#/unknown", { page: "list" }],
] as const)("parseRoute(%j)", (hash, route) => {
  expect(parseRoute(hash)).toEqual(route);
  expect(parseRoute(routeHref(route))).toEqual(route);
});

test("normalizeSettings fills defaults and rejects invalid values", () => {
  expect(normalizeSettings(undefined)).toEqual(DEFAULT_SETTINGS);
  expect(
    normalizeSettings({ mode: "oni", lives: 7, editor: { fontSize: 99, closeBrackets: false } }),
  ).toEqual({
    ...DEFAULT_SETTINGS,
    mode: "oni",
    editor: { ...DEFAULT_SETTINGS.editor, closeBrackets: false },
  });
});

test("notepadEditor turns every aid off", () => {
  expect(notepadEditor(DEFAULT_SETTINGS.editor)).toMatchObject({
    syntaxHighlight: false,
    closeBrackets: false,
    autoIndent: false,
  });
});

test("lineDiff", () => {
  expect(lineDiff("a\nb\n", "a\nc\n")).toEqual([
    { kind: "same", text: "a" },
    { kind: "removed", text: "b" },
    { kind: "added", text: "c" },
  ]);
});
