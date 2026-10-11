import type { VerdictKind } from "./classify.ts";
import type { Messages } from "./i18n.ts";

/** The one title left from golf: solving a problem with the first submission. */
export const HOLE_IN_ONE = "Hole in One";

export function titleFor(submissions: number): string | undefined {
  return submissions === 1 ? HOLE_IN_ONE : undefined;
}

export function verdictEmoji(kind: VerdictKind): string {
  switch (kind) {
    case "AC":
      return "🟩";
    case "CE":
      return "🟥";
    case "WA":
    case "RE":
    case "UB":
    case "TLE":
      return "🟨";
    case "IE":
      return "⬜";
  }
}

/** How a verdict is colored: accepted, compile error, failed at run time, or not counted. */
export type VerdictTone = "ac" | "ce" | "miss" | "ie";

export function verdictTone(kind: VerdictKind): VerdictTone {
  switch (kind) {
    case "AC":
      return "ac";
    case "CE":
      return "ce";
    case "WA":
    case "RE":
    case "UB":
    case "TLE":
      return "miss";
    case "IE":
      return "ie";
  }
}

export interface ShareInput {
  problemId: string;
  verdicts: VerdictKind[];
  solved: boolean;
}

/** Wordle-style result text for the clipboard: the verdicts and the number of tries. */
export function shareText(input: ShareInput, messages: Messages): string {
  const squares = input.verdicts.map(verdictEmoji).join("");
  const tries = messages.shareTries(input.verdicts.length);
  const result = input.solved ? tries : `${tries} (${messages.unsolved})`;
  return `OneShot Rust 🦀 ${input.problemId}\n${squares}  ${result}`;
}
