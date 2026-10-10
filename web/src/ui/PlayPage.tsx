import { ArrowLeft, Flag, Heart, History, Play, RotateCcw, SkipForward } from "lucide-preact";
import { useMemo, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { type EditorMarker, present } from "../core/present.ts";
import type { Problem } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import { remainingLives, type Run } from "../core/run.ts";
import { useApp } from "./context.ts";
import { Dialog } from "./Dialog.tsx";
import { Editor } from "./Editor.tsx";
import { FinishedDialog } from "./FinishedDialog.tsx";
import { IconButton } from "./IconButton.tsx";
import { KindBadge } from "./KindBadge.tsx";
import { ResultPanel } from "./ResultPanel.tsx";
import { type RunRules, usePlay } from "./usePlay.ts";

const NO_MARKERS: EditorMarker[] = [];

const isMac = typeof navigator !== "undefined" && /Mac|iPhone|iPad/.test(navigator.platform);
const SUBMIT_SHORTCUT = isMac ? "⌘↵" : "Ctrl+↵";

function Statement({ problem }: { problem: Problem }) {
  const { locale, t } = useApp();
  const [tab, setTab] = useState<"statement" | "harness">("statement");
  return (
    <aside class="statement">
      {problem.judge.harnessVisible && (
        <div class="tabs" role="tablist">
          <button
            type="button"
            role="tab"
            aria-selected={tab === "statement"}
            onClick={() => setTab("statement")}
          >
            {t.tabStatement}
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={tab === "harness"}
            onClick={() => setTab("harness")}
          >
            {t.tabHarness}
          </button>
        </div>
      )}
      {tab === "statement" ? (
        <>
          <div
            class="markdown"
            // statementHtml is rendered at build time from the repository's own Markdown.
            dangerouslySetInnerHTML={{ __html: localize(problem.statementHtml, locale) }}
          />
          <div class="concepts">
            {problem.concepts.map((c) => (
              <span class="tag" key={c}>
                {c}
              </span>
            ))}
          </div>
        </>
      ) : (
        <pre class="harness">{problem.tests}</pre>
      )}
    </aside>
  );
}

/** Set when the problem is played as part of a 連続チャレンジ. */
export interface ChallengeContext {
  id: string;
  rules: RunRules;
  /** e.g. "2 / 5" */
  position: string;
  onRunFinished: (run: Run) => void;
  /** Moves on to the next problem (after solving, failing or giving up). */
  onNext: () => void;
}

interface ConfirmProps {
  open: boolean;
  message: string;
  confirmLabel: string;
  onConfirm: () => void;
  onCancel: () => void;
}

function Confirm({ open, message, confirmLabel, onConfirm, onCancel }: ConfirmProps) {
  const { t } = useApp();
  return (
    <Dialog open={open} onClose={onCancel} onEnter={onConfirm} class="confirm">
      <p>{message}</p>
      <div class="actions">
        <button type="button" class="primary" onClick={onConfirm}>
          <kbd>↵</kbd>
          {confirmLabel}
        </button>
        <button type="button" onClick={onCancel}>
          {t.cancel}
        </button>
      </div>
    </Dialog>
  );
}

export function PlayView({
  problem,
  challenge,
}: {
  problem: Problem;
  challenge?: ChallengeContext;
}) {
  const { content, storage, settings, toolchain, locale, t } = useApp();
  const pool = toolchain.status === "ready" ? toolchain.pool : undefined;
  const freeRules = useMemo(
    () => ({ mode: settings.mode, lives: settings.lives }),
    [settings.mode, settings.lives],
  );
  const rules = challenge?.rules ?? freeRules;
  const { state, setCode, submit, restart, dismissFinished } = usePlay({
    problem,
    judgeSources: content.judge,
    storage,
    rules,
    pool,
    challengeId: challenge?.id,
    onRunFinished: challenge?.onRunFinished,
  });
  const [confirming, setConfirming] = useState<"submit" | "restart" | null>(null);

  const running = state.phase.kind === "running";
  const canSubmit = pool !== undefined && !running && state.loaded;
  const requestSubmit = () => {
    if (!canSubmit) return;
    if (settings.confirmSubmit) setConfirming("submit");
    else void submit();
  };

  const presentation = useMemo(
    () =>
      state.last?.kind === "judged"
        ? present(state.last.judgement, {
            mode: state.last.mode,
            harnessVisible: problem.judge.harnessVisible,
            messages: t,
          })
        : null,
    [state.last, problem, t],
  );

  const run = state.run;
  const playing = run?.status === "playing";
  const runMode = playing ? run.mode : rules.mode;
  const lives = playing ? remainingLives(run) : rules.lives;
  // A finished run keeps showing its count; the next submission starts a new run.
  const tries = run?.attempts.length ?? 0;
  const finishedInChallenge = challenge !== undefined && run !== null && !playing;

  return (
    <div class="play">
      <div class="play-header">
        {challenge === undefined ? (
          <IconButton
            icon={ArrowLeft}
            label={t.backToProblems}
            href={routeHref({ page: "list" })}
          />
        ) : (
          <span class="chip accent">{t.challengeChip(challenge.position)}</span>
        )}
        <h1>{localize(problem.title, locale)}</h1>
        <KindBadge kind={problem.kind} />
        <span class="chip" title={t.modeHeading}>
          {t.modeChip(t.modes[runMode])}
        </span>
        <span class="chip">{t.par(problem.par)}</span>
        <span class="chip strong">{t.tries(tries)}</span>
        {lives !== null && (
          <span class="chip" title={t.lives(lives)}>
            <Heart size={14} aria-hidden="true" /> {lives}
          </span>
        )}
        <span class="spacer" />
        {challenge === undefined && (
          <IconButton
            icon={History}
            label={t.history}
            href={routeHref({ page: "history", problemId: problem.id })}
          />
        )}
        {finishedInChallenge ? (
          <IconButton
            icon={SkipForward}
            label={t.nextInChallenge}
            onClick={challenge.onNext}
            class="primary"
          >
            {t.nextInChallenge}
          </IconButton>
        ) : (
          <>
            <IconButton
              icon={challenge === undefined ? RotateCcw : Flag}
              label={challenge === undefined ? t.startOver : t.giveUp}
              onClick={() => setConfirming("restart")}
              disabled={running}
            />
            <IconButton
              icon={Play}
              label={`${t.submit} (${SUBMIT_SHORTCUT})`}
              onClick={requestSubmit}
              disabled={!canSubmit}
              class="primary"
            >
              {running ? t.judging : pool === undefined ? t.toolchainStarting : t.submit}
              <kbd>{SUBMIT_SHORTCUT}</kbd>
            </IconButton>
          </>
        )}
      </div>
      {challenge === undefined && playing && run.mode !== settings.mode && (
        <p class="notice">{t.runModeNotice(t.modes[run.mode])}</p>
      )}
      <div class="play-body">
        <Statement problem={problem} />
        <div class="editor-pane">
          {state.loaded && (
            <Editor
              key={state.codeRevision}
              initialValue={state.code}
              onChange={setCode}
              onSubmit={requestSubmit}
              settings={settings.editor}
              markers={presentation?.markers ?? NO_MARKERS}
              readOnly={running}
            />
          )}
        </div>
      </div>
      <ResultPanel phase={state.phase} last={state.last} presentation={presentation} />

      <Confirm
        open={confirming === "submit"}
        message={t.confirmSubmit(playing ? tries + 1 : 1)}
        confirmLabel={t.confirmSubmitYes}
        onCancel={() => setConfirming(null)}
        onConfirm={() => {
          setConfirming(null);
          void submit();
        }}
      />
      <Confirm
        open={confirming === "restart"}
        message={
          challenge === undefined
            ? `${t.confirmStartOver}${playing ? ` ${t.startOverAbandons}` : ""}`
            : t.confirmGiveUp
        }
        confirmLabel={challenge === undefined ? t.confirmStartOverYes : t.confirmGiveUpYes}
        onCancel={() => setConfirming(null)}
        onConfirm={() => {
          setConfirming(null);
          void restart().then(() => challenge?.onNext());
        }}
      />
      <FinishedDialog
        problem={problem}
        finished={state.finished}
        onClose={dismissFinished}
        next={
          challenge === undefined
            ? {
                label: t.tryAgain,
                action: () => {
                  dismissFinished();
                  void restart();
                },
              }
            : {
                label: t.nextInChallenge,
                action: () => {
                  dismissFinished();
                  challenge.onNext();
                },
              }
        }
      />
    </div>
  );
}

export function PlayPage({ problemId }: { problemId: string }) {
  const { content, t } = useApp();
  const problem = content.problems.find((p) => p.id === problemId);
  if (problem === undefined) {
    return (
      <section>
        <p>{t.problemNotFound(problemId)}</p>
        <a href={routeHref({ page: "list" })}>{t.backToProblems}</a>
      </section>
    );
  }
  // Remount per problem so that all per-problem state starts fresh.
  return <PlayView key={problem.id} problem={problem} />;
}
