import { ArrowRight, BookOpen, X } from "lucide-preact";
import { type Run, verdictsOf } from "../core/run.ts";
import { titleFor } from "../core/scoring.ts";
import { useApp } from "./context.ts";
import { Dialog } from "./Dialog.tsx";
import { ShotMeter } from "./ShotMeter.tsx";

interface Props {
  finished: { run: Run; beatBest: boolean } | null;
  onClose: () => void;
  onShowSolution: () => void;
  /** What comes next: the next problem of the round, the scorecard, or another try. */
  next: { label: string; action: () => void };
}

/**
 * The result of one problem: the verdicts and the count, what comes next, and the model
 * solution. Sharing is left to the scorecard and the submission history.
 */
export function FinishedDialog({ finished, onClose, onShowSolution, next }: Props) {
  const { t } = useApp();
  if (finished === null) return null;
  const { run, beatBest } = finished;
  const n = run.attempts.length;
  const solved = run.status === "solved";

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
        {solved ? `${titleFor(n) ?? t.solvedHeadline}!` : t.unsolvedHeadline}
      </h2>
      {n > 0 && (
        <p class="finished-shots">
          <ShotMeter shots={verdictsOf(run)} label={t.shareTries(n)} />
          <span class="tries">{t.shareTries(n)}</span>
        </p>
      )}
      {beatBest && <p class="new-best">{t.newBest}</p>}
      {run.assisted === true && <p class="practice-note">{t.assistedRun}</p>}
      <div class="actions">
        <button type="button" class="primary icon-button" onClick={next.action}>
          {next.label}
          <ArrowRight size={16} aria-hidden="true" />
        </button>
        <button type="button" class="icon-button" onClick={onShowSolution}>
          <BookOpen size={16} aria-hidden="true" />
          {t.seeSolution}
        </button>
      </div>
    </Dialog>
  );
}
