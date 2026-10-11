import { ArrowRight, Play } from "lucide-preact";
import { Fragment } from "preact";
import { useEffect, useState } from "preact/hooks";
import type { Problem } from "../core/problem.ts";
import {
  abandonRound,
  activeRound,
  currentProblemId,
  newRound,
  pickRound,
  ROUND_LADDER,
  type Round,
  seenProblemIds,
} from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import type { Storage } from "../storage/db.ts";
import { useApp } from "./context.ts";
import { Confirm } from "./Dialog.tsx";
import { DifficultyBadge } from "./DifficultyBadge.tsx";
import { RoundProgress } from "./RoundProgress.tsx";

/** Starts a round, unless one is already in progress: then that one is returned. */
export async function startRound(storage: Storage, problems: readonly Problem[]): Promise<Round> {
  const [rounds, bests] = await Promise.all([storage.rounds(), storage.allBests()]);
  const active = activeRound(rounds);
  if (active !== undefined) return active;
  const played = { solved: new Set(bests.map((b) => b.problemId)), seen: seenProblemIds(rounds) };
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
 * The way into the game: "Play", or "Continue" with the progress of the round in progress
 * (which can also be abandoned).
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
        <RoundProgress
          round={active}
          current={position}
          label={t.roundPosition(position + 1, active.problemIds.length)}
        />
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
          <Fragment key={d}>
            {i > 0 && <span aria-hidden="true">→</span>}
            <DifficultyBadge difficulty={d} />
          </Fragment>
        ))}
      </p>
    </div>
  );
}
