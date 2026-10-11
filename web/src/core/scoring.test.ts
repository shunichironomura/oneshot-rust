import { expect, test } from "vite-plus/test";
import { MESSAGES } from "./i18n.ts";
import { type ShareInput, shareText, titleFor } from "./scoring.ts";

test.each([
  [1, "Hole in One"],
  [2, undefined],
  [5, undefined],
])("titleFor(%i) = %s", (n, title) => {
  expect(titleFor(n)).toBe(title);
});

const solvedRun: ShareInput = {
  problemId: "lifetime-01",
  verdicts: ["CE", "WA", "AC"],
  solved: true,
};

test("shareText in Japanese", () => {
  expect(shareText(solvedRun, MESSAGES.ja)).toBe("OneShot Rust 🦀 lifetime-01\n🟥🟨🟩  3回");
});

test("shareText in English", () => {
  expect(shareText(solvedRun, MESSAGES.en)).toBe("OneShot Rust 🦀 lifetime-01\n🟥🟨🟩  3 tries");
});

test("shareText for an unsolved run", () => {
  const input: ShareInput = {
    problemId: "x",
    verdicts: ["CE", "CE", "UB"],
    solved: false,
  };
  expect(shareText(input, MESSAGES.en)).toBe("OneShot Rust 🦀 x\n🟥🟥🟨  3 tries (unsolved)");
  expect(shareText(input, MESSAGES.ja)).toBe("OneShot Rust 🦀 x\n🟥🟥🟨  3回 (未解決)");
});
