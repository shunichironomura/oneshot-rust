import { expect, test } from "vite-plus/test";
import { BOX, digitOf, marksFor, type Score } from "./marks.ts";

/** The number of strokes (`M`s) in a path. */
const strokes = (d: string) => d.split("M").length - 1;

test("a hole in one is circled, over par boxed, twice over double boxed", () => {
  expect(marksFor({ kind: "solved", shots: 1 }, "a")).toHaveLength(1);
  expect(strokes(marksFor({ kind: "solved", shots: 1 }, "a")[0])).toBe(1);
  expect(marksFor({ kind: "solved", shots: 2 }, "a")).toHaveLength(1);
  expect(strokes(marksFor({ kind: "solved", shots: 2 }, "a")[0])).toBe(4);
  expect(marksFor({ kind: "solved", shots: 3 }, "a")).toHaveLength(2);
  expect(marksFor({ kind: "solved", shots: 9 }, "a")).toHaveLength(2);
});

test("a total is marked against the round's par", () => {
  expect(marksFor({ kind: "solved", shots: 3, par: 3 }, "a")).toHaveLength(1);
  expect(strokes(marksFor({ kind: "solved", shots: 4, par: 3 }, "a")[0])).toBe(4);
  expect(marksFor({ kind: "solved", shots: 5, par: 3 }, "a")).toHaveLength(2);
});

test("a hole not finished gets a cross, the rest nothing", () => {
  expect(strokes(marksFor({ kind: "unsolved" }, "a")[0])).toBe(2);
  expect(marksFor({ kind: "plain", shots: 4 }, "a")).toEqual([]);
  expect(marksFor({ kind: "empty" }, "a")).toEqual([]);
});

test("a mark keeps its shape for a seed, and differs between seeds", () => {
  const score: Score = { kind: "solved", shots: 1 };
  expect(marksFor(score, "r:0")).toEqual(marksFor(score, "r:0"));
  expect(marksFor(score, "r:0")).not.toEqual(marksFor(score, "r:1"));
});

test("the marks stay around the box", () => {
  const numbers = (d: string) => d.match(/-?\d+(\.\d+)?/g)?.map(Number) ?? [];
  for (const score of [
    { kind: "solved", shots: 1 },
    { kind: "solved", shots: 3 },
    { kind: "unsolved" },
  ] satisfies Score[]) {
    for (const d of marksFor(score, "seed")) {
      for (const n of numbers(d)) {
        expect(n).toBeGreaterThan(-4);
        expect(n).toBeLessThan(BOX + 4);
      }
    }
  }
});

test("the digit is the count, when there is one", () => {
  expect(digitOf({ kind: "solved", shots: 2 })).toBe(2);
  expect(digitOf({ kind: "plain", shots: 4 })).toBe(4);
  expect(digitOf({ kind: "unsolved" })).toBeUndefined();
  expect(digitOf({ kind: "empty" })).toBeUndefined();
});
