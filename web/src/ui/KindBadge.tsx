import { Braces, type LucideIcon, PenLine, Wrench } from "lucide-preact";
import type { ProblemKind } from "../core/problem.ts";
import { useApp } from "./context.ts";

const KIND_ICONS: Record<ProblemKind, LucideIcon> = {
  implement: PenLine,
  repair: Wrench,
  signature: Braces,
};

export function KindBadge({ kind }: { kind: ProblemKind }) {
  const { t } = useApp();
  const Icon = KIND_ICONS[kind];
  return (
    <span class={`badge kind-${kind}`}>
      <Icon size={13} aria-hidden="true" />
      {t.kinds[kind]}
    </span>
  );
}

export function Difficulty({ n }: { n: number }) {
  const { t } = useApp();
  return (
    <span class="difficulty" title={t.difficulty(n)} aria-label={t.difficulty(n)}>
      {[1, 2, 3, 4, 5].map((i) => (
        <span key={i} class={i <= n ? "dot on" : "dot"} />
      ))}
    </span>
  );
}
