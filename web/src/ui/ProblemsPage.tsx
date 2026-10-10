import { CircleCheck, CircleDot } from "lucide-preact";
import { useEffect, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { routeHref } from "../core/route.ts";
import { type Best, bestVerdicts, type Run } from "../core/run.ts";
import { titleFor } from "../core/scoring.ts";
import { useApp } from "./context.ts";
import { KindBadge } from "./KindBadge.tsx";
import { ParMeter } from "./ParMeter.tsx";

export function ProblemsPage() {
  const { content, storage, settings, locale, t, openSettings } = useApp();
  const [bests, setBests] = useState<Best[]>([]);
  const [bestRuns, setBestRuns] = useState<Map<string, Run>>(new Map());
  const [inProgress, setInProgress] = useState<Set<string>>(new Set());

  useEffect(() => {
    void storage.allBests().then(async (all) => {
      setBests(all);
      setBestRuns(await storage.runsById(all.map((b) => b.runId)));
    });
    void storage.problemsInProgress().then(setInProgress);
  }, [storage]);

  const modeBests = new Map(
    bests.filter((b) => b.mode === settings.mode).map((b) => [b.problemId, b]),
  );
  const modeName = t.modes[settings.mode];

  return (
    <section class="problems">
      <div class="section-head">
        <h1>{t.problemsHeading}</h1>
        <button type="button" class="chip" onClick={openSettings} title={t.modeHeading}>
          {t.modeHeading}: {modeName}
        </button>
      </div>
      <table class="problem-table">
        <thead>
          <tr>
            <th aria-hidden="true" />
            <th>{t.columnProblem}</th>
            <th title={t.parTooltip(2)}>Par</th>
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
                  <ParMeter par={p.par} label={t.parTooltip(p.par)} />
                </td>
                <td class="best">
                  {best === undefined ? (
                    <span class="dim">—</span>
                  ) : (
                    <span class="best-record" title={t.bestTooltip(best.ceCount)}>
                      <ParMeter
                        par={p.par}
                        shots={
                          bestVerdicts(best, bestRuns.get(best.runId)) ??
                          Array.from({ length: best.submissions }, () => "done" as const)
                        }
                        label={`${best.submissions}`}
                      />
                      {titleFor(best.submissions, p.par)}
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
