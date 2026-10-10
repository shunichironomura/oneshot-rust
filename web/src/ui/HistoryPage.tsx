import { useEffect, useMemo, useState } from "preact/hooks";
import { lineDiff } from "../core/diff.ts";
import { MODE_LABELS } from "../core/mode.ts";
import { present, VERDICT_LABELS } from "../core/present.ts";
import type { Problem } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import { type Run, verdictsOf } from "../core/run.ts";
import { titleFor, verdictEmoji } from "../core/scoring.ts";
import { useApp } from "./context.ts";

const dateFormat = new Intl.DateTimeFormat("ja-JP", { dateStyle: "short", timeStyle: "short" });

function runStatus(run: Run, problem: Problem): string {
  switch (run.status) {
    case "solved":
      return `${run.attempts.length} 回で正解（${titleFor(run.attempts.length, problem.par)}）`;
    case "failed":
      return "未解決で終了";
    case "playing":
      return "挑戦中";
  }
}

function DiffView({ before, after }: { before: string; after: string }) {
  const lines = useMemo(() => lineDiff(before, after), [before, after]);
  return (
    <pre class="diff">
      {lines.map((l, i) => (
        <div key={i} class={`diff-line ${l.kind}`}>
          {l.kind === "added" ? "+ " : l.kind === "removed" ? "- " : "  "}
          {l.text}
        </div>
      ))}
    </pre>
  );
}

function RunDetail({ run, problem }: { run: Run; problem: Problem }) {
  const [selected, setSelected] = useState(run.attempts.length - 1);
  const attempt = run.attempts[selected];
  if (attempt === undefined) return <p>提出がありません。</p>;
  const before = selected === 0 ? problem.template : run.attempts[selected - 1].code;
  const view = present(attempt.judgement, {
    mode: run.mode,
    harnessVisible: problem.judge.harnessVisible,
  });
  return (
    <div class="run-detail">
      <ol class="attempts">
        {run.attempts.map((a, i) => (
          <li key={i}>
            <button type="button" aria-pressed={i === selected} onClick={() => setSelected(i)}>
              {verdictEmoji(a.judgement.verdict.kind)} {i + 1} 回目: {a.judgement.verdict.kind}
            </button>
          </li>
        ))}
      </ol>
      <div class="attempt">
        <h3>
          {selected + 1} 回目の提出 — {VERDICT_LABELS[view.kind]}
        </h3>
        {view.summary !== undefined && <p class="summary">{view.summary}</p>}
        {view.diagnostics.map((d, i) => (
          <pre class="diagnostic" key={i}>
            {d}
          </pre>
        ))}
        <h4>{selected === 0 ? "テンプレートからの変更" : `${selected} 回目からの変更`}</h4>
        <DiffView before={before} after={attempt.code} />
      </div>
    </div>
  );
}

export function HistoryPage({ problemId }: { problemId: string }) {
  const { content, storage } = useApp();
  const problem = content.problems.find((p) => p.id === problemId);
  const [runs, setRuns] = useState<Run[]>();
  const [openRun, setOpenRun] = useState<string>();

  useEffect(() => {
    void storage.runsOf(problemId).then((rs) => {
      setRuns(rs);
      setOpenRun(rs[0]?.id);
    });
  }, [storage, problemId]);

  if (problem === undefined) return <p>問題「{problemId}」は見つかりませんでした。</p>;
  return (
    <section class="history">
      <h1>
        提出履歴: <a href={routeHref({ page: "play", problemId })}>{problem.title}</a>
      </h1>
      {runs?.length === 0 && <p>まだ提出がありません。</p>}
      {runs?.map((run) => (
        <details
          key={run.id}
          open={run.id === openRun}
          onToggle={(e) => {
            if ((e.currentTarget as HTMLDetailsElement).open) setOpenRun(run.id);
          }}
        >
          <summary>
            {dateFormat.format(run.startedAt)} [{MODE_LABELS[run.mode]}]{" "}
            {verdictsOf(run).map(verdictEmoji).join("")} {runStatus(run, problem)}
          </summary>
          {run.id === openRun && <RunDetail run={run} problem={problem} />}
        </details>
      ))}
    </section>
  );
}
