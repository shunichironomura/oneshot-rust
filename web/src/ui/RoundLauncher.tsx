import { ArrowRight, Play } from "lucide-preact";
import { useEffect, useState } from "preact/hooks";
import { DIFFICULTIES, type Difficulty, type Problem } from "../core/problem.ts";
import {
  abandonRound,
  activeRound,
  currentProblemId,
  newRound,
  type PlayedProblems,
  pickProblem,
  pickRound,
  ROUND_LADDER,
  type Round,
  seenProblemIds,
} from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import type { Storage } from "../storage/db.ts";
import { useApp } from "./context.ts";
import { Confirm } from "./Dialog.tsx";
import { DifficultyBadge, DifficultyMarks } from "./DifficultyBadge.tsx";
import { RoundProgress } from "./RoundProgress.tsx";

/** What the player has done so far: problems shown in rounds and problems played at all count as seen. */
async function playedProblems(storage: Storage, rounds: readonly Round[]): Promise<PlayedProblems> {
  const [bests, played] = await Promise.all([storage.allBests(), storage.problemsPlayed()]);
  return {
    solved: new Set(bests.map((b) => b.problemId)),
    seen: new Set([...seenProblemIds(rounds), ...played]),
  };
}

/** Starts a round, unless one is already in progress: then that one is returned. */
export async function startRound(storage: Storage, problems: readonly Problem[]): Promise<Round> {
  const rounds = await storage.rounds();
  const active = activeRound(rounds);
  if (active !== undefined) return active;
  const played = await playedProblems(storage, rounds);
  const round = newRound({
    id: crypto.randomUUID(),
    problemIds: pickRound(problems, played, Math.random),
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
 * Picks a problem of the given difficulty to play on its own, outside a round. The problems
 * of a round in progress are left out, so that its problems still to come are not met early.
 */
async function pickQuickProblem(
  storage: Storage,
  problems: readonly Problem[],
  difficulty: Difficulty,
): Promise<string | undefined> {
  const rounds = await storage.rounds();
  const inRound = new Set(activeRound(rounds)?.problemIds);
  const played = await playedProblems(storage, rounds);
  return pickProblem(
    problems.filter((p) => !inRound.has(p.id)),
    played,
    difficulty,
    Math.random,
  );
}

/** A single problem of a chosen difficulty, played on its own as practice. */
function QuickPlay() {
  const { content, storage, t } = useApp();
  const [picking, setPicking] = useState(false);
  // Only the difficulties there are problems of.
  const difficulties = DIFFICULTIES.filter((d) => content.problems.some((p) => p.difficulty === d));
  if (difficulties.length === 0) return null;

  const play = (difficulty: Difficulty) => {
    setPicking(true);
    void pickQuickProblem(storage, content.problems, difficulty)
      .then((problemId) => {
        if (problemId !== undefined)
          window.location.hash = routeHref({ page: "practice", problemId });
      })
      .finally(() => setPicking(false));
  };
  return (
    <p class="quick-play">
      {t.quickPlay}
      {difficulties.map((d) => (
        <button key={d} type="button" class="level" onClick={() => play(d)} disabled={picking}>
          <DifficultyMarks difficulty={d} />
          {t.difficulties[d]}
        </button>
      ))}
    </p>
  );
}

/**
 * The way into the game: "Play", or "Continue" with the progress of the round in progress
 * (which can also be abandoned). Either way, a single problem of a chosen difficulty can
 * also be played on its own, as practice.
 */
export function RoundLauncher({ onStarted }: { onStarted: (round: Round) => void }) {
  const { content, storage, t } = useApp();
  /** `null` once loaded with no round in progress. */
  const [active, setActive] = useState<Round | null>();
  const [starting, setStarting] = useState(false);
  const [confirmingAbandon, setConfirmingAbandon] = useState(false);

  useEffect(() => {
    void storage.rounds().then((all) => setActive(activeRound(all) ?? null));
  }, [storage]);

  if (active === undefined) return null;
  if (active !== null) {
    const position = active.entries.length;
    return (
      <div class="launcher">
        <div class="launcher-strip">
          <RoundProgress
            round={active}
            current={position}
            label={t.roundPosition(position + 1, active.problemIds.length)}
          />
        </div>
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
        <QuickPlay />
      </div>
    );
  }

  const start = () => {
    setStarting(true);
    void startRound(storage, content.problems)
      .then(onStarted)
      .finally(() => setStarting(false));
  };
  return (
    <div class="launcher">
      <button
        type="button"
        class="primary large"
        onClick={start}
        disabled={starting || content.problems.length === 0}
      >
        <Play size={18} aria-hidden="true" />
        {t.play}
      </button>
      <p class="round-ladder">
        {t.roundLadder(ROUND_LADDER.length)}
        {ROUND_LADDER.map((d, i) => (
          // An arrow stays with the badge after it when the line wraps.
          <span key={d} class="ladder-step">
            {i > 0 && <span aria-hidden="true">→</span>}
            <DifficultyBadge difficulty={d} />
          </span>
        ))}
      </p>
      <QuickPlay />
    </div>
  );
}
