import { LOCALES, type Locale } from "./i18n.ts";
import { DIFFICULTIES, type Difficulty } from "./problem.ts";

/** Defaults for the next round. A round keeps the course it started with. */
export interface Settings {
  locale: Locale;
  /** The course of the next round. */
  course: Difficulty;
}

export const DEFAULT_SETTINGS: Settings = {
  locale: "en",
  course: 1,
};

/** Fills in defaults for settings stored by an older version (or corrupted ones). */
export function normalizeSettings(stored: unknown): Settings {
  if (typeof stored !== "object" || stored === null) return DEFAULT_SETTINGS;
  const s = stored as Partial<Settings>;
  const d = DEFAULT_SETTINGS;
  return {
    locale: LOCALES.find((l) => l === s.locale) ?? d.locale,
    course: DIFFICULTIES.find((c) => c === s.course) ?? d.course,
  };
}
