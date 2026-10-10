import {
  ArrowLeft,
  ArrowRight,
  Flag,
  Heart,
  History,
  Lock,
  Play,
  RotateCcw,
  SkipForward,
} from "lucide-preact";
import type { ComponentChildren } from "preact";
import { useMemo, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { type EditorMarker, present } from "../core/present.ts";
import type { Problem } from "../core/problem.ts";
import { nextProblem } from "../core/recommend.ts";
import { routeHref } from "../core/route.ts";
import { remainingLives, type Run, verdictsOf } from "../core/run.ts";
import { type SolutionAccess, solutionAccess } from "../core/solution.ts";
import { useApp } from "./context.ts";
import { Dialog } from "./Dialog.tsx";
import { Editor } from "./Editor.tsx";
import { FinishedDialog } from "./FinishedDialog.tsx";
import { IconButton } from "./IconButton.tsx";
import { KindBadge } from "./KindBadge.tsx";
import { ParChip } from "./ParMeter.tsx";
import { ResultPanel } from "./ResultPanel.tsx";
import { SolutionPanel } from "./SolutionPanel.tsx";
import { type RunRules, usePlay } from "./usePlay.ts";

const NO_MARKERS: EditorMarker[] = [];

const isMac = typeof navigator !== "undefined" && /Mac|iPhone|iPad/.test(navigator.platform);
const SUBMIT_SHORTCUT = isMac ? "⌘↵" : "Ctrl+↵";

type StatementTab = "statement" | "harness" | "solution";

interface StatementProps {
  problem: Problem;
  tab: StatementTab;
  onTab: (tab: StatementTab) => void;
  solution: SolutionAccess;
  solutionPanel: ComponentChildren;
}

function Statement({ problem, tab, onTab, solution, solutionPanel }: StatementProps) {
  const { locale, t } = useApp();
  const tabs: { id: StatementTab; label: ComponentChildren }[] = [
    { id: "statement", label: t.tabStatement },
    ...(problem.judge.harnessVisible ? [{ id: "harness" as const, label: t.tabHarness }] : []),
    ...(solution === "hidden"
      ? []
      : [
          {
            id: "solution" as const,
            label: (
              <>
                {solution === "locked" && <Lock size={12} aria-hidden="true" />}
                {t.tabSolution}
              </>
            ),
          },
        ]),
  ];
  return (
    <aside class="statement">
      {tabs.length > 1 && (
        <div class="tabs" role="tablist">
          {tabs.map(({ id, label }) => (
            <button
              key={id}
              type="button"
              role="tab"
              aria-selected={tab === id}
              onClick={() => onTab(id)}
            >
              {label}
            </button>
          ))}
        </div>
      )}
      {tab === "solution" && solution !== "hidden" ? (
        <div class="solution">{solutionPanel}</div>
      ) : tab === "statement" || tab === "solution" ? (
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
  const { content, storage, settings, toolchain, locale, t, openSettings } = useApp();
  // A submission made while the compiler is still loading waits for it; the time
  // limit only starts once a Worker has picked the submission up.
  const toolchainFailed =
    toolchain.state.status === "missing" || toolchain.state.status === "error";
  const freeRules = useMemo(
    () => ({ mode: settings.mode, lives: settings.lives }),
    [settings.mode, settings.lives],
  );
  const rules = challenge?.rules ?? freeRules;
  const { state, setCode, submit, restart, dismissFinished, revealSolution } = usePlay({
    problem,
    judgeSources: content.judge,
    storage,
    rules,
    pool: toolchain.pool,
    challengeId: challenge?.id,
    onRunFinished: challenge?.onRunFinished,
  });
  const [confirming, setConfirming] = useState<"submit" | "restart" | "reveal" | null>(null);
  const [tab, setTab] = useState<StatementTab>("statement");
  const solution = solutionAccess({
    inChallenge: challenge !== undefined,
    revealed: state.revealed,
    finished: state.hasFinished,
  });

  const running = state.phase.kind !== "idle";
  const canSubmit = !toolchainFailed && !running && state.loaded;
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
  const nextId =
    challenge === undefined
      ? nextProblem(
          content.problems.map((p) => p.id),
          problem.id,
        )
      : undefined;
  const nextProblemHref =
    nextId === undefined ? undefined : routeHref({ page: "play", problemId: nextId });
  // Seeing the solution turns the current and all later runs into practice.
  const practice = playing ? run.assisted === true : state.revealed;

  return (
    <div class="play">
      <div class="play-header">
        {challenge === undefined ? (
          <IconButton
            icon={ArrowLeft}
            label={t.backToProblems}
            href={routeHref({ page: "problems" })}
          />
        ) : (
          <span class="chip accent">{t.challengeChip(challenge.position)}</span>
        )}
        <h1>{localize(problem.title, locale)}</h1>
        <KindBadge kind={problem.kind} />
        <button
          type="button"
          class="chip"
          title={t.modeHeading}
          onClick={challenge === undefined ? openSettings : undefined}
          disabled={challenge !== undefined}
        >
          {t.modeChip(t.modes[runMode])}
        </button>
        <ParChip
          par={problem.par}
          shots={run === null ? [] : verdictsOf(run)}
          label={t.meterTooltip(problem.par, tries)}
        />
        {practice && (
          <span class="chip" title={t.practiceTooltip}>
            {t.practiceChip}
          </span>
        )}
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
        {nextProblemHref !== undefined && run?.status === "solved" && (
          <IconButton icon={ArrowRight} label={t.nextProblem} href={nextProblemHref}>
            {t.nextProblem}
          </IconButton>
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
              {running ? t.judging : t.submit}
              <kbd>{SUBMIT_SHORTCUT}</kbd>
            </IconButton>
          </>
        )}
      </div>
      {challenge === undefined && playing && run.mode !== settings.mode && (
        <p class="notice">{t.runModeNotice(t.modes[run.mode])}</p>
      )}
      <div class="play-body">
        <Statement
          problem={problem}
          tab={tab}
          onTab={setTab}
          solution={solution}
          solutionPanel={
            solution !== "hidden" && (
              <SolutionPanel
                problem={problem}
                access={solution}
                code={state.code}
                onReveal={() => void revealSolution()}
                onGiveUp={() => setConfirming("reveal")}
              />
            )
          }
        />
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
      <Confirm
        open={confirming === "reveal"}
        message={t.confirmRevealSolution}
        confirmLabel={t.confirmRevealSolutionYes}
        onCancel={() => setConfirming(null)}
        onConfirm={() => {
          setConfirming(null);
          void revealSolution();
        }}
      />
      <FinishedDialog
        problem={problem}
        finished={state.finished}
        nextProblemHref={nextProblemHref}
        onClose={dismissFinished}
        onShowSolution={
          solution === "hidden"
            ? undefined
            : () => {
                dismissFinished();
                setTab("solution");
              }
        }
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
        <a href={routeHref({ page: "problems" })}>{t.backToProblems}</a>
      </section>
    );
  }
  // Remount per problem so that all per-problem state starts fresh.
  return <PlayView key={problem.id} problem={problem} />;
}
