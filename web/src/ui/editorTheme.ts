// Editor colors. Everything refers to CSS variables (see styles.css) so that the
// editor follows the light/dark color scheme of the page.
import { HighlightStyle } from "@codemirror/language";
import { EditorView } from "@codemirror/view";
import { tags as t } from "@lezer/highlight";

export const highlightStyle = HighlightStyle.define([
  {
    tag: [t.keyword, t.definitionKeyword, t.moduleKeyword, t.controlKeyword],
    color: "var(--syn-keyword)",
  },
  { tag: [t.modifier, t.operatorKeyword, t.self], color: "var(--syn-keyword)" },
  {
    tag: [t.function(t.variableName), t.function(t.definition(t.variableName))],
    color: "var(--syn-function)",
  },
  { tag: t.function(t.propertyName), color: "var(--syn-function)" },
  { tag: t.macroName, color: "var(--syn-macro)" },
  { tag: [t.typeName, t.namespace], color: "var(--syn-type)" },
  { tag: [t.string, t.special(t.string), t.character], color: "var(--syn-string)" },
  { tag: [t.integer, t.float, t.bool], color: "var(--syn-number)" },
  { tag: [t.lineComment, t.blockComment], color: "var(--syn-comment)", fontStyle: "italic" },
  { tag: t.special(t.variableName), color: "var(--syn-lifetime)" },
  { tag: t.labelName, color: "var(--syn-lifetime)" },
  { tag: t.propertyName, color: "var(--syn-property)" },
  { tag: t.meta, color: "var(--syn-meta)" },
]);

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
