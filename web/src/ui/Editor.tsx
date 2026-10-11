import { closeBrackets, closeBracketsKeymap } from "@codemirror/autocomplete";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { rust } from "@codemirror/lang-rust";
import {
  bracketMatching,
  indentOnInput,
  indentUnit,
  syntaxHighlighting,
} from "@codemirror/language";
import { type Diagnostic as LintDiagnostic, lintGutter, setDiagnostics } from "@codemirror/lint";
import { Compartment, EditorState } from "@codemirror/state";
import {
  drawSelection,
  EditorView,
  highlightActiveLine,
  highlightActiveLineGutter,
  keymap,
  lineNumbers,
} from "@codemirror/view";
import { useEffect, useRef } from "preact/hooks";
import type { EditorMarker } from "../core/present.ts";
import { baseTheme, highlightStyle } from "./editorTheme.ts";

interface Props {
  /**
   * Initial document. The editor is uncontrolled: it owns the document and reports
   * edits via `onChange`. To replace the content, remount it with a new `key`.
   */
  initialValue: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
  markers: EditorMarker[];
  readOnly?: boolean;
}

function toLintDiagnostics(state: EditorState, markers: EditorMarker[]): LintDiagnostic[] {
  return markers
    .filter((m) => m.line >= 1 && m.line <= state.doc.lines)
    .map((m) => {
      const line = state.doc.line(m.line);
      // Underline the code, not the indentation before it.
      const indent = line.text.length - line.text.trimStart().length;
      return { from: line.from + indent, to: line.to, severity: "error", message: m.message };
    });
}

function sameMarkers(a: EditorMarker[], b: EditorMarker[]): boolean {
  return (
    a.length === b.length && a.every((m, i) => m.line === b[i].line && m.message === b[i].message)
  );
}

export function Editor({ initialValue, onChange, onSubmit, markers, readOnly = false }: Props) {
  const host = useRef<HTMLDivElement>(null);
  const view = useRef<EditorView | null>(null);
  const readOnlyCompartment = useRef(new Compartment());
  // Keep the latest callbacks without recreating the editor.
  const callbacks = useRef({ onChange, onSubmit });
  callbacks.current = { onChange, onSubmit };

  useEffect(() => {
    if (host.current === null) return;
    const v = new EditorView({
      parent: host.current,
      state: EditorState.create({
        doc: initialValue,
        extensions: [
          lineNumbers(),
          highlightActiveLineGutter(),
          highlightActiveLine(),
          drawSelection(),
          bracketMatching(),
          closeBrackets(),
          indentOnInput(),
          rust(),
          syntaxHighlighting(highlightStyle),
          history(),
          lintGutter(),
          indentUnit.of("    "),
          EditorState.tabSize.of(4),
          keymap.of([
            { key: "Mod-Enter", run: () => (callbacks.current.onSubmit(), true) },
            indentWithTab,
            ...closeBracketsKeymap,
            ...defaultKeymap,
            ...historyKeymap,
          ]),
          baseTheme,
          readOnlyCompartment.current.of(EditorState.readOnly.of(readOnly)),
          EditorView.updateListener.of((u) => {
            if (u.docChanged) callbacks.current.onChange(u.state.doc.toString());
          }),
        ],
      }),
    });
    view.current = v;
    return () => v.destroy();
    // The editor is created once; later prop changes are applied by the effects below.
  }, []);

  useEffect(() => {
    view.current?.dispatch({
      effects: readOnlyCompartment.current.reconfigure(EditorState.readOnly.of(readOnly)),
    });
  }, [readOnly]);

  // Dispatch only when the markers actually change: a transaction on every render would
  // feed back into onChange and re-render again.
  const shownMarkers = useRef<EditorMarker[]>([]);
  useEffect(() => {
    const v = view.current;
    if (v === null || sameMarkers(shownMarkers.current, markers)) return;
    shownMarkers.current = markers;
    v.dispatch(setDiagnostics(v.state, toLintDiagnostics(v.state, markers)));
  }, [markers]);

  return <div class="editor" ref={host} />;
}
