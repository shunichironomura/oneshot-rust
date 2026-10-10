import { useEffect, useState } from "preact/hooks";
import { MODE_LABELS, MODES } from "../core/mode.ts";
import { KIND_LABELS } from "../core/problem.ts";
import { routeHref } from "../core/route.ts";
import type { Best } from "../core/run.ts";
import { titleFor } from "../core/scoring.ts";
import { useApp } from "./context.ts";

function Stars({ n }: { n: number }) {
  return (
    <span class="stars" title={`難易度 ${n}`}>
      {"★".repeat(n)}
      <span class="dim">{"★".repeat(5 - n)}</span>
    </span>
  );
}

export function ProblemList() {
  const { content, storage, settings } = useApp();
  const [bests, setBests] = useState<Best[]>([]);

  useEffect(() => {
    void storage.allBests().then(setBests);
  }, [storage]);

  const bestFor = (problemId: string, mode: (typeof MODES)[number]) =>
    bests.find((b) => b.problemId === problemId && b.mode === mode);

  return (
    <section class="problem-list">
      <h1>問題一覧</h1>
      <p class="hint">
        現在のモード: <strong>{MODE_LABELS[settings.mode]}</strong>
        （記録はモードごとに管理されます）
      </p>
      <table>
        <thead>
          <tr>
            <th>問題</th>
            <th>種類</th>
            <th>難易度</th>
            <th>Par</th>
            <th>概念</th>
            {MODES.map((m) => (
              <th key={m}>{MODE_LABELS[m]}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {content.problems.map((p) => (
            <tr key={p.id}>
              <td>
                <a href={routeHref({ page: "play", problemId: p.id })}>{p.title}</a>
                <div class="problem-id">{p.id}</div>
              </td>
              <td>{KIND_LABELS[p.kind]}</td>
              <td>
                <Stars n={p.difficulty} />
              </td>
              <td>{p.par}</td>
              <td>
                {p.concepts.map((c) => (
                  <span class="tag" key={c}>
                    {c}
                  </span>
                ))}
              </td>
              {MODES.map((m) => {
                const best = bestFor(p.id, m);
                return (
                  <td key={m} class="best">
                    {best === undefined ? (
                      <span class="dim">—</span>
                    ) : (
                      <span title={`CE ${best.ceCount} 回`}>
                        {best.submissions}回 <small>{titleFor(best.submissions, p.par)}</small>
                      </span>
                    )}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}
