import { LOCALES, type Locale } from "./i18n.ts";
import { MODES, type Mode } from "./mode.ts";

export interface EditorSettings {
  syntaxHighlight: boolean;
  closeBrackets: boolean;
  autoIndent: boolean;
  fontSize: number;
  fontFamily: string;
}

export interface Settings {
  locale: Locale;
  mode: Mode;
  /** Maximum submissions per run (残機制), or `null` for unlimited. */
  lives: number | null;
  /** Ask for confirmation before each submission. */
  confirmSubmit: boolean;
  editor: EditorSettings;
}

export const LIVES_OPTIONS: readonly (number | null)[] = [null, 3, 5];

export const FONT_FAMILIES: readonly string[] = [
  "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace",
  "'Courier New', Courier, monospace",
  "'Comic Sans MS', 'Comic Sans', cursive",
];

export const DEFAULT_SETTINGS: Settings = {
  locale: "en",
  mode: "standard",
  lives: null,
  confirmSubmit: true,
  editor: {
    syntaxHighlight: true,
    closeBrackets: true,
    autoIndent: true,
    fontSize: 14,
    fontFamily: FONT_FAMILIES[0],
  },
};

/** "Notepad mode": every editor aid turned off. */
export function notepadEditor(editor: EditorSettings): EditorSettings {
  return { ...editor, syntaxHighlight: false, closeBrackets: false, autoIndent: false };
}

/** Fills in defaults for settings stored by an older version (or corrupted ones). */
export function normalizeSettings(stored: unknown): Settings {
  if (typeof stored !== "object" || stored === null) return DEFAULT_SETTINGS;
  const s = stored as Partial<Settings>;
  const e: Partial<EditorSettings> =
    typeof s.editor === "object" && s.editor !== null ? s.editor : {};
  const d = DEFAULT_SETTINGS;
  const bool = (v: unknown, fallback: boolean) => (typeof v === "boolean" ? v : fallback);
  return {
    locale: LOCALES.find((l) => l === s.locale) ?? d.locale,
    mode: MODES.find((m) => m === s.mode) ?? d.mode,
    lives: LIVES_OPTIONS.includes(s.lives ?? null) ? (s.lives ?? null) : d.lives,
    confirmSubmit: bool(s.confirmSubmit, d.confirmSubmit),
    editor: {
      syntaxHighlight: bool(e.syntaxHighlight, d.editor.syntaxHighlight),
      closeBrackets: bool(e.closeBrackets, d.editor.closeBrackets),
      autoIndent: bool(e.autoIndent, d.editor.autoIndent),
      fontSize:
        typeof e.fontSize === "number" && e.fontSize >= 10 && e.fontSize <= 28
          ? e.fontSize
          : d.editor.fontSize,
      fontFamily:
        typeof e.fontFamily === "string" && e.fontFamily !== ""
          ? e.fontFamily
          : d.editor.fontFamily,
    },
  };
}
