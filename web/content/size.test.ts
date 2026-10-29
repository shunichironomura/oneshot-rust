import { expect, test } from "vite-plus/test";
import { countLines, countWords, exceedances, measure, SIZE_LIMITS } from "./size.ts";

const files = {
  statementMd: "# Title\n\nImplement `f`.\n\n- one\n- two three\n",
  templateRs: "pub fn f() -> u32 {\n    todo!()\n}\n\npub fn g() -> u32 {\n    todo!()\n}\n",
  solutionRs:
    "pub fn f() -> u32 {\n    1 + 1\n}\n\npub fn g() -> u32 {\n    let x = 2;\n    x * 2\n}\n",
  testsRs: 'case("example-1", || Ok(()));\ncase("case-1", || Ok(()));\n',
};

test.each([
  ["", 0],
  ["a", 1],
  ["a\n", 1],
  ["a\nb", 2],
  ["a\n\nb\n", 3],
])("countLines(%j) = %i", (text, lines) => {
  expect(countLines(text)).toBe(lines);
});

test("countWords splits on any whitespace", () => {
  expect(countWords("")).toBe(0);
  expect(countWords(" a  b\n\tc\n")).toBe(3);
});

test("measure an implement problem: written lines = solution − template + todo!()s", () => {
  expect(measure("implement", files)).toEqual({
    writtenLines: 8 - 7 + 2,
    statementWords: 9,
    cases: 2,
    todos: 2,
  });
});

test("measure a repair problem counts diff hunks without context", () => {
  const m = measure("repair", files);
  expect(m.writtenLines).toBeUndefined();
  expect(m.hunks).toBe(2);
});

test("exceedances lists the metrics over the limits, in table order", () => {
  const over = {
    writtenLines: 21,
    statementWords: 401,
    cases: 11,
    todos: 1,
  };
  expect(exceedances(over, SIZE_LIMITS[1])).toEqual([
    { metric: "writtenLines", value: 21, limit: 20 },
    { metric: "statementWords", value: 401, limit: 400 },
    { metric: "cases", value: 11, limit: 10 },
  ]);
  expect(exceedances(over, SIZE_LIMITS[2])).toEqual([{ metric: "cases", value: 11, limit: 10 }]);
  expect(exceedances({ hunks: 3, statementWords: 1, cases: 1, todos: 0 }, SIZE_LIMITS[1])).toEqual(
    [],
  );
});
