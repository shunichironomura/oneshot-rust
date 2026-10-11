import { RotateCcw } from "lucide-preact";
import { useEffect, useMemo, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { type Round, roundResult, roundShareText, totalSubmissions } from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import { roundShareCard } from "../core/share.ts";
import { useApp } from "./context.ts";
import { DifficultyBadge } from "./DifficultyBadge.tsx";
import { startRound } from "./RoundLauncher.tsx";
import { ShareButton } from "./ShareButton.tsx";
import { ShotMeter } from "./ShotMeter.tsx";

function Scorecard({ round }: { round: Round }) {
  const { content, storage, locale, t } = useApp();
  const [starting, setStarting] = useState(false);
  const text = roundShareText(round, t);
  const card = useMemo(() => {
    const date = new Intl.DateTimeFormat(locale, { dateStyle: "medium" }).format(round.startedAt);
    return roundShareCard(round, date, t);
  }, [round, locale, t]);
  const dateFormat = new Intl.DateTimeFormat(locale, { dateStyle: "medium", timeStyle: "short" });

  const oneMore = () => {
    setStarting(true);
    void startRound(storage, content.problems).then(() => {
      window.location.hash = routeHref({ page: "round" });
    });
  };

  return (
    <section class="scorecard">
      <p class="run-date">{dateFormat.format(round.startedAt)}</p>
      <h1 class="scorecard-total">
        {t.roundTotal(totalSubmissions(round))}
        <span>{roundResult(round, t)}</span>
      </h1>
      <ol class="scorecard-rows">
        {round.problemIds.map((problemId, i) => {
          const entry = round.entries[i];
          const problem = content.problems.find((p) => p.id === problemId);
          const n = entry?.verdicts.length ?? 0;
          return (
            <li key={`${i}:${problemId}`} class={entry === undefined ? "dim" : undefined}>
              <span class="row-number">{i + 1}</span>
              {entry === undefined ? (
                // Not reached (the round was abandoned): it may come up in a later round.
                <span>—</span>
              ) : problem === undefined ? (
                <span>{problemId}</span>
              ) : (
                <a href={routeHref({ page: "history", problemId })}>
                  {localize(problem.title, locale)}
                </a>
              )}
              {problem !== undefined && <DifficultyBadge difficulty={problem.difficulty} />}
              <span class="scorecard-shots">
                {n > 0 && <ShotMeter shots={entry?.verdicts} label={t.shareTries(n)} />}
                {entry !== undefined && <span class="shot-count">{n}</span>}
                {entry?.solved === false && <span class="unsolved-mark">✗</span>}
              </span>
            </li>
          );
        })}
      </ol>
      <div class="actions">
        <button type="button" class="primary icon-button" onClick={oneMore} disabled={starting}>
          <RotateCcw size={16} aria-hidden="true" />
          {t.oneMoreRound}
        </button>
        <ShareButton text={text} card={card} fileName={`oneshot-rust-round-${round.id}`} />
      </div>
    </section>
  );
}

/** The results of a finished round, to share and to play again. */
export function ScorecardPage({ roundId }: { roundId: string }) {
  const { storage, t } = useApp();
  /** `null` once loaded when the round does not exist. */
  const [round, setRound] = useState<Round | null>();

  useEffect(() => {
    void storage.round(roundId).then((r) => setRound(r ?? null));
  }, [storage, roundId]);

  if (round === undefined) return null;
  if (round === null) return <p>{t.roundNotFound}</p>;
  if (round.finishedAt === undefined) {
    return (
      <section class="scorecard">
        <p>{t.roundUnfinished}</p>
        <a class="button primary" href={routeHref({ page: "round" })}>
          {t.continueRound}
        </a>
      </section>
    );
  }
  return <Scorecard round={round} />;
}
