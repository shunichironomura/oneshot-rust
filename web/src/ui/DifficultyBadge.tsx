import { DIFFICULTIES, type Difficulty } from "../core/problem.ts";
import { useApp } from "./context.ts";

/**
 * A difficulty as one slot per tier, filled up to it. Drawn in the text color: green,
 * amber and red are kept for verdicts. Decorative; the label or an aria-label says it.
 */
export function DifficultyMarks({ difficulty }: { difficulty: Difficulty }) {
  return (
    <span class="marks" aria-hidden="true">
      {DIFFICULTIES.map((d) => (
        <i key={d} class={d <= difficulty ? "on" : undefined} />
      ))}
    </span>
  );
}

/** A difficulty (or course): its marks and its name. */
export function DifficultyBadge({ difficulty }: { difficulty: Difficulty }) {
  const { t } = useApp();
  return (
    <span class="badge level">
      <DifficultyMarks difficulty={difficulty} />
      {t.difficulties[difficulty]}
    </span>
  );
}
