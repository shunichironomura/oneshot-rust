// A shot count as it would be written on a golf scorecard: by hand, in pencil, and marked
// against par. Par is one shot. One shot is circled (the hole in one); over par gets a box,
// and two or more over a double box. A hole not finished gets a cross.

/** What is written in the cell. */
export type Score =
  /** Marked against par, which is one shot unless given (a total has the holes' sum). */
  | { kind: "solved"; shots: number; par?: number }
  /** A number with no mark: a total of a round not finished. */
  | { kind: "plain"; shots: number }
  | { kind: "unsolved" }
  /** Not played yet: an empty cell. */
  | { kind: "empty" };

interface Props {
  score: Score;
  /** Decides the wobble of the lines, so that a mark keeps its shape from render to render. */
  seed: string;
  /** Just written: the lines are drawn in front of the viewer. */
  fresh?: boolean;
}

type Point = [number, number];

/** Pencil marks around a digit, in a box of 0–40 with the digit at its center. */
const CENTER = 20;

function hash(text: string): number {
  // FNV-1a, enough to spread seeds apart.
  let h = 0x811c9dc5;
  for (const ch of text) {
    h ^= ch.codePointAt(0) ?? 0;
    h = Math.imul(h, 0x01000193);
  }
  return h >>> 0;
}

/** A small deterministic generator (mulberry32) of numbers in [0, 1). */
function generator(seed: number): () => number {
  let state = seed;
  return () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let t = state;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const fixed = (n: number) => n.toFixed(1);

/** A smooth open curve through the points (Catmull–Rom, as cubic Béziers). */
function curve(points: readonly Point[]): string {
  const at = (i: number) => points[Math.min(Math.max(i, 0), points.length - 1)];
  const segments = points.slice(0, -1).map((_, i) => {
    const [p0, p1, p2, p3] = [at(i - 1), at(i), at(i + 1), at(i + 2)];
    const c1: Point = [p1[0] + (p2[0] - p0[0]) / 6, p1[1] + (p2[1] - p0[1]) / 6];
    const c2: Point = [p2[0] - (p3[0] - p1[0]) / 6, p2[1] - (p3[1] - p1[1]) / 6];
    return `C${fixed(c1[0])} ${fixed(c1[1])} ${fixed(c2[0])} ${fixed(c2[1])} ${fixed(p2[0])} ${fixed(p2[1])}`;
  });
  return `M${fixed(points[0][0])} ${fixed(points[0][1])}${segments.join("")}`;
}

/** A ring drawn in one go, a little past where it started, never quite round. */
function circle(random: () => number): string {
  const rx = 13.5 * (0.94 + random() * 0.12);
  const ry = 12.5 * (0.94 + random() * 0.12);
  const tilt = (random() - 0.5) * 0.5;
  const start = random() * Math.PI * 2;
  const steps = 28;
  const points: Point[] = Array.from({ length: steps + 1 }, (_, i) => {
    const a = start + (i / steps) * Math.PI * 2 * 1.12;
    const wobble = 1 + (random() - 0.5) * 0.05;
    const x = Math.cos(a) * rx * wobble;
    const y = Math.sin(a) * ry * wobble;
    return [
      CENTER + x * Math.cos(tilt) - y * Math.sin(tilt),
      CENTER + x * Math.sin(tilt) + y * Math.cos(tilt),
    ];
  });
  return curve(points);
}

/** A box of four strokes, each run a little past the corner, as a quick hand does. */
function box(random: () => number, half: number): string {
  const jitter = () => (random() - 0.5) * 1.6;
  const corners: Point[] = [
    [CENTER - half + jitter(), CENTER - half + jitter()],
    [CENTER + half + jitter(), CENTER - half + jitter()],
    [CENTER + half + jitter(), CENTER + half + jitter()],
    [CENTER - half + jitter(), CENTER + half + jitter()],
  ];
  return corners
    .map((from, i) => {
      const to = corners[(i + 1) % 4];
      const dx = to[0] - from[0];
      const dy = to[1] - from[1];
      const length = Math.hypot(dx, dy);
      const over = () => (0.6 + random() * 1.4) / length;
      const a: Point = [from[0] - dx * over(), from[1] - dy * over()];
      const b: Point = [to[0] + dx * over(), to[1] + dy * over()];
      // Bowed a touch, across the stroke.
      const bow = (random() - 0.5) * 1.6;
      const mid: Point = [
        (a[0] + b[0]) / 2 - (dy / length) * bow,
        (a[1] + b[1]) / 2 + (dx / length) * bow,
      ];
      return `M${fixed(a[0])} ${fixed(a[1])}Q${fixed(mid[0])} ${fixed(mid[1])} ${fixed(b[0])} ${fixed(b[1])}`;
    })
    .join("");
}

/** Two strokes, corner to corner. */
function cross(random: () => number): string {
  const stroke = (from: Point, to: Point) => {
    const bow = (random() - 0.5) * 2;
    const mid: Point = [(from[0] + to[0]) / 2 + bow, (from[1] + to[1]) / 2 - bow];
    return `M${fixed(from[0])} ${fixed(from[1])}Q${fixed(mid[0])} ${fixed(mid[1])} ${fixed(to[0])} ${fixed(to[1])}`;
  };
  const j = () => (random() - 0.5) * 2;
  return (
    stroke([11 + j(), 11 + j()], [29 + j(), 29 + j()]) +
    stroke([29 + j(), 11 + j()], [11 + j(), 29 + j()])
  );
}

/** The marks for a score, in the order they are drawn. */
export function marksFor(score: Score, seed: string): string[] {
  const random = generator(hash(seed));
  switch (score.kind) {
    case "empty":
    case "plain":
      return [];
    case "unsolved":
      return [cross(random)];
    case "solved": {
      const over = score.shots - (score.par ?? 1);
      if (over <= 0) return [circle(random)];
      if (over === 1) return [box(random, 12.5)];
      return [box(random, 16), box(random, 12)];
    }
  }
}

export function ScoreMark({ score, seed, fresh = false }: Props) {
  const marks = marksFor(score, seed);
  return (
    <span class={`score-mark${fresh ? " fresh" : ""}`} aria-hidden="true">
      {(score.kind === "solved" || score.kind === "plain") && (
        <span class="score-digit">{score.shots}</span>
      )}
      {marks.length > 0 && (
        <svg viewBox="0 0 40 40">
          {marks.map((d, i) => (
            <path key={i} d={d} pathLength={1} />
          ))}
        </svg>
      )}
    </span>
  );
}
