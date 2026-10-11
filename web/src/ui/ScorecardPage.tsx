import { ArrowLeft, RotateCcw } from "lucide-preact";
import { useEffect, useMemo, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import { type Round, roundResult, roundShareText, totalScore } from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import { roundShareCard } from "../core/share.ts";
import { useApp } from "./context.ts";
import { DifficultyMarks } from "./DifficultyBadge.tsx";
import { IconButton } from "./IconButton.tsx";
import { startRound } from "./RoundLauncher.tsx";
import { type Score, ScoreMark } from "./ScoreMark.tsx";
import { ShareButton } from "./ShareButton.tsx";
import { ShotMeter } from "./ShotMeter.tsx";

/**
 * A golf scorecard: one column per hole, the shots written by hand and marked against par.
 * Right after the round (`fresh`) it offers one more; opened from the records, the way back.
 */
function Scorecard({ round, fresh }: { round: Round; fresh: boolean }) {
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

  const holes = round.problemIds.map((problemId, i) => {
    const entry = round.entries[i];
    const problem = content.problems.find((p) => p.id === problemId);
    const score: Score =
      entry === undefined
        ? { kind: "empty" }
        : entry.solved
          ? { kind: "solved", shots: entry.verdicts.length }
          : { kind: "unsolved" };
    return { key: `${i}:${problemId}`, seed: `${round.id}:${i}`, problemId, problem, entry, score };
  });
  const count = round.problemIds.length;

  return (
    <section class="scorecard">
      {!fresh && (
        <div class="section-head">
          <IconButton
            icon={ArrowLeft}
            label={t.backToRecords}
            href={routeHref({ page: "problems" })}
          />
        </div>
      )}
      <p class="run-date">{dateFormat.format(round.startedAt)}</p>
      <h1 class="scorecard-result">{roundResult(round, t)}</h1>
      <div class="card-scroll">
        <table class="card">
          <thead>
            <tr>
              <th scope="row">{t.card.hole}</th>
              {holes.map((h, i) => (
                <th key={h.key} scope="col">
                  {i + 1}
                </th>
              ))}
              <th scope="col">{t.card.total}</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row">{t.card.problem}</th>
              {holes.map(({ key, problemId, problem, entry }) => (
                <td key={key} class="name">
                  {entry === undefined ? (
                    // Not reached (the round was abandoned): it may come up in a later round.
                    <span class="dim">—</span>
                  ) : problem === undefined ? (
                    problemId
                  ) : (
                    <a href={routeHref({ page: "history", problemId })}>
                      {localize(problem.title, locale)}
                    </a>
                  )}
                </td>
              ))}
              <td />
            </tr>
            <tr>
              <th scope="row">{t.card.level}</th>
              {holes.map(({ key, problem }) => (
                <td key={key}>
                  {problem !== undefined && (
                    <span title={t.difficulties[problem.difficulty]}>
                      <DifficultyMarks difficulty={problem.difficulty} />
                    </span>
                  )}
                </td>
              ))}
              <td />
            </tr>
            <tr>
              <th scope="row">{t.card.par}</th>
              {holes.map(({ key }) => (
                <td key={key}>1</td>
              ))}
              <td class="total">{count}</td>
            </tr>
            <tr>
              <th scope="row">{t.card.shots}</th>
              {holes.map(({ key, seed, entry, score }) => (
                <td key={key} class="score">
                  <ScoreMark score={score} seed={seed} />
                  {entry !== undefined && entry.verdicts.length > 0 && (
                    <ShotMeter shots={entry.verdicts} label={t.shareTries(entry.verdicts.length)} />
                  )}
                </td>
              ))}
              <td class="score total">
                <ScoreMark score={totalScore(round)} seed={round.id} />
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="actions">
        {fresh && (
          <button type="button" class="primary icon-button" onClick={oneMore} disabled={starting}>
            <RotateCcw size={16} aria-hidden="true" />
            {t.oneMoreRound}
          </button>
        )}
        <ShareButton text={text} card={card} fileName={`oneshot-rust-round-${round.id}`} />
      </div>
    </section>
  );
}

/** The results of a finished round, to share and (right after finishing it) to play again. */
export function ScorecardPage({ roundId, fresh }: { roundId: string; fresh: boolean }) {
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
  return <Scorecard round={round} fresh={fresh} />;
}
