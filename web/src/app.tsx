import content from "virtual:oneshot-content";
import { useCallback, useEffect, useState } from "preact/hooks";
import { parseRoute, type Route, routeHref } from "./core/route.ts";
import type { Settings } from "./core/settings.ts";
import { Storage } from "./storage/db.ts";
import { AppContext } from "./ui/context.ts";
import { HistoryPage } from "./ui/HistoryPage.tsx";
import { PlayPage } from "./ui/PlayPage.tsx";
import { ProblemList } from "./ui/ProblemList.tsx";
import { SettingsPage } from "./ui/SettingsPage.tsx";
import { ToolchainStatus } from "./ui/ToolchainStatus.tsx";
import { useToolchain } from "./ui/useToolchain.ts";

function useRoute(): Route {
  const [route, setRoute] = useState(() => parseRoute(window.location.hash));
  useEffect(() => {
    const onChange = () => setRoute(parseRoute(window.location.hash));
    window.addEventListener("hashchange", onChange);
    return () => window.removeEventListener("hashchange", onChange);
  }, []);
  return route;
}

function Page({ route }: { route: Route }) {
  switch (route.page) {
    case "list":
      return <ProblemList />;
    case "play":
      return <PlayPage problemId={route.problemId} />;
    case "history":
      return <HistoryPage problemId={route.problemId} />;
    case "settings":
      return <SettingsPage />;
  }
}

export function App() {
  const route = useRoute();
  const toolchain = useToolchain();
  const [storage, setStorage] = useState<Storage>();
  const [settings, setSettings] = useState<Settings>();
  const [storageError, setStorageError] = useState<string>();

  useEffect(() => {
    Storage.open()
      .then(async (s) => {
        setSettings(await s.loadSettings());
        setStorage(s);
      })
      .catch((e: unknown) => setStorageError(String(e)));
  }, []);

  const updateSettings = useCallback(
    (next: Settings) => {
      setSettings(next);
      void storage?.saveSettings(next);
    },
    [storage],
  );

  if (storageError !== undefined) {
    return <p class="fatal">保存領域（IndexedDB）を開けませんでした: {storageError}</p>;
  }
  if (storage === undefined || settings === undefined) return null;

  return (
    <AppContext.Provider value={{ content, storage, settings, updateSettings, toolchain }}>
      <header class="app-header">
        <a class="logo" href={routeHref({ page: "list" })}>
          OneShot Rust <span aria-hidden="true">🦀</span>
        </a>
        <span class="tagline">LSP なし、なるべく少ないコンパイル回数で。</span>
        <nav>
          <a href={routeHref({ page: "list" })}>問題一覧</a>
          <a href={routeHref({ page: "settings" })}>設定</a>
        </nav>
      </header>
      <ToolchainStatus state={toolchain} />
      <main>
        <Page route={route} />
      </main>
    </AppContext.Provider>
  );
}
