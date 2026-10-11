import { LOCALES, type Locale } from "./i18n.ts";
import { MODES, type Mode } from "./mode.ts";
import { DIFFICULTIES, type Difficulty } from "./problem.ts";

/** Defaults for the next round (and for practice). A round keeps the rules it started with. */
export interface Settings {
  locale: Locale;
  /** The course of the next round. */
  course: Difficulty;
  /** The feedback mode of the next round and of practice. */
  mode: Mode;
}

export const DEFAULT_SETTINGS: Settings = {
  locale: "en",
  course: 1,
  mode: "standard",
};

/** Fills in defaults for settings stored by an older version (or corrupted ones). */
export function normalizeSettings(stored: unknown): Settings {
  if (typeof stored !== "object" || stored === null) return DEFAULT_SETTINGS;
  const s = stored as Partial<Settings>;
  const d = DEFAULT_SETTINGS;
  return {
    locale: LOCALES.find((l) => l === s.locale) ?? d.locale,
    course: DIFFICULTIES.find((c) => c === s.course) ?? d.course,
    mode: MODES.find((m) => m === s.mode) ?? d.mode,
  };
}
