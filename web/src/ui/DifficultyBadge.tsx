import type { Difficulty } from "../core/problem.ts";
import { useApp } from "./context.ts";

/** A difficulty (or course), colored from green (easy) to red (hard). */
export function DifficultyBadge({ difficulty }: { difficulty: Difficulty }) {
  const { t } = useApp();
  return <span class={`badge level difficulty-${difficulty}`}>{t.difficulties[difficulty]}</span>;
}
