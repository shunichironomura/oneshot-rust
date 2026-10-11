import { ArrowLeft } from "lucide-preact";
import { useEffect, useMemo, useState } from "preact/hooks";
import { localize, type Messages } from "../core/i18n.ts";
import { present } from "../core/present.ts";
import type { Problem } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import { type Run, verdictsOf } from "../core/run.ts";
import { shareText, verdictEmoji } from "../core/scoring.ts";
import { shareCard } from "../core/share.ts";
import { useApp } from "./context.ts";
import { DiffView } from "./DiffView.tsx";
import { IconButton } from "./IconButton.tsx";
import { ShareButtons } from "./ShareButtons.tsx";

function runStatus(run: Run, t: Messages): string {
  switch (run.status) {
    case "solved":
      return t.runSolved(run.attempts.length);
    case "failed":
      return t.runFailed;
    case "playing":
      return t.runPlaying;
  }
}

/** Copy and image sharing for a finished run. */
function RunShare({ run, problem }: { run: Run; problem: Problem }) {
  const { locale, t } = useApp();
  const input = useMemo(
    () => ({
      problemId: problem.id,
      verdicts: verdictsOf(run),
      solved: run.status === "solved",
    }),
    [run, problem],
  );
  const card = useMemo(
    () => shareCard({ ...input, problemTitle: localize(problem.title, locale) }, t),
    [input, problem, locale, t],
  );
  return (
    <div class="actions">
      <ShareButtons
        text={shareText(input, t)}
        card={card}
        fileName={`oneshot-rust-${problem.id}`}
      />
    </div>
  );
}

function RunDetail({ run, problem }: { run: Run; problem: Problem }) {
  const { t } = useApp();
  const [selected, setSelected] = useState(run.attempts.length - 1);
  const attempt = run.attempts[selected];
  if (attempt === undefined) return <p>{t.noSubmissions}</p>;
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
              {verdictEmoji(a.judgement.verdict.kind)} #{i + 1} {a.judgement.verdict.kind}
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

export function HistoryPage({ problemId }: { problemId: string }) {
  const { content, storage, locale, t } = useApp();
  const problem = content.problems.find((p) => p.id === problemId);
  const [runs, setRuns] = useState<Run[]>();
  const [openRun, setOpenRun] = useState<string>();
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
  return (
    <section class="history">
      <div class="section-head">
        <IconButton
          icon={ArrowLeft}
          label={localize(problem.title, locale)}
          href={routeHref({ page: "practice", problemId })}
        />
        <h1>
          {t.historyTitle}: {localize(problem.title, locale)}
        </h1>
      </div>
      {runs?.length === 0 && <p>{t.noSubmissions}</p>}
      {runs?.map((run) => (
        <details
          key={run.id}
          open={run.id === openRun}
          onToggle={(e) => {
            if ((e.currentTarget as HTMLDetailsElement).open) setOpenRun(run.id);
          }}
        >
          <summary>
            <span>{verdictsOf(run).map(verdictEmoji).join("")}</span>
            <span>{runStatus(run, t)}</span>
            {run.assisted === true && <span class="badge">{t.assistedChip}</span>}
            <span class="run-date">{dateFormat.format(run.startedAt)}</span>
          </summary>
          {run.id === openRun && <RunDetail run={run} problem={problem} />}
        </details>
      ))}
    </section>
  );
}
