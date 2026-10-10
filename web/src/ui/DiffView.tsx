import { useMemo } from "preact/hooks";
import { lineDiff } from "../core/diff.ts";

export function DiffView({ before, after }: { before: string; after: string }) {
  const lines = useMemo(() => lineDiff(before, after), [before, after]);
  return (
    <pre class="diff">
      {lines.map((l, i) => (
        <div key={i} class={`diff-line ${l.kind}`}>
          {l.kind === "added" ? "+ " : l.kind === "removed" ? "- " : "  "}
          {l.text}
        </div>
      ))}
    </pre>
  );
}
