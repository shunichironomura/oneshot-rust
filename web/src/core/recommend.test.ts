import { expect, test } from "vite-plus/test";
import { recommend } from "./recommend.ts";

const ids = ["a", "b", "c"];

test("starts with the first problem", () => {
  expect(recommend(ids, new Set(), new Set())).toEqual({ kind: "start", problemId: "a" });
});

test("skips solved problems", () => {
  expect(recommend(ids, new Set(["a"]), new Set())).toEqual({ kind: "start", problemId: "b" });
});

test("prefers a problem in progress", () => {
  expect(recommend(ids, new Set(), new Set(["c"]))).toEqual({ kind: "continue", problemId: "c" });
});

test("all solved", () => {
  expect(recommend(ids, new Set(ids), new Set())).toEqual({ kind: "all-solved" });
});
