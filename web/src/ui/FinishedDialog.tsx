import { BookOpen, Copy, History, X } from "lucide-preact";
import { useState } from "preact/hooks";
import type { Problem } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import { type Best, ceCount, type Run, verdictsOf } from "../core/run.ts";
import { shareText, titleFor, verdictEmoji } from "../core/scoring.ts";
import { useApp } from "./context.ts";
import { Dialog } from "./Dialog.tsx";

interface Props {
  problem: Problem;
  finished: { run: Run; newBest: Best | undefined } | null;
  onClose: () => void;
  /** Opens the model solution tab (not offered inside a challenge). */
  onShowSolution?: () => void;
  /** The main follow-up action (try again, or move on in a challenge). */
  next: { label: string; action: () => void };
}

export function FinishedDialog({ problem, finished, onClose, onShowSolution, next }: Props) {
  const { t } = useApp();
  const [copied, setCopied] = useState(false);
  if (finished === null) return null;
  const { run, newBest } = finished;
  const n = run.attempts.length;
  const solved = run.status === "solved";
  const verdicts = verdictsOf(run);
  const text = shareText(
    { problemId: problem.id, mode: run.mode, par: problem.par, verdicts, solved },
    t,
  );
  const copy = async () => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
  };

  return (
    <Dialog open onClose={onClose} class="finished">
      <button
        type="button"
        class="dialog-close icon-button icon-only"
        aria-label={t.close}
        onClick={onClose}
      >
        <X size={18} aria-hidden="true" />
      </button>
      <h2 class={solved ? "solved" : "failed"}>
        {solved ? `${titleFor(n, problem.par)}!` : t.outOfLives}
      </h2>
      <p class="squares">{verdicts.map(verdictEmoji).join("")}</p>
      {newBest !== undefined && <p class="new-best">{t.newBest}</p>}
      {run.assisted === true && <p class="practice-note">{t.practiceRun}</p>}
      <dl>
        <dt>{t.submissions}</dt>
        <dd>{t.submissionsValue(n, problem.par)}</dd>
        <dt>{t.compileErrors}</dt>
        <dd>{ceCount(run)}</dd>
        <dt>{t.mode}</dt>
        <dd>{t.modes[run.mode]}</dd>
      </dl>
      <div class="actions">
        <button type="button" class="primary" onClick={next.action}>
          {next.label}
        </button>
        <button type="button" class="icon-button" onClick={() => void copy()}>
          <Copy size={16} aria-hidden="true" />
          {copied ? t.copied : t.copyResult}
        </button>
        {onShowSolution !== undefined && (
          <button type="button" class="icon-button" onClick={onShowSolution}>
            <BookOpen size={16} aria-hidden="true" />
            {t.seeSolution}
          </button>
        )}
        <a class="button icon-button" href={routeHref({ page: "history", problemId: problem.id })}>
          <History size={16} aria-hidden="true" />
          {t.reviewSubmissions}
        </a>
      </div>
    </Dialog>
  );
}
