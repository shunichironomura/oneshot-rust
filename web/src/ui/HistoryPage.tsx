import { ArrowLeft, CircleCheck, CircleDot, CircleX, Play } from "lucide-preact";
import { useEffect, useMemo, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { present } from "../core/present.ts";
import type { Problem } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import { type Run, verdictsOf } from "../core/run.ts";
import { shareText } from "../core/scoring.ts";
import { shareCard } from "../core/share.ts";
import { hasFinishedRun } from "../core/solution.ts";
import { useApp } from "./context.ts";
import { DiffView } from "./DiffView.tsx";
import { DifficultyBadge } from "./DifficultyBadge.tsx";
import { IconButton } from "./IconButton.tsx";
import { ModelSolution } from "./ModelSolution.tsx";
import { ShareButton } from "./ShareButton.tsx";
import { ShotMeter } from "./ShotMeter.tsx";

function RunStatus({ run }: { run: Run }) {
  const { t } = useApp();
  switch (run.status) {
    case "solved":
      return (
        <CircleCheck
          size={16}
          class="status-icon solved"
          role="img"
          aria-label={t.solvedHeadline}
        />
      );
    case "failed":
      return (
        <CircleX size={16} class="status-icon failed" role="img" aria-label={t.unsolvedHeadline} />
      );
    case "playing":
      return (
        <CircleDot size={16} class="status-icon in-progress" role="img" aria-label={t.runPlaying} />
      );
  }
}

/** Sharing for a finished run. */
function RunShare({ run, problem }: { run: Run; problem: Problem }) {
  const { locale, t } = useApp();
  const input = useMemo(
    () => ({ problemId: problem.id, verdicts: verdictsOf(run), solved: run.status === "solved" }),
    [run, problem],
  );
  const card = useMemo(
    () => shareCard({ ...input, problemTitle: localize(problem.title, locale) }, t),
    [input, problem, locale, t],
  );
  return (
    <div class="actions">
      <ShareButton text={shareText(input, t)} card={card} fileName={`oneshot-rust-${problem.id}`} />
    </div>
  );
}

function RunDetail({ run, problem }: { run: Run; problem: Problem }) {
  const { t } = useApp();
  const [selected, setSelected] = useState(run.attempts.length - 1);
  const attempt = run.attempts[selected];
  // Given up without a submission: nothing to show.
  if (attempt === undefined) return null;
  const before = selected === 0 ? problem.template : run.attempts[selected - 1].code;
  const view = present(attempt.judgement, {
    harnessVisible: problem.judge.harnessVisible,
    messages: t,
  });
  return (
    <div class="run-detail">
      {run.status !== "playing" && <RunShare run={run} problem={problem} />}
      <ol class="attempts">
        {run.attempts.map((a, i) => (
          <li key={i}>
            <button type="button" aria-pressed={i === selected} onClick={() => setSelected(i)}>
              <span class={`verdict ${a.judgement.verdict.kind}`}>{a.judgement.verdict.kind}</span>#
              {i + 1}
            </button>
          </li>
        ))}
      </ol>
      <div class="attempt">
        <h3>{t.attemptHeading(selected + 1, t.verdicts[view.kind])}</h3>
        {view.summary !== undefined && <p class="summary">{view.summary}</p>}
        {view.diagnostics.map((d, i) => (
          <pre class="diagnostic" key={i}>
            {d}
          </pre>
        ))}
        <h4>{selected === 0 ? t.changesFromTemplate : t.changesFrom(selected)}</h4>
        <DiffView before={before} after={attempt.code} />
      </div>
    </div>
  );
}

/** A problem's record: its runs with every submission, the model solution, and practice. */
export function HistoryPage({ problemId }: { problemId: string }) {
  const { content, storage, locale, t } = useApp();
  const problem = content.problems.find((p) => p.id === problemId);
  const [runs, setRuns] = useState<Run[]>();
  const [openRun, setOpenRun] = useState<string>();
  const [solutionOpen, setSolutionOpen] = useState(false);
  const dateFormat = useMemo(
    () => new Intl.DateTimeFormat(locale, { dateStyle: "medium", timeStyle: "short" }),
    [locale],
  );

  useEffect(() => {
    void storage.runsOf(problemId).then((rs) => {
      setRuns(rs);
      setOpenRun(rs[0]?.id);
    });
  }, [storage, problemId]);

  if (problem === undefined) return <p>{t.problemNotFound(problemId)}</p>;
  // The player's latest code, to compare with the model solution.
  const latestCode = runs?.find((r) => r.attempts.length > 0)?.attempts.at(-1)?.code;
  return (
    <section class="history">
      <div class="section-head">
        <IconButton
          icon={ArrowLeft}
          label={t.backToRecords}
          href={routeHref({ page: "problems" })}
        />
        <h1>{localize(problem.title, locale)}</h1>
        <DifficultyBadge difficulty={problem.difficulty} />
        <a class="button primary" href={routeHref({ page: "practice", problemId })}>
          <Play size={16} aria-hidden="true" />
          {t.practice}
        </a>
      </div>
      {runs !== undefined && hasFinishedRun(runs) && (
        <details
          class="model-solution"
          onToggle={(e) => {
            const open = (e.currentTarget as HTMLDetailsElement).open;
            setSolutionOpen(open);
            if (open) void storage.saveReveal(problemId, Date.now());
          }}
        >
          <summary>{t.seeSolution}</summary>
          {solutionOpen && (
            <ModelSolution problem={problem} code={latestCode ?? problem.template} />
          )}
        </details>
      )}
      {runs?.length === 0 && <p class="dim">{t.noSubmissions}</p>}
      {runs?.map((run) => (
        <details
          key={run.id}
          class="run"
          open={run.id === openRun}
          onToggle={(e) => {
            if ((e.currentTarget as HTMLDetailsElement).open) setOpenRun(run.id);
          }}
        >
          <summary>
            <RunStatus run={run} />
            {run.attempts.length > 0 && (
              <ShotMeter shots={verdictsOf(run)} label={t.shareTries(run.attempts.length)} />
            )}
            <span class="shot-count">{run.attempts.length}</span>
            {run.assisted === true && <span class="chip">{t.assistedChip}</span>}
            <span class="run-date">{dateFormat.format(run.startedAt)}</span>
          </summary>
          {run.id === openRun && <RunDetail run={run} problem={problem} />}
        </details>
      ))}
    </section>
  );
}
