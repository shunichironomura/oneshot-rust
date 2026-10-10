import { ArrowRight, BookOpen, Copy, History, Share2, X } from "lucide-preact";
import { useEffect, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import type { Problem } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import { type Best, ceCount, type Run, verdictsOf } from "../core/run.ts";
import { shareText, titleFor, verdictEmoji } from "../core/scoring.ts";
import { shareCard } from "../core/share.ts";
import { useApp } from "./context.ts";
import { Dialog } from "./Dialog.tsx";
import { renderShareImage, type ShareOutcome, shareImage } from "./shareImage.ts";

interface Props {
  problem: Problem;
  finished: { run: Run; newBest: Best | undefined } | null;
  onClose: () => void;
  /** Opens the model solution tab (not offered inside a challenge). */
  onShowSolution?: () => void;
  /** The main follow-up action (try again, or move on in a challenge). */
  next: { label: string; action: () => void };
  /** Link to the next problem in the list (free play only). */
  nextProblemHref?: string;
}

export function FinishedDialog({
  problem,
  finished,
  onClose,
  onShowSolution,
  next,
  nextProblemHref,
}: Props) {
  const { locale, t } = useApp();
  const [copied, setCopied] = useState(false);
  // Rendered when the dialog opens, so that sharing can start right from the click.
  const [image, setImage] = useState<File>();
  const [shared, setShared] = useState<ShareOutcome>();

  useEffect(() => {
    setImage(undefined);
    setShared(undefined);
    if (finished === null) return;
    const { run } = finished;
    const card = shareCard(
      {
        problemId: problem.id,
        problemTitle: localize(problem.title, locale),
        mode: run.mode,
        par: problem.par,
        verdicts: verdictsOf(run),
        solved: run.status === "solved",
      },
      t,
    );
    let current = true;
    renderShareImage(card)
      .then((blob) => {
        const file = new File([blob], `oneshot-rust-${problem.id}.png`, { type: "image/png" });
        if (current) setImage(file);
      })
      .catch((e: unknown) => console.error("Could not render the share image", e));
    return () => {
      current = false;
    };
  }, [finished, problem, locale, t]);

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
  const share = async () => {
    if (image !== undefined) setShared(await shareImage(image, text));
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
        {nextProblemHref !== undefined && solved && (
          <a class="button primary icon-button" href={nextProblemHref}>
            {t.nextProblem}
            <ArrowRight size={16} aria-hidden="true" />
          </a>
        )}
        <button
          type="button"
          class={nextProblemHref !== undefined && solved ? undefined : "primary"}
          onClick={next.action}
        >
          {next.label}
        </button>
        <button type="button" class="icon-button" onClick={() => void copy()}>
          <Copy size={16} aria-hidden="true" />
          {copied ? t.copied : t.copyResult}
        </button>
        <button
          type="button"
          class="icon-button"
          onClick={() => void share()}
          disabled={image === undefined}
        >
          <Share2 size={16} aria-hidden="true" />
          {shared === "downloaded" ? t.imageSaved : t.shareImage}
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
        {nextProblemHref !== undefined && !solved && (
          <a class="button icon-button" href={nextProblemHref}>
            {t.nextProblem}
            <ArrowRight size={16} aria-hidden="true" />
          </a>
        )}
      </div>
    </Dialog>
  );
}
