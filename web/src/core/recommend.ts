/** What the landing page suggests playing next. */
export type Recommendation =
  | { kind: "continue"; problemId: string }
  | { kind: "start"; problemId: string }
  | { kind: "all-solved" };

/**
 * Prefers a problem with a run in progress, then the first unsolved problem in
 * the given order (problems are ordered by difficulty).
 */
export function recommend(
  problemIds: readonly string[],
  solved: ReadonlySet<string>,
  inProgress: ReadonlySet<string>,
): Recommendation {
  const continuing = problemIds.find((id) => inProgress.has(id));
  if (continuing !== undefined) return { kind: "continue", problemId: continuing };
  const next = problemIds.find((id) => !solved.has(id));
  return next === undefined ? { kind: "all-solved" } : { kind: "start", problemId: next };
}
