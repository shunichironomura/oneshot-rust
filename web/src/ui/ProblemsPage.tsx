import { CircleCheck, CircleDot } from "lucide-preact";
import { useEffect, useMemo, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { type Round, roundSummary, seenProblemIds, totalSubmissions } from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import { type Best, bestsByProblem, bestVerdicts, type Run } from "../core/run.ts";
import { useApp } from "./context.ts";
import { DifficultyBadge } from "./DifficultyBadge.tsx";
import { RoundProgress } from "./RoundProgress.tsx";
import { ShotMeter } from "./ShotMeter.tsx";

function BestRecord({ best, run }: { best: Best; run: Run | undefined }) {
  const { t } = useApp();
  return (
    <span class="best-record" title={t.bestTooltip(best.ceCount)}>
      <ShotMeter
        shots={
          bestVerdicts(best, run) ?? Array.from({ length: best.submissions }, () => "done" as const)
        }
        label={t.shareTries(best.submissions)}
      />
      <span class="shot-count">{best.submissions}</span>
    </span>
  );
}

/**
 * Records: past rounds, and the best of every problem met so far. A problem opens its
 * history, from where it can be practiced. Problems no round has shown yet are left out,
 * unless they were played anyway.
 */
export function ProblemsPage() {
  const { content, storage, locale, t } = useApp();
  const [bests, setBests] = useState<Map<string, Best>>(new Map());
  const [bestRuns, setBestRuns] = useState<Map<string, Run>>(new Map());
  const [inProgress, setInProgress] = useState<Set<string>>(new Set());
  const [rounds, setRounds] = useState<Round[]>([]);
  /** Problems shown in any round, including the one being played. */
  const [seen, setSeen] = useState<Set<string>>(new Set());
  const dateFormat = useMemo(
    () => new Intl.DateTimeFormat(locale, { dateStyle: "medium", timeStyle: "short" }),
    [locale],
  );

  useEffect(() => {
    void storage.allBests().then(async (all) => {
      const byProblem = bestsByProblem(all);
      setBests(byProblem);
      setBestRuns(await storage.runsById([...byProblem.values()].map((b) => b.runId)));
    });
    void storage.problemsInProgress().then(setInProgress);
    void storage.rounds().then((all) => {
      setSeen(seenProblemIds(all));
      setRounds(all.filter((r) => r.finishedAt !== undefined));
    });
  }, [storage]);

  const problems = content.problems.filter(
    (p) => seen.has(p.id) || bests.has(p.id) || inProgress.has(p.id),
  );

  return (
    <section class="problems">
      <h1>{t.recordsHeading}</h1>

      <h2>{t.roundsHeading}</h2>
      {rounds.length === 0 ? (
        <p class="dim">{t.noRounds}</p>
      ) : (
        <ul class="round-history">
          {rounds.map((r) => (
            <li key={r.id}>
              <a href={routeHref({ page: "scorecard", roundId: r.id })}>
                <RoundProgress round={r} label={roundSummary(r, t)} />
                <strong class="round-total">{totalSubmissions(r)}</strong>
                <span class="run-date">{dateFormat.format(r.startedAt)}</span>
              </a>
            </li>
          ))}
        </ul>
      )}

      <h2>{t.problemsHeading}</h2>
      {problems.length === 0 ? (
        <p class="dim">{t.noProblemsSeen}</p>
      ) : (
        <table class="problem-table">
          <tbody>
            {problems.map((p) => {
              const best = bests.get(p.id);
              return (
                <tr key={p.id}>
                  <td class="status">
                    {best !== undefined ? (
                      <CircleCheck size={18} class="solved" aria-label={t.solvedHeadline} />
                    ) : inProgress.has(p.id) ? (
                      <CircleDot size={18} class="in-progress" aria-label={t.runPlaying} />
                    ) : null}
                  </td>
                  <td>
                    <a class="problem-link" href={routeHref({ page: "history", problemId: p.id })}>
                      {localize(p.title, locale)}
                    </a>
                  </td>
                  <td class="difficulty">
                    <DifficultyBadge difficulty={p.difficulty} />
                  </td>
                  <td class="best">
                    {best === undefined ? (
                      <span class="dim">—</span>
                    ) : (
                      <BestRecord best={best} run={bestRuns.get(best.runId)} />
                    )}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      )}
    </section>
  );
}
