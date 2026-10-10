import { LayoutList, Languages, Settings as SettingsIcon, Trophy } from "lucide-preact";
import content from "virtual:oneshot-content";
import { useCallback, useEffect, useState } from "preact/hooks";
import { type Locale, MESSAGES } from "./core/i18n.ts";
import { parseRoute, type Route, routeHref } from "./core/route.ts";
import { DEFAULT_SETTINGS, type Settings } from "./core/settings.ts";
import { Storage } from "./storage/db.ts";
import { ChallengePage } from "./ui/ChallengePage.tsx";
import { AppContext } from "./ui/context.ts";
import { HistoryPage } from "./ui/HistoryPage.tsx";
import { HomePage } from "./ui/HomePage.tsx";
import { IconButton } from "./ui/IconButton.tsx";
import { PlayPage } from "./ui/PlayPage.tsx";
import { ProblemsPage } from "./ui/ProblemsPage.tsx";
import { SettingsPanel } from "./ui/SettingsPanel.tsx";
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
    case "home":
      return <HomePage />;
    case "problems":
      return <ProblemsPage />;
    case "play":
      return <PlayPage problemId={route.problemId} />;
    case "history":
      return <HistoryPage problemId={route.problemId} />;
    case "challenge":
      return <ChallengePage startRequested={route.start === true} />;
  }
}

const OTHER_LOCALE: Record<Locale, Locale> = { en: "ja", ja: "en" };

export function App() {
  const route = useRoute();
  const toolchain = useToolchain();
  const [storage, setStorage] = useState<Storage>();
  const [settings, setSettings] = useState<Settings>();
  const [storageError, setStorageError] = useState<string>();
  /** The database upgrade waits for other tabs, or another tab needs this one to close. */
  const [storageWait, setStorageWait] = useState<"blocked" | "outdated">();
  const [settingsOpen, setSettingsOpen] = useState(false);
  const openSettings = useCallback(() => setSettingsOpen(true), []);

  useEffect(() => {
    Storage.open({
      onBlocked: () => setStorageWait("blocked"),
      onBlocking: () => setStorageWait("outdated"),
    })
      .then(async (s) => {
        setSettings(await s.loadSettings());
        setStorage(s);
        setStorageWait((w) => (w === "blocked" ? undefined : w));
      })
      .catch((e: unknown) => setStorageError(String(e)));
  }, []);

  const locale = settings?.locale ?? DEFAULT_SETTINGS.locale;
  const t = MESSAGES[locale];
  useEffect(() => {
    document.documentElement.lang = locale;
  }, [locale]);

  const updateSettings = useCallback(
    (next: Settings) => {
      setSettings(next);
      void storage?.saveSettings(next);
    },
    [storage],
  );

  if (storageError !== undefined) return <p class="fatal">{t.storageError(storageError)}</p>;
  if (storageWait === "outdated") {
    return (
      <div class="fatal">
        <p>{t.storageOutdated}</p>
        <button type="button" class="primary" onClick={() => window.location.reload()}>
          {t.reload}
        </button>
      </div>
    );
  }
  if (storageWait === "blocked" && storage === undefined) {
    return <p class="fatal">{t.storageBlocked}</p>;
  }
  if (storage === undefined || settings === undefined) return null;

  const other = OTHER_LOCALE[locale];
  return (
    <AppContext.Provider
      value={{ content, storage, settings, updateSettings, toolchain, locale, t, openSettings }}
    >
      <header class="app-header">
        <a class="logo" href={routeHref({ page: "home" })}>
          <span class="logo-mark" aria-hidden="true">
            ◎
          </span>
          OneShot Rust
        </a>
        <nav>
          <IconButton
            icon={LayoutList}
            label={t.navProblems}
            href={routeHref({ page: "problems" })}
          >
            <span class="nav-text">{t.navProblems}</span>
          </IconButton>
          <IconButton icon={Trophy} label={t.navChallenge} href={routeHref({ page: "challenge" })}>
            <span class="nav-text">{t.navChallenge}</span>
          </IconButton>
          <IconButton
            icon={Languages}
            label={`${t.language}: ${other === "ja" ? "日本語" : "English"}`}
            onClick={() => updateSettings({ ...settings, locale: other })}
          >
            <span class="nav-text">{other === "ja" ? "日本語" : "EN"}</span>
          </IconButton>
          <IconButton icon={SettingsIcon} label={t.navSettings} onClick={openSettings} />
        </nav>
      </header>
      <ToolchainStatus state={toolchain.state} />
      <main>
        <Page route={route} />
      </main>
      <SettingsPanel open={settingsOpen} onClose={() => setSettingsOpen(false)} />
    </AppContext.Provider>
  );
}
