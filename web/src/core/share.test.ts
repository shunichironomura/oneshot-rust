import { expect, test } from "vite-plus/test";
import { MESSAGES } from "./i18n.ts";
import { layoutShots, shareCard } from "./share.ts";

test("a solved run", () => {
  const card = shareCard(
    {
      problemId: "lifetime-01",
      problemTitle: "Longest word",
      mode: "standard",
      verdicts: ["CE", "WA", "AC"],
      solved: true,
    },
    MESSAGES.en,
  );
  expect(card).toEqual({
    headline: "Solved!",
    solved: true,
    problemTitle: "Longest word",
    detail: "3 tries · Standard",
    shots: [
      { label: "CE", tone: "ce" },
      { label: "WA", tone: "miss" },
      { label: "AC", tone: "ac" },
    ],
  });
});

test("a hole in one", () => {
  const card = shareCard(
    {
      problemId: "iter-01",
      problemTitle: "Sum of even squares",
      mode: "standard",
      verdicts: ["AC"],
      solved: true,
    },
    MESSAGES.en,
  );
  expect(card.headline).toBe("Hole in One!");
  expect(card.detail).toBe("1 try · Standard");
});

test("a failed run", () => {
  const card = shareCard(
    {
      problemId: "lifetime-01",
      problemTitle: "最長の単語",
      mode: "oni",
      verdicts: ["CE", "UB"],
      solved: false,
    },
    MESSAGES.ja,
  );
  expect(card.headline).toBe(MESSAGES.ja.unsolvedHeadline);
  expect(card.detail).toBe("2回 · 鬼");
});

test("few squares keep their maximum size", () => {
  expect(layoutShots(3, 1000, 80)).toEqual({ size: 80, xs: [0, 100, 200] });
});

test("many squares shrink to fit the width", () => {
  const row = layoutShots(20, 990, 80);
  expect(row.size).toBeCloseTo(40);
  expect(row.xs.at(-1)! + row.size).toBeCloseTo(990);
});

test("no squares", () => {
  expect(layoutShots(0, 1000, 80).xs).toEqual([]);
});
