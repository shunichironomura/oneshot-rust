import { ArrowRight, Play, Trophy } from "lucide-preact";
import { useEffect, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { recommend } from "../core/recommend.ts";
import { routeHref } from "../core/route.ts";
import { useApp } from "./context.ts";
import { HeroDemo } from "./HeroDemo.tsx";
import { ParChip } from "./ParMeter.tsx";

export function HomePage() {
  const { content, storage, settings, locale, t } = useApp();
  const [solved, setSolved] = useState<Set<string>>(new Set());
  const [inProgress, setInProgress] = useState<Set<string>>(new Set());
  const [challengeActive, setChallengeActive] = useState(false);

  useEffect(() => {
    void storage
      .allBests()
      .then((bests) =>
        setSolved(new Set(bests.filter((b) => b.mode === settings.mode).map((b) => b.problemId))),
      );
    void storage.problemsInProgress().then(setInProgress);
    void storage
      .challenges()
      .then((all) => setChallengeActive(all.some((c) => c.finishedAt === undefined)));
  }, [storage, settings.mode]);

  const rec = recommend(
    content.problems.map((p) => p.id),
    solved,
    inProgress,
  );
  const next =
    rec.kind === "all-solved"
      ? content.problems[0]
      : content.problems.find((p) => p.id === rec.problemId);

  return (
    <section class="home">
      <div class="hero">
        <div class="hero-text">
          <h1>{t.heroTitle}</h1>
          <p class="tagline">{t.heroTagline}</p>
        </div>
        <HeroDemo />
      </div>

      <div class="choices">
        {next !== undefined && (
          <div class="choice">
            <h2>
              <Play size={20} aria-hidden="true" /> {t.playCardTitle}
            </h2>
            <p class="choice-next">
              <span>
                {rec.kind === "continue"
                  ? t.playCardContinue(localize(next.title, locale))
                  : rec.kind === "start"
                    ? t.playCardStart(localize(next.title, locale))
                    : t.playCardAllSolved(t.modes[settings.mode])}
              </span>
              <ParChip par={next.par} label={t.parTooltip(next.par)} />
            </p>
            <div class="choice-actions">
              <a
                class="button primary large"
                href={routeHref({ page: "play", problemId: next.id })}
              >
                {rec.kind === "continue" ? t.continue : t.start}
                <ArrowRight size={18} aria-hidden="true" />
              </a>
              <a class="subtle-link" href={routeHref({ page: "problems" })}>
                {t.browseProblems(content.problems.length)}
              </a>
            </div>
          </div>
        )}
        <div class="choice">
          <h2>
            <Trophy size={20} aria-hidden="true" /> {t.challengeTitle}
          </h2>
          <p class="choice-next">{t.challengeCardDescription}</p>
          <div class="choice-actions">
            <a class="button large" href={routeHref({ page: "challenge", start: true })}>
              {challengeActive ? t.challengeCardContinue : t.challengeCardStart}
              <ArrowRight size={18} aria-hidden="true" />
            </a>
          </div>
        </div>
      </div>
    </section>
  );
}
