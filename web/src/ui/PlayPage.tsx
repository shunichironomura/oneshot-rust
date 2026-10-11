import { ArrowLeft, Flag, Play, RotateCcw, SkipForward } from "lucide-preact";
import type { ComponentChildren } from "preact";
import { useEffect, useLayoutEffect, useMemo, useRef, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { type EditorMarker, present } from "../core/present.ts";
import type { Problem } from "../core/problem.ts";
import type { Round } from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import { type Run, verdictsOf } from "../core/run.ts";
import { CodeView } from "./CodeView.tsx";
import { useApp } from "./context.ts";
import { Confirm } from "./Dialog.tsx";
import { Editor } from "./Editor.tsx";
import { FinishedDialog } from "./FinishedDialog.tsx";
import { IconButton } from "./IconButton.tsx";
import { ModelSolution } from "./ModelSolution.tsx";
import { ResultPanel } from "./ResultPanel.tsx";
import { RoundProgress } from "./RoundProgress.tsx";
import { ShotChip } from "./ShotMeter.tsx";
import { usePlay } from "./usePlay.ts";

const NO_MARKERS: EditorMarker[] = [];

const isMac = typeof navigator !== "undefined" && /Mac|iPhone|iPad/.test(navigator.platform);
const SUBMIT_SHORTCUT = isMac ? "⌘↵" : "Ctrl+↵";

type StatementTab = "statement" | "harness" | "solution";

interface StatementProps {
  problem: Problem;
  tab: StatementTab;
  onTab: (tab: StatementTab) => void;
  /** The model solution, once the run has ended; `null` before. */
  solution: ComponentChildren | null;
}

/** The problem, with tabs only when there is more to see: a visible harness, the solution. */
function Statement({ problem, tab, onTab, solution }: StatementProps) {
  const { locale, t } = useApp();
  const tabs: { id: StatementTab; label: string }[] = [
    { id: "statement", label: t.tabStatement },
    ...(problem.judge.harnessVisible ? [{ id: "harness" as const, label: t.tabHarness }] : []),
    ...(solution === null ? [] : [{ id: "solution" as const, label: t.tabSolution }]),
  ];
  const shown = tabs.some(({ id }) => id === tab) ? tab : "statement";
  return (
    <aside class="statement">
      {tabs.length > 1 && (
        <div class="tabs" role="tablist">
          {tabs.map(({ id, label }) => (
            <button
              key={id}
              type="button"
              role="tab"
              aria-selected={shown === id}
              onClick={() => onTab(id)}
            >
              {label}
            </button>
          ))}
        </div>
      )}
      {shown === "solution" ? (
        <div class="solution">{solution}</div>
      ) : shown === "statement" ? (
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
      round: Round;
      /** The problem shown (0-based). */
      index: number;
      onRunFinished: (run: Run) => void;
      /** Moves on after the problem is finished: to the next problem, or the scorecard. */
      next: { label: string; action: () => void };
    }
  | { kind: "practice" };

export function PlayView({ problem, session }: { problem: Problem; session: Session }) {
  const { content, storage, toolchain, locale, t } = useApp();
  const round = session.kind === "round" ? session : undefined;
  // A submission made while the compiler is still loading waits for it; the time
  // limit only starts once a Worker has picked the submission up.
  const toolchainFailed =
    toolchain.state.status === "missing" || toolchain.state.status === "error";
  const { state, setCode, submit, restart, giveUp, dismissFinished, revealSolution } = usePlay({
    problem,
    judgeSources: content.judge,
    storage,
    pool: toolchain.pool,
    roundId: round?.round.id,
    onRunFinished: round?.onRunFinished,
  });
  const [confirming, setConfirming] = useState<"submit" | "give-up" | null>(null);
  const [tab, setTab] = useState<StatementTab>("statement");

  const run = state.run;
  const playing = run?.status === "playing";
  // Solved or given up: what is left is moving on, and the model solution.
  const ended = run !== null && !playing;
  const tries = run?.attempts.length ?? 0;
  // A run started after seeing the model solution does not update the best.
  const assisted = run === null ? state.revealed : run.assisted === true;

  const running = state.phase.kind !== "idle";
  const canSubmit = !toolchainFailed && !running && state.loaded && !ended;
  const requestSubmit = () => {
    if (!canSubmit) return;
    setConfirming("submit");
  };

  // On small screens the view scrolls as one page (see styles.css): a problem starts at the
  // top, and a submission brings the result, at the end, into view. On larger screens the
  // view fits the screen, so these scroll nothing.
  const view = useRef<HTMLDivElement>(null);
  const scrollToTop = () => view.current?.scrollIntoView({ block: "start" });
  useLayoutEffect(() => scrollToTop(), []);
  useEffect(() => {
    if (running) view.current?.scrollIntoView({ block: "end" });
  }, [running]);

  const presentation = useMemo(
    () =>
      state.last?.kind === "judged"
        ? present(state.last.judgement, {
            harnessVisible: problem.judge.harnessVisible,
            messages: t,
          })
        : null,
    [state.last, problem, t],
  );

  const openTab = (next: StatementTab) => {
    setTab(next);
    if (next === "solution" && !state.revealed) void revealSolution();
  };
  const next = round?.next ?? {
    label: t.tryAgain,
    action: () => {
      setTab("statement");
      scrollToTop();
      void restart();
    },
  };

  return (
    <div class="play" ref={view}>
      <div class="play-header">
        {round === undefined ? (
          <>
            <IconButton
              icon={ArrowLeft}
              label={t.history}
              href={routeHref({ page: "history", problemId: problem.id })}
            />
            <span class="chip" title={t.practiceTooltip}>
              {t.practiceChip}
            </span>
          </>
        ) : (
          <RoundProgress
            round={round.round}
            current={round.index}
            label={t.roundPosition(round.index + 1, round.round.problemIds.length)}
          />
        )}
        <h1>{localize(problem.title, locale)}</h1>
        <ShotChip shots={run === null ? [] : verdictsOf(run)} label={t.shotsTooltip(tries)} />
        {assisted && (
          <span class="chip" title={t.assistedTooltip}>
            {t.assistedChip}
          </span>
        )}
        <div class="play-actions">
          {ended ? (
            <IconButton
              icon={round === undefined ? RotateCcw : SkipForward}
              label={next.label}
              onClick={next.action}
              class="primary"
            >
              {next.label}
            </IconButton>
          ) : (
            <>
              <IconButton
                icon={Flag}
                label={t.giveUp}
                onClick={() => setConfirming("give-up")}
                disabled={running || !state.loaded}
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
      </div>
      <div class="play-body">
        <Statement
          problem={problem}
          tab={tab}
          onTab={openTab}
          solution={ended ? <ModelSolution problem={problem} code={state.code} /> : null}
        />
        <div class="editor-pane">
          {state.loaded && (
            <Editor
              key={state.codeRevision}
              initialValue={state.code}
              onChange={setCode}
              onSubmit={requestSubmit}
              markers={presentation?.markers ?? NO_MARKERS}
              readOnly={running || ended}
            />
          )}
        </div>
      </div>
      <ResultPanel phase={state.phase} last={state.last} presentation={presentation} />

      <Confirm
        open={confirming === "submit"}
        message={t.confirmSubmit(tries + 1)}
        confirmLabel={t.confirmSubmitYes}
        onCancel={() => setConfirming(null)}
        onConfirm={() => {
          setConfirming(null);
          void submit();
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
      <FinishedDialog
        finished={state.finished}
        onClose={dismissFinished}
        onShowSolution={() => {
          dismissFinished();
          openTab("solution");
          scrollToTop();
        }}
        next={{
          label: next.label,
          action: () => {
            dismissFinished();
            next.action();
          },
        }}
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
