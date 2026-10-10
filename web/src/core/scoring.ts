import type { VerdictKind } from "./classify.ts";
import { MODE_LABELS, type Mode } from "./mode.ts";

/** Golf-style title for solving a problem in `submissions` tries. */
export function titleFor(submissions: number, par: number): string {
  if (submissions === 1) return "Hole in One";
  const diff = submissions - par;
  switch (diff) {
    case -3:
      return "Albatross";
    case -2:
      return "Eagle";
    case -1:
      return "Birdie";
    case 0:
      return "Par";
    case 1:
      return "Bogey";
    case 2:
      return "Double Bogey";
    case 3:
      return "Triple Bogey";
    default:
      return diff < 0 ? "Condor" : `+${diff}`;
  }
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

export interface ShareInput {
  problemId: string;
  mode: Mode;
  par: number;
  verdicts: VerdictKind[];
  solved: boolean;
}

/** Wordle-style result text for the clipboard. */
export function shareText(input: ShareInput): string {
  const n = input.verdicts.length;
  const squares = input.verdicts.map(verdictEmoji).join("");
  const result = input.solved
    ? `${n}回 (Par ${input.par}, ${titleFor(n, input.par)})`
    : `${n}回 (未解決)`;
  return `OneShot Rust 🦀 ${input.problemId} [${MODE_LABELS[input.mode]}]\n${squares}  ${result}`;
}
