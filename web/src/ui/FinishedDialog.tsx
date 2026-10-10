import { useState } from "preact/hooks";
import { MODE_LABELS } from "../core/mode.ts";
import type { Problem } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import { type Best, ceCount, type Run, verdictsOf } from "../core/run.ts";
import { shareText, titleFor, verdictEmoji } from "../core/scoring.ts";
import { Dialog } from "./Dialog.tsx";

interface Props {
  problem: Problem;
  finished: { run: Run; newBest: Best | undefined } | null;
  onClose: () => void;
  onRestart: () => void;
}

export function FinishedDialog({ problem, finished, onClose, onRestart }: Props) {
  const [copied, setCopied] = useState(false);
  if (finished === null) return null;
  const { run, newBest } = finished;
  const n = run.attempts.length;
  const solved = run.status === "solved";
  const text = shareText({
    problemId: problem.id,
    mode: run.mode,
    par: problem.par,
    verdicts: verdictsOf(run),
    solved,
  });
  const copy = async () => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
  };

  return (
    <Dialog open onClose={onClose} class="finished">
      <h2>{solved ? `${titleFor(n, problem.par)}!` : "残機がなくなりました"}</h2>
      <p class="squares">{verdictsOf(run).map(verdictEmoji).join("")}</p>
      <dl>
        <dt>提出回数</dt>
        <dd>
          {n} 回（Par {problem.par}）
        </dd>
        <dt>うち CE</dt>
        <dd>{ceCount(run)} 回</dd>
        <dt>モード</dt>
        <dd>{MODE_LABELS[run.mode]}</dd>
      </dl>
      {newBest !== undefined && <p class="new-best">自己ベスト更新！</p>}
      <div class="actions">
        <button type="button" onClick={() => void copy()}>
          {copied ? "コピーしました" : "結果をコピー"}
        </button>
        <a class="button" href={routeHref({ page: "history", problemId: problem.id })}>
          振り返る（提出履歴と差分）
        </a>
        <button type="button" onClick={onRestart}>
          最初から挑戦
        </button>
        <button type="button" onClick={onClose}>
          閉じる
        </button>
      </div>
    </Dialog>
  );
}
