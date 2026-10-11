import solutions from "virtual:oneshot-solutions";
import { useEffect, useState } from "preact/hooks";
import { localize } from "../core/i18n.ts";
import type { ModelSolution as Solution, Problem } from "../core/problem.ts";
import { CodeView } from "./CodeView.tsx";
import { useApp } from "./context.ts";
import { DiffView } from "./DiffView.tsx";

type Loaded =
  | { kind: "loading" }
  | { kind: "ok"; solution: Solution }
  | { kind: "error"; message: string };

function useSolution(problemId: string): Loaded {
  const [loaded, setLoaded] = useState<Loaded>({ kind: "loading" });
  useEffect(() => {
    let cancelled = false;
    setLoaded({ kind: "loading" });
    const load = solutions[problemId];
    void (
      load === undefined ? Promise.reject(new Error(`unknown problem ${problemId}`)) : load()
    ).then(
      (solution) => !cancelled && setLoaded({ kind: "ok", solution }),
      (e: unknown) => !cancelled && setLoaded({ kind: "error", message: String(e) }),
    );
    return () => {
      cancelled = true;
    };
  }, [problemId]);
  return loaded;
}

/** The model solution with its editorial, and a diff from the player's code. */
export function ModelSolution({ problem, code }: { problem: Problem; code: string }) {
  const { locale, t } = useApp();
  const loaded = useSolution(problem.id);
  const [view, setView] = useState<"code" | "diff">("code");
  if (loaded.kind === "loading") return <p class="solution-note">{t.solutionLoading}</p>;
  if (loaded.kind === "error")
    return <p class="solution-note">{t.solutionLoadError(loaded.message)}</p>;
  const { solution } = loaded;
  return (
    <>
      <div class="segmented" role="group">
        <button type="button" aria-pressed={view === "code"} onClick={() => setView("code")}>
          {t.solutionCode}
        </button>
        <button type="button" aria-pressed={view === "diff"} onClick={() => setView("diff")}>
          {t.solutionDiff}
        </button>
      </div>
      {view === "code" ? (
        <CodeView code={solution.code} />
      ) : (
        <>
          <p class="solution-note">{t.solutionDiffLegend}</p>
          <DiffView before={code} after={solution.code} />
        </>
      )}
      {solution.editorialHtml !== undefined && (
        <div
          class="markdown"
          // editorialHtml is rendered at build time from the repository's own Markdown.
          dangerouslySetInnerHTML={{ __html: localize(solution.editorialHtml, locale) }}
        />
      )}
    </>
  );
}
