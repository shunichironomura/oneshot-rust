import { useCallback, useEffect, useMemo, useState } from "preact/hooks";
import { activeRound, type Round, recordRun, skipCurrent } from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import type { Run } from "../core/run.ts";
import { useApp } from "./context.ts";
import { PlayView, type Session } from "./PlayPage.tsx";
import { RoundLauncher } from "./RoundLauncher.tsx";

function showScorecard(round: Round): void {
  window.location.hash = routeHref({ page: "scorecard", roundId: round.id });
}

/** The round in progress, one problem after another; the launcher when there is none. */
export function RoundPage() {
  const { content, storage, t } = useApp();
  /** `null` once loaded with no round in progress. */
  const [round, setRound] = useState<Round | null>();
  /**
   * Whether to keep showing the problem that was just finished (to review the result)
   * instead of the next one. The shown problem is derived from the recorded entries,
   * so moving on twice cannot skip a problem.
   */
  const [lingering, setLingering] = useState(false);

  useEffect(() => {
    void storage.rounds().then((all) => setRound(activeRound(all) ?? null));
  }, [storage]);

  useEffect(() => {
    if (round !== undefined && round !== null) void storage.saveRound(round);
  }, [round, storage]);

  const update = useCallback(
    (f: (round: Round) => Round) => setRound((r) => (r === undefined || r === null ? r : f(r))),
    [],
  );

  const onRunFinished = useCallback(
    (run: Run) => {
      update((r) => recordRun(r, run));
      setLingering(true);
    },
    [update],
  );

  const shownIndex =
    round === undefined || round === null
      ? 0
      : Math.max(0, round.entries.length - (lingering ? 1 : 0));

  const session = useMemo((): Session | undefined => {
    if (round === undefined || round === null) return undefined;
    const last = shownIndex === round.problemIds.length - 1;
    return {
      kind: "round",
      round,
      index: shownIndex,
      onRunFinished,
      next: last
        ? { label: t.toScorecard, action: () => showScorecard(round) }
        : { label: t.nextProblem, action: () => setLingering(false) },
    };
  }, [round, shownIndex, onRunFinished, t]);

  if (round === undefined) return null;
  if (round === null || session === undefined) {
    return (
      <section class="round-start">
        <RoundLauncher onStarted={setRound} />
      </section>
    );
  }
  const problemId = round.problemIds[shownIndex];
  if (problemId === undefined) {
    // Finished: the scorecard has the results.
    return (
      <section class="round-start">
        <p>
          <a href={routeHref({ page: "scorecard", roundId: round.id })}>{t.toScorecard}</a>
        </p>
      </section>
    );
  }
  const problem = content.problems.find((p) => p.id === problemId);
  if (problem === undefined) {
    return (
      <section class="round-start">
        <p>{t.roundSkipDeleted(problemId)}</p>
        <button type="button" onClick={() => update((r) => skipCurrent(r, Date.now()))}>
          {t.nextProblem}
        </button>
      </section>
    );
  }
  return <PlayView key={`${round.id}:${problem.id}`} problem={problem} session={session} />;
}
