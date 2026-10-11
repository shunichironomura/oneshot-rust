import { createContext } from "preact";
import { useContext } from "preact/hooks";
import type { Locale, Messages } from "../core/i18n.ts";
import type { Content } from "../core/problem.ts";
import type { Settings } from "../core/settings.ts";
import type { Storage } from "../storage/db.ts";
import type { Toolchain } from "./useToolchain.ts";

export interface AppContextValue {
  content: Content;
  storage: Storage;
  settings: Settings;
  updateSettings: (settings: Settings) => void;
  toolchain: Toolchain;
  locale: Locale;
  /** UI messages in the current locale. */
  t: Messages;
}

export const AppContext = createContext<AppContextValue | null>(null);

export function useApp(): AppContextValue {
  const value = useContext(AppContext);
  if (value === null) throw new Error("useApp must be used inside <AppContext.Provider>");
  return value;
}
