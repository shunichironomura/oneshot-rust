import { expect, test } from "vite-plus/test";
import { keyTarget } from "./tabs.ts";

test.each([
  ["ArrowRight", 0, "horizontal", 1],
  ["ArrowRight", 2, "horizontal", 0],
  ["ArrowLeft", 0, "horizontal", 2],
  ["ArrowLeft", 2, "horizontal", 1],
  ["ArrowDown", 0, "horizontal", undefined],
  ["ArrowDown", 2, "vertical", 0],
  ["ArrowUp", 1, "vertical", 0],
  ["ArrowRight", 1, "vertical", undefined],
  ["ArrowDown", 1, "both", 2],
  ["ArrowLeft", 1, "both", 0],
  ["Home", 2, "vertical", 0],
  ["End", 0, "horizontal", 2],
  ["Enter", 1, "both", undefined],
] as const)("keyTarget(%s) from %i of 3, %s", (key, current, orientation, target) => {
  expect(keyTarget(key, current, 3, orientation)).toBe(target);
});

test("keyTarget moves nowhere without options", () => {
  expect(keyTarget("Home", 0, 0, "horizontal")).toBeUndefined();
});
