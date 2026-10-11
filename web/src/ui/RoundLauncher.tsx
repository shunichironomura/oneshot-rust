import { ArrowRight, Play } from "lucide-preact";
import { useEffect, useState } from "preact/hooks";
import { DIFFICULTIES, type Problem } from "../core/problem.ts";
import {
  abandonRound,
  activeRound,
  courseName,
  courseProblemIds,
  currentProblemId,
  newRound,
  pickProblems,
  ROUND_LENGTH,
  type Round,
  roundPosition,
} from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import type { Settings } from "../core/settings.ts";
import type { Storage } from "../storage/db.ts";
import { useApp } from "./context.ts";
import { Confirm } from "./Dialog.tsx";

/**
 * Starts a round with the course of the settings, unless one is already in progress: then
 * that one is returned.
 */
export async function startRound(
  storage: Storage,
  problems: readonly Problem[],
  settings: Settings,
): Promise<Round> {
  const [rounds, bests] = await Promise.all([storage.rounds(), storage.allBests()]);
  const active = activeRound(rounds);
  if (active !== undefined) return active;
  const solved = new Set(bests.map((b) => b.problemId));
  const round = newRound({
    id: crypto.randomUUID(),
    course: settings.course,
    problemIds: pickProblems(
      courseProblemIds(problems, settings.course),
      solved,
      ROUND_LENGTH,
      Math.random,
    ),
    now: Date.now(),
  });
  await storage.saveRound(round);
  return round;
}

/** Ends a round in progress early, ending the run of its current problem unsolved. */
export async function abandonActiveRound(storage: Storage, round: Round): Promise<void> {
  const now = Date.now();
  const problemId = currentProblemId(round);
  const runs = problemId === undefined ? [] : await storage.runsOf(problemId);
  const playing = runs.find((r) => r.status === "playing" && r.challengeId === round.id);
  const ended =
    playing === undefined ? undefined : { ...playing, status: "failed" as const, finishedAt: now };
  if (ended !== undefined) await storage.saveRun(ended);
  await storage.saveRound(abandonRound(round, ended, now));
}

/**
 * The way into the game: "Continue" (or abandoning) when a round is in progress, and
 * otherwise the course and "Start round".
 */
export function RoundLauncher({ onStarted }: { onStarted: (round: Round) => void }) {
  const { content, storage, settings, updateSettings, t } = useApp();
  /** `null` once loaded with no round in progress. */
  const [active, setActive] = useState<Round | null>();
  const [starting, setStarting] = useState(false);
  const [confirmingAbandon, setConfirmingAbandon] = useState(false);

  useEffect(() => {
    void storage.rounds().then((all) => setActive(activeRound(all) ?? null));
  }, [storage]);

  if (active === undefined) return null;
  if (active !== null) {
    return (
      <div class="launcher">
        <h2>
          <Play size={20} aria-hidden="true" /> {t.roundHeading}
        </h2>
        <p class="launcher-text">
          {t.continueRoundDetail(
            courseName(active.course, t),
            roundPosition(active, active.entries.length),
          )}
        </p>
        <a class="button primary large" href={routeHref({ page: "round" })}>
          {t.continueRound}
          <ArrowRight size={18} aria-hidden="true" />
        </a>
        <button
          type="button"
          class="link-button abandon-round"
          onClick={() => setConfirmingAbandon(true)}
        >
          {t.abandonRound}
        </button>
        <Confirm
          open={confirmingAbandon}
          message={t.confirmAbandonRound}
          confirmLabel={t.confirmAbandonRoundYes}
          onCancel={() => setConfirmingAbandon(false)}
          onConfirm={() => {
            setConfirmingAbandon(false);
            void abandonActiveRound(storage, active).then(() => setActive(null));
          }}
        />
      </div>
    );
  }

  const courseSize = (course: Problem["difficulty"]) =>
    courseProblemIds(content.problems, course).length;
  const start = () => {
    setStarting(true);
    void startRound(storage, content.problems, settings)
      .then(onStarted)
      .finally(() => setStarting(false));
  };
  return (
    <div class="launcher">
      <h2>
        <Play size={20} aria-hidden="true" /> {t.roundHeading}
      </h2>
      <p class="launcher-text">{t.roundIntro(ROUND_LENGTH)}</p>
      <div class="course-options segmented" role="group" aria-label={t.courseHeading}>
        {DIFFICULTIES.map((course) => (
          <button
            key={course}
            type="button"
            aria-pressed={settings.course === course}
            disabled={courseSize(course) === 0}
            onClick={() => updateSettings({ ...settings, course })}
          >
            <strong>{t.difficulties[course]}</strong>
          </button>
        ))}
      </div>
      <button
        type="button"
        class="primary large"
        onClick={start}
        disabled={starting || courseSize(settings.course) === 0}
      >
        {t.startRound}
        <ArrowRight size={18} aria-hidden="true" />
      </button>
    </div>
  );
}
