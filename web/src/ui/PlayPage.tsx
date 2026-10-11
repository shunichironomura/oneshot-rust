import { ArrowLeft, Flag, History, Lock, Play, RotateCcw, SkipForward, X } from "lucide-preact";
import type { ComponentChildren } from "preact";
import { useMemo, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { type EditorMarker, present } from "../core/present.ts";
import type { Problem } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import { type Run, verdictsOf } from "../core/run.ts";
import { type SolutionAccess, solutionAccess } from "../core/solution.ts";
import { CodeView } from "./CodeView.tsx";
import { useApp } from "./context.ts";
import { Confirm, Dialog } from "./Dialog.tsx";
import { Editor } from "./Editor.tsx";
import { FinishedDialog } from "./FinishedDialog.tsx";
import { IconButton } from "./IconButton.tsx";
import { KindBadge } from "./KindBadge.tsx";
import { ModeOptions } from "./ModeOptions.tsx";
import { ShotChip } from "./ShotMeter.tsx";
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
    {
      id: "solution",
      label: (
        <>
          {solution === "locked" && <Lock size={12} aria-hidden="true" />}
          {t.tabSolution}
        </>
      ),
    },
  ];
  return (
    <aside class="statement">
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
      {tab === "solution" ? (
        <div class="solution">{solutionPanel}</div>
      ) : tab === "statement" ? (
        <div
          class="markdown"
          // statementHtml is rendered at build time from the repository's own Markdown.
          dangerouslySetInnerHTML={{ __html: localize(problem.statementHtml, locale) }}
        />
      ) : (
        <div class="harness">
          <CodeView code={problem.tests} />
        </div>
      )}
    </aside>
  );
}

/** How the problem is played: as part of a round, or on its own as practice. */
export type Session =
  | {
      kind: "round";
      roundId: string;
      rules: RunRules;
      /** e.g. "2 / 5" */
      position: string;
      onRunFinished: (run: Run) => void;
      /** Moves on after the problem is finished: to the next problem, or the scorecard. */
      next: { label: string; action: () => void };
      /** Ends the whole round early (after confirmation). */
      abandon: () => void;
    }
  | { kind: "practice" };

