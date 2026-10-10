import { EyeOff } from "lucide-preact";
import type { Presentation } from "../core/present.ts";
import { useApp } from "./context.ts";
import type { LastResult, SubmitPhase } from "./usePlay.ts";

function Running({ phase }: { phase: Extract<SubmitPhase, { kind: "running" }> }) {
  const { t } = useApp();
  const p = phase.progress;
  return (
    <div class="result running">
      <div class="verdict-line">
        <span class="spinner" aria-hidden="true" />
        {p.phase === "compiling"
          ? t.compiling
          : t.runningCase(Math.min(p.done + 1, p.total), p.total)}
      </div>
      <Output stdout={phase.output} stderr="" />
    </div>
  );
}

function Output({ stdout, stderr }: { stdout: string; stderr: string }) {
  const { t } = useApp();
  if (stdout === "" && stderr === "") return null;
  return (
    <details class="output" open>
      <summary>{t.output}</summary>
      {stdout !== "" && <pre class="stdout">{stdout}</pre>}
      {stderr !== "" && <pre class="stderr">{stderr}</pre>}
    </details>
  );
}

function Cases({ cases }: { cases: NonNullable<Presentation["cases"]> }) {
  const { t } = useApp();
  return (
    <ul class="cases">
      {cases.map((c) => (
        <li key={c.id} class={c.status}>
          <span class="case-status">{t.caseStatus[c.status]}</span>
          <span class="case-id">
            {c.id}
            {!c.visible && <EyeOff size={13} aria-label={t.hiddenCase} class="hidden-case" />}
          </span>
          {c.message !== undefined && <code class="case-message">{c.message}</code>}
        </li>
      ))}
    </ul>
  );
}

interface Props {
  phase: SubmitPhase;
  last: LastResult | null;
  presentation: Presentation | null;
}

export function ResultPanel({ phase, last, presentation }: Props) {
  const { t } = useApp();
  if (phase.kind === "running") return <Running phase={phase} />;
  if (last === null) return <div class="result empty">{t.resultEmpty}</div>;
  if (last.kind === "rejected") {
    return (
      <div class="result">
        <div class="verdict-line">{t.rejected}</div>
        <p>{t.forbiddenMacro(last.forbiddenMacro)}</p>
      </div>
    );
  }
  if (last.kind === "error") {
    return (
      <div class="result">
        <div class="verdict-line">
          <span class="verdict IE">IE</span>
          {t.submitError}
        </div>
        <p>{last.message}</p>
      </div>
    );
  }
  if (presentation === null) return null;
  return (
    <div class="result">
      <div class="verdict-line">
        <span class={`verdict ${presentation.kind}`}>{presentation.kind}</span>
        <span>{t.verdicts[presentation.kind]}</span>
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
