import { RotateCcw } from "lucide-preact";
import { useEffect, useMemo, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import {
  courseName,
  type Round,
  roundResult,
  roundShareText,
  totalSubmissions,
} from "../core/round.ts";
import { routeHref } from "../core/route.ts";
import { verdictEmoji } from "../core/scoring.ts";
import { roundShareCard } from "../core/share.ts";
import { useApp } from "./context.ts";
import { startRound } from "./RoundLauncher.tsx";
import { ShareButtons } from "./ShareButtons.tsx";

function Scorecard({ round }: { round: Round }) {
  const { content, storage, settings, locale, t } = useApp();
  const [starting, setStarting] = useState(false);
  const text = roundShareText(round, t);
  const card = useMemo(() => roundShareCard(round, t), [round, t]);
  const dateFormat = new Intl.DateTimeFormat(locale, { dateStyle: "medium", timeStyle: "short" });

  const oneMore = () => {
    setStarting(true);
    void startRound(storage, content.problems, settings).then(() => {
      window.location.hash = routeHref({ page: "round" });
    });
  };

  return (
    <section class="scorecard">
      <h1>{t.scorecardTitle}</h1>
      <p class="scorecard-meta">
        {t.courseTitle(courseName(round.course, t))}
        <span class="badge">{t.modes[round.mode]}</span>
        <span class="run-date">{dateFormat.format(round.startedAt)}</span>
      </p>
      <p class="scorecard-total">
        <strong>{t.roundTotal(totalSubmissions(round))}</strong>
        <span>{roundResult(round, t)}</span>
      </p>
      <table class="scorecard-table">
        <thead>
          <tr>
            <th>#</th>
            <th>{t.columnProblem}</th>
            <th aria-hidden="true" />
            <th>{t.columnTries}</th>
          </tr>
        </thead>
        <tbody>
          {round.entries.map((e, i) => {
            const problem = content.problems.find((p) => p.id === e.problemId);
            return (
              <tr key={e.problemId}>
                <td class="dim">{i + 1}</td>
                <td>
                  {problem === undefined ? (
                    e.problemId
                  ) : (
                    <a href={routeHref({ page: "history", problemId: e.problemId })}>
                      {localize(problem.title, locale)}
                    </a>
                  )}
                </td>
                <td class="squares">{e.verdicts.map(verdictEmoji).join("") || "—"}</td>
                <td>
                  {e.verdicts.length}
                  {!e.solved && <span class="dim"> ✗</span>}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <pre class="share">{text}</pre>
      <div class="actions">
        <button type="button" class="primary icon-button" onClick={oneMore} disabled={starting}>
          <RotateCcw size={16} aria-hidden="true" />
          {t.oneMoreRound}
        </button>
        <ShareButtons text={text} card={card} fileName={`oneshot-rust-round-${round.id}`} />
        <a class="button" href={routeHref({ page: "home" })}>
          {t.changeCourse}
        </a>
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
