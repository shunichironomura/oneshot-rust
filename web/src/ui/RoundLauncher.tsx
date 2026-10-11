import { ArrowRight, Play } from "lucide-preact";
import { useEffect, useState } from "preact/hooks";
import { DIFFICULTIES, type Problem } from "../core/problem.ts";
import {
  activeRound,
  courseName,
  courseProblemIds,
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
import { ModeOptions } from "./ModeOptions.tsx";

/**
 * Starts a round with the course and rules of the settings, unless one is already in
 * progress: then that one is returned.
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
    mode: settings.mode,
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

/**
 * The way into the game: "Continue" when a round is in progress, and otherwise the course
 * (with the feedback mode under a disclosure) and "Start round".
 */
export function RoundLauncher({ onStarted }: { onStarted: (round: Round) => void }) {
  const { content, storage, settings, updateSettings, t } = useApp();
  /** `null` once loaded with no round in progress. */
  const [active, setActive] = useState<Round | null>();
  const [starting, setStarting] = useState(false);

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
            <span>{t.courseSize(courseSize(course))}</span>
          </button>
        ))}
      </div>
      <details class="more-options">
        <summary>
          {t.modeHeading}: {t.modes[settings.mode]}
        </summary>
        <ModeOptions />
      </details>
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
