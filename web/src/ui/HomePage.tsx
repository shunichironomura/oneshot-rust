import { routeHref } from "../core/route.ts";
import { useApp } from "./context.ts";
import { HeroDemo } from "./HeroDemo.tsx";
import { RoundLauncher } from "./RoundLauncher.tsx";

const AUTHOR = { name: "Shunichiro Nomura", url: "https://github.com/shunichironomura" };
// The README lists the licenses of the code, the problems and the toolchain.
const REPOSITORY_URL = "https://github.com/shunichironomura/oneshot-rust#readme";

function openRound(): void {
  window.location.hash = routeHref({ page: "round" });
}

export function HomePage() {
  const { t } = useApp();
  return (
    <section class="home">
      <div class="hero">
        <div class="hero-text">
          <h1>{t.heroTitle}</h1>
          <p class="tagline">{t.heroTagline}</p>
          <RoundLauncher onStarted={openRound} />
        </div>
        <HeroDemo />
      </div>

      <footer class="credit">
        <p>
          {t.madeBy} <a href={AUTHOR.url}>{AUTHOR.name}</a> ·{" "}
          <a href={REPOSITORY_URL}>{t.sourceAndLicenses}</a>
        </p>
        <p class="ai-notice">{t.aiNotice}</p>
      </footer>
    </section>
  );
}
