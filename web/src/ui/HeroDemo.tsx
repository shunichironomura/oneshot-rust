import { Play } from "lucide-preact";
import { useEffect, useState } from "preact/hooks";
import { titleFor } from "../core/scoring.ts";
import { CodeView } from "./CodeView.tsx";
import { useApp } from "./context.ts";
import { ParChip, type Shot } from "./ParMeter.tsx";

const BUGGY = `fn longest(a: &str, b: &str) -> &str {
    if a.len() < b.len() { a } else { b }
}`;

const COMPILES = `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() < b.len() { a } else { b }
}`;

const FIXED = `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}`;

const PAR = 3;
const TESTS = 4;
/** Tests that the wrong comparison still passes (strings of equal length). */
const PASSED_WITH_BUG = 1;

interface Frame {
  code: string;
  shots: Shot[];
  /** How long the frame stays, in ms. */
  duration: number;
}

const FRAMES: Frame[] = [
  { code: BUGGY, shots: [], duration: 2200 },
  { code: BUGGY, shots: ["CE"], duration: 3200 },
  { code: COMPILES, shots: ["CE"], duration: 1600 },
  { code: COMPILES, shots: ["CE", "WA"], duration: 3200 },
  { code: FIXED, shots: ["CE", "WA"], duration: 1600 },
  { code: FIXED, shots: ["CE", "WA", "AC"], duration: 4000 },
];

function prefersReducedMotion(): boolean {
  return window.matchMedia("(prefers-reduced-motion: reduce)").matches;
}

/** A looping replay of one round: a compile error, then a wrong answer, then a pass. */
export function HeroDemo() {
  const { t } = useApp();
  const [index, setIndex] = useState(() => (prefersReducedMotion() ? FRAMES.length - 1 : 0));

  useEffect(() => {
    if (prefersReducedMotion()) return;
    const timer = setTimeout(
      () => setIndex((i) => (i + 1) % FRAMES.length),
      FRAMES[index].duration,
    );
    return () => clearTimeout(timer);
  }, [index]);

  const frame = FRAMES[index];
  const last = frame.shots.at(-1);
  const tries = frame.shots.length;
  return (
    <div class="demo" aria-hidden="true">
      <div class="demo-bar">
        <span class="demo-dots">
          <span />
          <span />
          <span />
        </span>
        <span class="demo-file">longest.rs</span>
        <ParChip par={PAR} shots={frame.shots} label="" />
      </div>
      <CodeView code={frame.code} errorLines={last === "CE" && frame.code === BUGGY ? [1] : []} />
      <div class={`demo-result${last === undefined ? "" : ` ${last}`}`} key={index}>
        {last === undefined && (
          <span class="demo-submit">
            <Play size={14} /> {t.demoSubmitHint} <kbd>Ctrl+↵</kbd>
          </span>
        )}
        {last === "CE" && (
          <span>
            <span class="verdict CE">CE</span> error[E0106]: missing lifetime specifier
          </span>
        )}
        {last === "WA" && (
          <span>
            <span class="verdict WA">WA</span> {t.demoWrongAnswer(PASSED_WITH_BUG, TESTS)}
          </span>
        )}
        {last === "AC" && (
          <span>
            <span class="verdict AC">AC</span> {t.demoAccepted(TESTS)}
            <strong class="demo-title">{titleFor(tries, PAR)}!</strong>
          </span>
        )}
      </div>
    </div>
  );
}
