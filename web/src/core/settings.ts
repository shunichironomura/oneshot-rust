import { LOCALES, type Locale } from "./i18n.ts";

/** What the player can change: only the language. Everything else is the same for everyone. */
export interface Settings {
  locale: Locale;
}

export const DEFAULT_SETTINGS: Settings = { locale: "en" };

/** Fills in defaults for settings stored by an older version (or corrupted ones). */
export function normalizeSettings(stored: unknown): Settings {
  if (typeof stored !== "object" || stored === null) return DEFAULT_SETTINGS;
  const s = stored as Partial<Settings>;
  return { locale: LOCALES.find((l) => l === s.locale) ?? DEFAULT_SETTINGS.locale };
}
