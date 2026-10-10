import { Copy, Play, Trophy } from "lucide-preact";
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
import { routeHref } from "../core/route.ts";
import type { Run } from "../core/run.ts";
import { useApp } from "./context.ts";
import { type ChallengeContext, PlayView } from "./PlayPage.tsx";

function Summary({ challenge, onNew }: { challenge: Challenge; onNew: () => void }) {
  const { t } = useApp();
  const [copied, setCopied] = useState(false);
  const text = challengeShareText(challenge, t);
  return (
    <section class="challenge">
      <h1>{t.challengeResult}</h1>
      <p class="challenge-total">
        {t.challengeTotal(
          totalSubmissions(challenge),
          solvedCount(challenge),
          challenge.problemIds.length,
        )}
      </p>
      <pre class="share">{text}</pre>
      <div class="actions">
        <button type="button" class="primary" onClick={onNew}>
          {t.challengeNew}
        </button>
        <button
          type="button"
          class="icon-button"
          onClick={() => void navigator.clipboard.writeText(text).then(() => setCopied(true))}
        >
          <Copy size={16} aria-hidden="true" />
          {copied ? t.copied : t.copyResult}
        </button>
      </div>
    </section>
  );
}

function Start({ past, onStart }: { past: Challenge[]; onStart: () => void }) {
  const { settings, locale, t } = useApp();
  const dateFormat = new Intl.DateTimeFormat(locale, { dateStyle: "medium", timeStyle: "short" });
  return (
    <section class="challenge">
      <h1>
        <Trophy size={26} aria-hidden="true" /> {t.challengeTitle}
      </h1>
      <p class="lead">{t.challengeIntro(CHALLENGE_LENGTH)}</p>
      <p class="hint">
        {t.challengeRules(
          t.modes[settings.mode],
          settings.lives === null ? t.livesUnlimited : String(settings.lives),
        )}{" "}
        · <a href={routeHref({ page: "settings" })}>{t.navSettings}</a>
      </p>
      <button type="button" class="primary large" onClick={onStart}>
        <Play size={18} aria-hidden="true" /> {t.challengeStart}
      </button>
      {past.length > 0 && (
        <>
          <h2>{t.challengePast}</h2>
          <ul class="challenge-history">
            {past.map((c) => (
              <li key={c.id}>
                <span class="run-date">{dateFormat.format(c.startedAt)}</span>
                <span class="chip">{t.modes[c.mode]}</span>
                {t.challengeTotal(totalSubmissions(c), solvedCount(c), c.problemIds.length)}
              </li>
            ))}
          </ul>
        </>
      )}
    </section>
  );
}

export function ChallengePage() {
  const { content, storage, settings, t } = useApp();
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
        <p>{t.challengeSkipDeleted(problemId)}</p>
        <button type="button" onClick={onNext}>
          {t.nextInChallenge}
        </button>
      </section>
    );
  }
  return <PlayView key={`${active.id}:${problem.id}`} problem={problem} challenge={context} />;
}
