import { CircleCheck, CircleDot, PenLine, Play, Send, Target, Trophy } from "lucide-preact";
import { useEffect, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { recommend } from "../core/recommend.ts";
import { routeHref } from "../core/route.ts";
import type { Best } from "../core/run.ts";
import { titleFor } from "../core/scoring.ts";
import { useApp } from "./context.ts";
import { Difficulty, KindBadge } from "./KindBadge.tsx";

const STEP_ICONS = [PenLine, Send, Target];

export function HomePage() {
  const { content, storage, settings, locale, t } = useApp();
  const [bests, setBests] = useState<Best[]>([]);
  const [inProgress, setInProgress] = useState<Set<string>>(new Set());

  useEffect(() => {
    void storage.allBests().then(setBests);
    void storage.problemsInProgress().then(setInProgress);
  }, [storage]);

  const modeBests = new Map(
    bests.filter((b) => b.mode === settings.mode).map((b) => [b.problemId, b]),
  );
  const rec = recommend(
    content.problems.map((p) => p.id),
    new Set(modeBests.keys()),
    inProgress,
  );
  const recommended =
    rec.kind === "all-solved" ? undefined : content.problems.find((p) => p.id === rec.problemId);
  const modeName = t.modes[settings.mode];

  return (
    <section class="home">
      <div class="hero">
        <h1>{t.heroTitle}</h1>
        <p class="lead">{t.heroLead}</p>
        <ol class="steps">
          {t.howToPlay.map((step, i) => {
            const Icon = STEP_ICONS[i];
            return (
              <li key={i}>
                <Icon size={20} aria-hidden="true" />
                <span>{step}</span>
              </li>
            );
          })}
        </ol>
        <div class="cta">
          {recommended !== undefined ? (
            <a
              class="button primary large"
              href={routeHref({ page: "play", problemId: recommended.id })}
            >
              <Play size={18} aria-hidden="true" />
              {rec.kind === "continue" ? t.continueProblem : t.nextProblem}:{" "}
              {localize(recommended.title, locale)}
            </a>
          ) : (
            <p class="all-solved">{t.allSolved(modeName)}</p>
          )}
          <a class="button large" href={routeHref({ page: "challenge" })}>
            <Trophy size={18} aria-hidden="true" />
            {t.tryChallenge}
          </a>
        </div>
      </div>

      <div class="section-head">
        <h2>{t.problemsHeading}</h2>
        <a class="chip" href={routeHref({ page: "settings" })} title={t.modeHeading}>
          {t.modeHeading}: {modeName}
        </a>
      </div>
      <table class="problem-table">
        <thead>
          <tr>
            <th aria-hidden="true" />
            <th>{t.columnProblem}</th>
            <th>{t.columnDifficulty}</th>
            <th>Par</th>
            <th>{t.columnBest(modeName)}</th>
          </tr>
        </thead>
        <tbody>
          {content.problems.map((p) => {
            const best = modeBests.get(p.id);
            return (
              <tr key={p.id}>
                <td class="status">
                  {best !== undefined ? (
                    <CircleCheck size={18} class="solved" aria-label="solved" />
                  ) : inProgress.has(p.id) ? (
                    <CircleDot size={18} class="in-progress" aria-label="in progress" />
                  ) : null}
                </td>
                <td>
                  <a class="problem-link" href={routeHref({ page: "play", problemId: p.id })}>
                    {localize(p.title, locale)}
                  </a>
                  <KindBadge kind={p.kind} />
                </td>
                <td>
                  <Difficulty n={p.difficulty} />
                </td>
                <td class="num">{p.par}</td>
                <td class="best">
                  {best === undefined ? (
                    <span class="dim">—</span>
                  ) : (
                    <span title={t.bestTooltip(best.ceCount)}>
                      {best.submissions} · {titleFor(best.submissions, p.par)}
                    </span>
                  )}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </section>
  );
}
