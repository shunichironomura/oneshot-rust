import { diffLines } from "diff";

export interface DiffLine {
  kind: "added" | "removed" | "same";
  text: string;
}

/** Line-based diff between two submissions. */
export function lineDiff(before: string, after: string): DiffLine[] {
  return diffLines(before, after).flatMap((change) => {
    const kind = change.added ? "added" : change.removed ? "removed" : "same";
    const lines = change.value.replace(/\n$/, "").split("\n");
    return lines.map((text): DiffLine => ({ kind, text }));
  });
}
