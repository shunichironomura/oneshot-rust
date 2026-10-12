// Editor colors. Everything refers to CSS variables (see styles.css) so that the
// editor follows the light/dark color scheme of the page.
import { HighlightStyle } from "@codemirror/language";
import { EditorView } from "@codemirror/view";
import { type Tag, tagHighlighter, tags as t } from "@lezer/highlight";

/** Syntax token kinds; each has a `--syn-<kind>` color in styles.css. */
type Token =
  | "keyword"
  | "function"
  | "macro"
  | "type"
  | "string"
  | "number"
  | "comment"
  | "lifetime"
  | "property"
  | "meta";

const RULES: { tag: Tag | Tag[]; token: Token }[] = [
  {
    tag: [t.keyword, t.definitionKeyword, t.moduleKeyword, t.controlKeyword, t.modifier],
    token: "keyword",
  },
  { tag: [t.operatorKeyword, t.self], token: "keyword" },
  {
    tag: [t.function(t.variableName), t.function(t.definition(t.variableName))],
    token: "function",
  },
  { tag: t.function(t.propertyName), token: "function" },
  { tag: t.macroName, token: "macro" },
  { tag: [t.typeName, t.namespace], token: "type" },
  { tag: [t.string, t.special(t.string), t.character], token: "string" },
  { tag: [t.integer, t.float, t.bool], token: "number" },
  { tag: [t.lineComment, t.blockComment], token: "comment" },
  { tag: [t.special(t.variableName), t.labelName], token: "lifetime" },
  { tag: t.propertyName, token: "property" },
  { tag: t.meta, token: "meta" },
];

/** For the CodeMirror editor. */
export const highlightStyle = HighlightStyle.define(
  RULES.map(({ tag, token }) => ({
    tag,
    color: `var(--syn-${token})`,
    fontStyle: token === "comment" ? "italic" : undefined,
  })),
);

/** For static code outside the editor: assigns `syn-<kind>` classes (see styles.css). */
export const staticHighlighter = tagHighlighter(
  RULES.map(({ tag, token }) => ({ tag, class: `syn-${token}` })),
);

export const baseTheme = EditorView.theme({
  "&": {
    height: "100%",
    color: "var(--fg)",
    backgroundColor: "var(--editor-bg)",
  },
  // The font is the browser's monospace font, at the size of the page.
  ".cm-scroller": { lineHeight: "1.55" },
  ".cm-content": { caretColor: "var(--fg)", padding: "8px 0" },
  ".cm-cursor, .cm-dropCursor": { borderLeftColor: "var(--fg)" },
  ".cm-gutters": {
    backgroundColor: "var(--editor-bg)",
    color: "var(--muted)",
    border: "none",
    borderRight: "1px solid var(--border)",
  },
  ".cm-lineNumbers .cm-gutterElement": { padding: "0 10px 0 14px" },
  ".cm-activeLine": { backgroundColor: "var(--editor-active-line)" },
  ".cm-activeLineGutter": { backgroundColor: "var(--editor-active-line)", color: "var(--fg)" },
  "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection": {
    backgroundColor: "var(--editor-selection) !important",
  },
  "&.cm-focused .cm-matchingBracket": {
    backgroundColor: "var(--editor-bracket)",
    outline: "none",
  },
  "&.cm-focused": { outline: "none" },
  // The error tooltip (hover over an underlined range). CodeMirror's defaults assume a
  // light editor; this theme follows the page's color scheme instead, so the colors
  // must come from the same variables as the rest of the page.
  ".cm-tooltip": {
    backgroundColor: "var(--panel)",
    color: "var(--fg)",
    border: "1px solid var(--border)",
    borderRadius: "var(--radius)",
    boxShadow: "0 4px 16px rgb(0 0 0 / 0.25)",
    fontFamily: "var(--mono)",
    fontSize: "0.9em",
    lineHeight: "1.45",
    maxWidth: "min(60ch, 90vw)",
  },
  ".cm-tooltip-section:not(:first-child)": { borderTop: "1px solid var(--border)" },
  ".cm-tooltip.cm-tooltip-lint": { padding: "4px 0" },
  ".cm-diagnostic": { padding: "4px 10px 4px 10px" },
  ".cm-diagnostic-error": { borderLeftColor: "var(--ce)" },
  ".cm-diagnostic-warning": { borderLeftColor: "var(--wa)" },
  ".cm-diagnostic-info, .cm-diagnostic-hint": { borderLeftColor: "var(--muted)" },
});