export function PlayView({ problem, session }: { problem: Problem; session: Session }) {
  const { content, storage, settings, toolchain, locale, t } = useApp();
  const round = session.kind === "round" ? session : undefined;
  // A submission made while the compiler is still loading waits for it; the time
  // limit only starts once a Worker has picked the submission up.
  const toolchainFailed =
    toolchain.state.status === "missing" || toolchain.state.status === "error";
  const practiceRules = useMemo(() => ({ mode: settings.mode }), [settings.mode]);
  const rules = round?.rules ?? practiceRules;
  const { state, setCode, submit, restart, giveUp, dismissFinished, revealSolution } = usePlay({
    problem,
    judgeSources: content.judge,
    storage,
    rules,
    pool: toolchain.pool,
    roundId: round?.roundId,
    onRunFinished: round?.onRunFinished,
  });
  const [confirming, setConfirming] = useState<
    "submit" | "restart" | "give-up" | "reveal" | "abandon" | null
  >(null);
  const [choosingMode, setChoosingMode] = useState(false);
  const [tab, setTab] = useState<StatementTab>("statement");
  const solution = solutionAccess({ revealed: state.revealed, finished: state.hasFinished });

  const running = state.phase.kind !== "idle";
  const canSubmit = !toolchainFailed && !running && state.loaded;
  const requestSubmit = () => {
    if (!canSubmit) return;
    setConfirming("submit");
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
  // A finished run keeps showing its count; in practice, the next submission starts a new run.
  const tries = run?.attempts.length ?? 0;
  const finishedInRound = round !== undefined && run !== null && !playing;
  // Seeing the solution turns the current and all later runs into assisted ones.
  const assisted = playing ? run.assisted === true : state.revealed;
  const showSolution = () => {
    dismissFinished();
    setTab("solution");
  };

  return (
    <div class="play">
      <div class="play-header">
        {round === undefined ? (
          <>
            <IconButton
              icon={ArrowLeft}
              label={t.backToRecords}
              href={routeHref({ page: "problems" })}
            />
            <span class="chip" title={t.practiceTooltip}>
              {t.practiceChip}
            </span>
          </>
        ) : (
          <>
            <IconButton
              icon={X}
              label={t.abandonRound}
              onClick={() => setConfirming("abandon")}
              disabled={running || !state.loaded}
            />
            <span class="chip accent">{t.roundChip(round.position)}</span>
          </>
        )}
        <h1>{localize(problem.title, locale)}</h1>
        <KindBadge kind={problem.kind} />
        <button
          type="button"
          class="chip"
          title={t.modeHeading}
          onClick={round === undefined ? () => setChoosingMode(true) : undefined}
          disabled={round !== undefined}
        >
          {t.modeChip(t.modes[runMode])}
        </button>
        <ShotChip shots={run === null ? [] : verdictsOf(run)} label={t.shotsTooltip(tries)} />
        {assisted && (
          <span class="chip" title={t.assistedTooltip}>
            {t.assistedChip}
          </span>
        )}
        <span class="spacer" />
        {round === undefined && (
          <IconButton
            icon={History}
            label={t.history}
            href={routeHref({ page: "history", problemId: problem.id })}
          />
        )}
        {finishedInRound ? (
          <IconButton
            icon={SkipForward}
            label={round.next.label}
            onClick={round.next.action}
            class="primary"
          >
            {round.next.label}
          </IconButton>
        ) : (
          <>
            {round === undefined ? (
              <IconButton
                icon={RotateCcw}
                label={t.startOver}
                onClick={() => setConfirming("restart")}
                disabled={running}
              />
            ) : (
              <IconButton
                icon={Flag}
                label={t.giveUp}
                onClick={() => setConfirming("give-up")}
                disabled={running || !state.loaded}
              />
            )}
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
      {round === undefined && playing && run.mode !== settings.mode && (
        <p class="notice">{t.runModeNotice(t.modes[run.mode])}</p>
      )}
      <div class="play-body">
        <Statement
          problem={problem}
          tab={tab}
          onTab={setTab}
          solution={solution}
          solutionPanel={
            <SolutionPanel
              problem={problem}
              access={solution}
              code={state.code}
              onReveal={() => void revealSolution()}
              // In a round, looking at the solution ends the problem.
              onGiveUp={() => setConfirming(round === undefined ? "reveal" : "give-up")}
            />
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

      <Dialog open={choosingMode} onClose={() => setChoosingMode(false)} class="confirm">
        <h2>{t.modeHeading}</h2>
        <ModeOptions />
        <div class="actions">
          <button type="button" class="primary" onClick={() => setChoosingMode(false)}>
            {t.close}
          </button>
        </div>
      </Dialog>
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
        message={`${t.confirmStartOver}${playing ? ` ${t.startOverAbandons}` : ""}`}
        confirmLabel={t.confirmStartOverYes}
        onCancel={() => setConfirming(null)}
        onConfirm={() => {
          setConfirming(null);
          void restart();
        }}
      />
      <Confirm
        open={confirming === "give-up"}
        message={t.confirmGiveUp}
        confirmLabel={t.confirmGiveUpYes}
        onCancel={() => setConfirming(null)}
        onConfirm={() => {
          setConfirming(null);
          void giveUp();
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
      <Confirm
        open={confirming === "abandon"}
        message={t.confirmAbandonRound}
        confirmLabel={t.confirmAbandonRoundYes}
        onCancel={() => setConfirming(null)}
        onConfirm={() => {
          setConfirming(null);
          round?.abandon();
        }}
      />
      <FinishedDialog
        finished={state.finished}
        onClose={dismissFinished}
        onShowSolution={showSolution}
        next={
          round === undefined
            ? {
                label: t.tryAgain,
                action: () => {
                  dismissFinished();
                  void restart();
                },
              }
            : {
                label: round.next.label,
                action: () => {
                  dismissFinished();
                  round.next.action();
                },
              }
        }
      />
    </div>
  );
}

const PRACTICE: Session = { kind: "practice" };

/** A problem played on its own, as practice. */
export function PracticePage({ problemId }: { problemId: string }) {
  const { content, t } = useApp();
  const problem = content.problems.find((p) => p.id === problemId);
  if (problem === undefined) {
    return (
      <section>
        <p>{t.problemNotFound(problemId)}</p>
        <a href={routeHref({ page: "problems" })}>{t.backToRecords}</a>
      </section>
    );
  }
  // Remount per problem so that all per-problem state starts fresh.
  return <PlayView key={problem.id} problem={problem} session={PRACTICE} />;
}
