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
});
