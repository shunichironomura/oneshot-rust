import { useCallback, useEffect, useMemo, useState } from "preact/hooks";
import {
  CHALLENGE_LENGTH,
  type Challenge,
  challengeShareText,
  newChallenge,
  pickProblems,
  recordRun,
  solvedCount,
  totalSubmissions,
} from "../core/challenge.ts";
import { MODE_LABELS } from "../core/mode.ts";
import type { Run } from "../core/run.ts";
import { useApp } from "./context.ts";
import { type ChallengeContext, PlayView } from "./PlayPage.tsx";

const dateFormat = new Intl.DateTimeFormat("ja-JP", { dateStyle: "short", timeStyle: "short" });

function Summary({ challenge, onNew }: { challenge: Challenge; onNew: () => void }) {
  const [copied, setCopied] = useState(false);
  const text = challengeShareText(challenge);
  return (
    <section class="challenge">
      <h1>連続チャレンジ 結果</h1>
      <p class="challenge-total">
        合計 <strong>{totalSubmissions(challenge)}</strong> 回（{solvedCount(challenge)} /{" "}
        {challenge.problemIds.length} 問正解）
      </p>
      <pre class="share">{text}</pre>
      <div class="actions">
        <button
          type="button"
          onClick={() => void navigator.clipboard.writeText(text).then(() => setCopied(true))}
        >
          {copied ? "コピーしました" : "結果をコピー"}
        </button>
        <button type="button" class="primary" onClick={onNew}>
          新しいチャレンジ
        </button>
      </div>
    </section>
  );
}

function Start({ past, onStart }: { past: Challenge[]; onStart: () => void }) {
  const { settings } = useApp();
  return (
    <section class="challenge">
      <h1>連続チャレンジ</h1>
      <p>
        ランダムに選ばれた {CHALLENGE_LENGTH} 問を続けて解き、<strong>合計の提出回数</strong>
        を競います。途中でギブアップすると、その問題は未解決として記録されます。
      </p>
      <p class="hint">
        モード: {MODE_LABELS[settings.mode]} / 残機:{" "}
        {settings.lives === null ? "なし" : `${settings.lives} 回`}（設定画面で変更できます）
      </p>
      <button type="button" class="primary" onClick={onStart}>
        開始する
      </button>
      {past.length > 0 && (
        <>
          <h2>これまでの記録</h2>
          <ul class="challenge-history">
            {past.map((c) => (
              <li key={c.id}>
                {dateFormat.format(c.startedAt)} [{MODE_LABELS[c.mode]}] 合計 {totalSubmissions(c)}{" "}
                回（{solvedCount(c)} / {c.problemIds.length} 問正解）
              </li>
            ))}
          </ul>
        </>
      )}
    </section>
  );
}

export function ChallengePage() {
  const { content, storage, settings } = useApp();
  const [challenges, setChallenges] = useState<Challenge[]>();
  const [active, setActive] = useState<Challenge>();
  /**
   * Whether to keep showing the problem that was just finished (to review the result)
   * instead of the next one. The shown problem is derived from the recorded entries,
   * so moving on twice cannot skip a problem.
   */
  const [lingering, setLingering] = useState(false);

  useEffect(() => {
    void storage.challenges().then((all) => {
      setChallenges(all);
      const unfinished = all.find((c) => c.finishedAt === undefined);
      if (unfinished !== undefined) setActive(unfinished);
    });
  }, [storage]);

  useEffect(() => {
    if (active !== undefined) void storage.saveChallenge(active);
  }, [active, storage]);

  const start = useCallback(() => {
    const ids = content.problems.map((p) => p.id);
    setActive(
      newChallenge({
        id: crypto.randomUUID(),
        mode: settings.mode,
        lives: settings.lives,
        problemIds: pickProblems(ids, CHALLENGE_LENGTH, Math.random),
        now: Date.now(),
      }),
    );
    setLingering(false);
  }, [content, settings]);

  const onRunFinished = useCallback((run: Run) => {
    setActive((c) => (c === undefined ? c : recordRun(c, run)));
    setLingering(true);
  }, []);
  const onNext = useCallback(() => setLingering(false), []);
  const shownIndex =
    active === undefined ? 0 : Math.max(0, active.entries.length - (lingering ? 1 : 0));

  const context = useMemo((): ChallengeContext | undefined => {
    if (active === undefined) return undefined;
    return {
      id: active.id,
      rules: { mode: active.mode, lives: active.lives },
      position: `${shownIndex + 1} / ${active.problemIds.length}`,
      onRunFinished,
      onNext,
    };
  }, [active, shownIndex, onRunFinished, onNext]);

  if (challenges === undefined) return null;
  if (active === undefined || context === undefined) {
    return <Start past={challenges.filter((c) => c.finishedAt !== undefined)} onStart={start} />;
  }
  const problemId = active.problemIds[shownIndex];
  if (problemId === undefined) {
    return (
      <Summary
        challenge={active}
        onNew={() => {
          setChallenges((all) => [active, ...(all ?? []).filter((c) => c.id !== active.id)]);
          setActive(undefined);
        }}
      />
    );
  }
  const problem = content.problems.find((p) => p.id === problemId);
  if (problem === undefined) {
    return (
      <section class="challenge">
        <p>問題「{problemId}」は削除されたため、スキップします。</p>
        <button type="button" onClick={onNext}>
          次へ進む
        </button>
      </section>
    );
  }
  return <PlayView key={`${active.id}:${problem.id}`} problem={problem} challenge={context} />;
}
