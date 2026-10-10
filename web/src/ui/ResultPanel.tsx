import type { CaseStatus } from "../core/classify.ts";
import { type Presentation, VERDICT_LABELS } from "../core/present.ts";
import type { LastResult, SubmitPhase } from "./usePlay.ts";

const CASE_LABELS: Record<CaseStatus, string> = { pass: "PASS", fail: "FAIL", "not-run": "未実行" };

function Running({ phase }: { phase: Extract<SubmitPhase, { kind: "running" }> }) {
  const p = phase.progress;
  return (
    <div class="result running">
      <div class="verdict-line">
        <span class="spinner" aria-hidden="true" />
        {p.phase === "compiling"
          ? "コンパイル中…"
          : `実行中（ケース ${Math.min(p.done + 1, p.total)}/${p.total}）`}
      </div>
      {phase.output !== "" && <Output stdout={phase.output} stderr="" />}
    </div>
  );
}

function Output({ stdout, stderr }: { stdout: string; stderr: string }) {
  if (stdout === "" && stderr === "") return null;
  return (
    <details class="output" open>
      <summary>出力</summary>
      {stdout !== "" && <pre class="stdout">{stdout}</pre>}
      {stderr !== "" && <pre class="stderr">{stderr}</pre>}
    </details>
  );
}

function Cases({ cases }: { cases: NonNullable<Presentation["cases"]> }) {
  return (
    <table class="cases">
      <tbody>
        {cases.map((c) => (
          <tr key={c.id} class={c.status}>
            <td class="case-id">
              {c.id}
              {!c.visible && <span class="dim"> (非公開)</span>}
            </td>
            <td class="case-status">{CASE_LABELS[c.status]}</td>
            <td class="case-message">{c.message !== undefined && <code>{c.message}</code>}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

interface Props {
  phase: SubmitPhase;
  last: LastResult | null;
  presentation: Presentation | null;
}

export function ResultPanel({ phase, last, presentation }: Props) {
  if (phase.kind === "running") return <Running phase={phase} />;
  if (last === null) {
    return <div class="result empty">Ctrl+Enter（⌘+Enter）で提出します。一発で通しましょう。</div>;
  }
  if (last.kind === "rejected") {
    return (
      <div class="result">
        <div class="verdict-line">提出できません（カウントされません）</div>
        <p>{last.message}</p>
      </div>
    );
  }
  if (last.kind === "error") {
    return (
      <div class="result">
        <div class="verdict-line verdict IE">エラー（カウントされません）</div>
        <p>{last.message}</p>
      </div>
    );
  }
  if (presentation === null) return null;
  return (
    <div class="result">
      <div class="verdict-line">
        <span class={`verdict ${presentation.kind}`}>{presentation.kind}</span>
        <span>{VERDICT_LABELS[presentation.kind]}</span>
      </div>
      {presentation.summary !== undefined && <p class="summary">{presentation.summary}</p>}
      {presentation.diagnostics.map((d, i) => (
        <pre class="diagnostic" key={i}>
          {d}
        </pre>
      ))}
      {presentation.cases !== null && <Cases cases={presentation.cases} />}
      <Output stdout={presentation.output.stdout} stderr={presentation.output.stderr} />
    </div>
  );
}
