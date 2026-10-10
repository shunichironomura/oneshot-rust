import { createContext } from "preact";
import { useContext } from "preact/hooks";
import type { Content } from "../core/problem.ts";
import type { Settings } from "../core/settings.ts";
import type { Storage } from "../storage/db.ts";
import type { ToolchainState } from "./useToolchain.ts";

export interface AppContextValue {
  content: Content;
  storage: Storage;
  settings: Settings;
  updateSettings: (settings: Settings) => void;
  toolchain: ToolchainState;
}

export const AppContext = createContext<AppContextValue | null>(null);

export function useApp(): AppContextValue {
  const value = useContext(AppContext);
  if (value === null) throw new Error("useApp must be used inside <AppContext.Provider>");
  return value;
}
