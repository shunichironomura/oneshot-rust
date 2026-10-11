import { CircleCheck, CircleDot } from "lucide-preact";
import { useEffect, useMemo, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { courseName, type Round, roundSummary, seenProblemIds } from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import { type Best, bestsByProblem, bestVerdicts, type Run } from "../core/run.ts";
import { useApp } from "./context.ts";
import { KindBadge } from "./KindBadge.tsx";
import { DifficultyBadge, ModeBadge } from "./LevelBadges.tsx";
import { ShotMeter } from "./ShotMeter.tsx";

function BestRecord({ best, run }: { best: Best; run: Run | undefined }) {
  const { t } = useApp();
  return (
    <span class="best-record" title={t.bestTooltip(best.ceCount)}>
      <ShotMeter
        shots={
          bestVerdicts(best, run) ?? Array.from({ length: best.submissions }, () => "done" as const)
        }
        label={`${best.submissions}`}
      />
      {best.submissions}
      <ModeBadge mode={best.mode} />
    </span>
  );
}

/**
 * Records: the best of every problem met so far (each opens as practice) and past rounds.
 * Problems no round has shown yet are left out, unless they were played anyway.
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
      <h2>{t.problemsHeading}</h2>
      {problems.length === 0 ? (
        <p class="dim">{t.noProblemsSeen}</p>
      ) : (
        <>
          <p class="hint">{t.practiceHint}</p>
          <table class="problem-table">
            <thead>
              <tr>
                <th aria-hidden="true" />
                <th>{t.columnProblem}</th>
                <th>{t.columnDifficulty}</th>
                <th>{t.columnBest}</th>
              </tr>
            </thead>
            <tbody>
              {problems.map((p) => {
                const best = bests.get(p.id);
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
                      <a
                        class="problem-link"
                        href={routeHref({ page: "practice", problemId: p.id })}
                      >
                        {localize(p.title, locale)}
                      </a>
                      <KindBadge kind={p.kind} />
                    </td>
                    <td>
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
        </>
      )}

      <h2>{t.roundsHeading}</h2>
      {rounds.length === 0 ? (
        <p class="dim">{t.noRounds}</p>
      ) : (
        <ul class="round-history">
          {rounds.map((r) => (
            <li key={r.id}>
              <a class="run-date" href={routeHref({ page: "scorecard", roundId: r.id })}>
                {dateFormat.format(r.startedAt)}
              </a>
              {r.course === undefined ? (
                <span>{courseName(r.course, t)}</span>
              ) : (
                <DifficultyBadge difficulty={r.course} />
              )}
              <ModeBadge mode={r.mode} />
              <span>{roundSummary(r, t)}</span>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
