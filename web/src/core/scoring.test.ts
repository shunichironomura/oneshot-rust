import { expect, test } from "vite-plus/test";
import { MESSAGES } from "./i18n.ts";
import { type ShareInput, shareText, titleFor } from "./scoring.ts";

test.each([
  [1, 2, "Hole in One"],
  [1, 1, "Hole in One"],
  [2, 3, "Birdie"],
  [2, 4, "Eagle"],
  [3, 3, "Par"],
  [4, 3, "Bogey"],
  [5, 3, "Double Bogey"],
  [6, 3, "Triple Bogey"],
  [8, 3, "+5"],
])("titleFor(%i, par %i) = %s", (n, par, title) => {
  expect(titleFor(n, par)).toBe(title);
});

const solvedRun: ShareInput = {
  problemId: "lifetime-01",
  mode: "standard",
  par: 2,
  verdicts: ["CE", "WA", "AC"],
  solved: true,
};

test("shareText in Japanese matches the design", () => {
  expect(shareText(solvedRun, MESSAGES.ja)).toBe(
    "OneShot Rust 🦀 lifetime-01 [標準]\n🟥🟨🟩  3回 (Par 2, Bogey)",
  );
});

test("shareText in English", () => {
  expect(shareText(solvedRun, MESSAGES.en)).toBe(
    "OneShot Rust 🦀 lifetime-01 [Standard]\n🟥🟨🟩  3 tries (Par 2, Bogey)",
  );
});

test("shareText for an unsolved run", () => {
  const input: ShareInput = {
    problemId: "x",
    mode: "oni",
    par: 2,
    verdicts: ["CE", "CE", "UB"],
    solved: false,
  };
  expect(shareText(input, MESSAGES.en)).toBe(
    "OneShot Rust 🦀 x [Extreme]\n🟥🟥🟨  3 tries (unsolved)",
  );
  expect(shareText(input, MESSAGES.ja)).toBe("OneShot Rust 🦀 x [鬼]\n🟥🟥🟨  3回 (未解決)");
});
