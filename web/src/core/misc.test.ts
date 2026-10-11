import { expect, test } from "vite-plus/test";
import { lineDiff } from "./diff.ts";
import { parseRoute, routeHref } from "./route.ts";
import { DEFAULT_SETTINGS, normalizeSettings } from "./settings.ts";

test.each([
  ["", { page: "home" }],
  ["#/", { page: "home" }],
  ["#/problems", { page: "problems" }],
  ["#/p/lifetime-01", { page: "practice", problemId: "lifetime-01" }],
  ["#/p/lifetime-01/history", { page: "history", problemId: "lifetime-01" }],
  ["#/round", { page: "round" }],
  ["#/round/abc", { page: "scorecard", roundId: "abc" }],
  ["#/unknown", { page: "home" }],
] as const)("parseRoute(%j)", (hash, route) => {
  expect(parseRoute(hash)).toEqual(route);
  expect(parseRoute(routeHref(route))).toEqual(route);
});

test.each(["#/challenge", "#/challenge/new"])(
  "the old challenge link %s opens the round",
  (hash) => {
    expect(parseRoute(hash)).toEqual({ page: "round" });
  },
);

test("normalizeSettings fills defaults and rejects invalid values", () => {
  expect(normalizeSettings(undefined)).toEqual(DEFAULT_SETTINGS);
  expect(
    normalizeSettings({
      mode: "oni",
      course: 4,
      locale: "fr",
      // Stored by an older version, which had editor settings.
      editor: { fontSize: 99, closeBrackets: false },
    }),
  ).toEqual({ ...DEFAULT_SETTINGS, mode: "oni" });
});

test("normalizeSettings keeps a valid course", () => {
  expect(normalizeSettings({ course: 3 }).course).toBe(3);
});

test("lineDiff", () => {
  expect(lineDiff("a\nb\n", "a\nc\n")).toEqual([
    { kind: "same", text: "a" },
    { kind: "removed", text: "b" },
    { kind: "added", text: "c" },
  ]);
});
