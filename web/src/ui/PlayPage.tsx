import { useMemo, useState } from "preact/hooks";
import { MODE_LABELS } from "../core/mode.ts";
import { type EditorMarker, present } from "../core/present.ts";
import { KIND_LABELS, type Problem } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import { remainingLives, type Run } from "../core/run.ts";
import { useApp } from "./context.ts";
import { Dialog } from "./Dialog.tsx";
import { Editor } from "./Editor.tsx";
import { FinishedDialog } from "./FinishedDialog.tsx";
import { ResultPanel } from "./ResultPanel.tsx";
import { type RunRules, usePlay } from "./usePlay.ts";

const NO_MARKERS: EditorMarker[] = [];

function Statement({ problem }: { problem: Problem }) {
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
            問題文
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={tab === "harness"}
            onClick={() => setTab("harness")}
          >
            tests.rs（読み取り専用）
          </button>
        </div>
      )}
      {tab === "statement" ? (
        // statementHtml is rendered at build time from the repository's own Markdown.
        <div class="markdown" dangerouslySetInnerHTML={{ __html: problem.statementHtml }} />
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

export function PlayView({
  problem,
  challenge,
}: {
  problem: Problem;
  challenge?: ChallengeContext;
}) {
  const { content, storage, settings, toolchain } = useApp();
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
  const [confirming, setConfirming] = useState(false);
  const [confirmingRestart, setConfirmingRestart] = useState(false);

  const running = state.phase.kind === "running";
  const canSubmit = pool !== undefined && !running && state.loaded;
  const requestSubmit = () => {
    if (!canSubmit) return;
    if (settings.confirmSubmit) setConfirming(true);
    else void submit();
  };

  const presentation = useMemo(
    () =>
      state.last?.kind === "judged"
        ? present(state.last.judgement, {
            mode: state.last.mode,
            harnessVisible: problem.judge.harnessVisible,
          })
        : null,
    [state.last, problem],
  );

  const run = state.run;
  const playing = run?.status === "playing";
  const runMode = playing ? run.mode : rules.mode;
  const lives = playing ? remainingLives(run) : rules.lives;

  return (
    <div class="play">
      <div class="play-header">
        {challenge !== undefined && <span class="tag">連続チャレンジ {challenge.position}</span>}
        <h1>{problem.title}</h1>
        <span class="tag">{KIND_LABELS[problem.kind]}</span>
        <span class="meta">Par {problem.par}</span>
        <span class="meta">モード: {MODE_LABELS[runMode]}</span>
        <span class="meta">提出: {playing ? run.attempts.length : 0} 回</span>
        {lives !== null && <span class="meta">残機: {lives}</span>}
        <span class="spacer" />
        {challenge === undefined && (
          <a href={routeHref({ page: "history", problemId: problem.id })}>履歴</a>
        )}
        {challenge !== undefined && run !== null && !playing ? (
          <button type="button" class="primary" onClick={challenge.onNext}>
            次へ進む
          </button>
        ) : (
          <button type="button" onClick={() => setConfirmingRestart(true)} disabled={running}>
            {challenge === undefined ? "最初から" : "ギブアップ"}
          </button>
        )}
        <button type="button" class="primary" onClick={requestSubmit} disabled={!canSubmit}>
          {running ? "判定中…" : "提出 (Ctrl+Enter)"}
        </button>
      </div>
      {challenge === undefined && playing && run.mode !== settings.mode && (
        <p class="notice">
          このランは「{MODE_LABELS[run.mode]}
          」モードで開始されています。モードの変更は次のランから反映されます。
        </p>
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

      <Dialog open={confirming} onClose={() => setConfirming(false)} class="confirm">
        <p>この内容で提出しますか？（{playing ? run.attempts.length + 1 : 1} 回目の提出）</p>
        <div class="actions">
          <button
            type="button"
            class="primary"
            autofocus
            onClick={() => {
              setConfirming(false);
              void submit();
            }}
          >
            提出する
          </button>
          <button type="button" onClick={() => setConfirming(false)}>
            やめる
          </button>
        </div>
      </Dialog>
      <Dialog open={confirmingRestart} onClose={() => setConfirmingRestart(false)} class="confirm">
        <p>
          {challenge === undefined
            ? "コードをテンプレートに戻し、新しいランを始めますか？"
            : "この問題をあきらめて次に進みますか？（未解決として記録されます）"}
          {challenge === undefined && playing && "（進行中のランは未解決で終了します）"}
        </p>
        <div class="actions">
          <button
            type="button"
            class="primary"
            onClick={() => {
              setConfirmingRestart(false);
              void restart().then(() => challenge?.onNext());
            }}
          >
            {challenge === undefined ? "最初からやり直す" : "ギブアップする"}
          </button>
          <button type="button" onClick={() => setConfirmingRestart(false)}>
            やめる
          </button>
        </div>
      </Dialog>
      <FinishedDialog
        problem={problem}
        finished={state.finished}
        onClose={dismissFinished}
        next={
          challenge === undefined
            ? {
                label: "最初から挑戦",
                action: () => {
                  dismissFinished();
                  void restart();
                },
              }
            : {
                label: "次へ進む",
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
  const { content } = useApp();
  const problem = content.problems.find((p) => p.id === problemId);
  if (problem === undefined) {
    return (
      <p>
        問題「{problemId}」は見つかりませんでした。
        <a href={routeHref({ page: "list" })}>問題一覧へ</a>
      </p>
    );
  }
  // Remount per problem so that all per-problem state starts fresh.
  return <PlayView key={problem.id} problem={problem} />;
}
