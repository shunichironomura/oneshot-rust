import { highlightCode } from "@lezer/highlight";
import { parser } from "@lezer/rust";
import type { ComponentChildren } from "preact";
import { useMemo } from "preact/hooks";
import { staticHighlighter } from "./editorTheme.ts";

/** Read-only, syntax-highlighted Rust (no editor). Lines can be marked as errors. */
export function CodeView({ code, errorLines = [] }: { code: string; errorLines?: number[] }) {
  const lines = useMemo(() => {
    const out: ComponentChildren[][] = [[]];
    highlightCode(
      code,
      parser.parse(code),
      staticHighlighter,
      (text, classes) => {
        const line = out[out.length - 1];
        line.push(classes === "" ? text : <span class={classes}>{text}</span>);
      },
      () => out.push([]),
    );
    return out;
  }, [code]);
  return (
    <pre class="code-view">
      {lines.map((parts, i) => (
        <div key={i} class={errorLines.includes(i + 1) ? "code-line error" : "code-line"}>
          <span class="code-line-number">{i + 1}</span>
          <span>{parts.length === 0 ? " " : parts}</span>
        </div>
      ))}
    </pre>
  );
}
