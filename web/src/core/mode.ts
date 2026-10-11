/**
 * How much of the diagnostics a run showed. Players no longer choose: every new run is
 * played in "standard", and the other modes remain only in records of older versions.
 */
export type Mode = "beginner" | "standard" | "advanced" | "oni";

/** From the most to the least verbose. */
export const MODES: readonly Mode[] = ["beginner", "standard", "advanced", "oni"];

/** The mode of every new run and round. */
export const PLAY_MODE: Mode = "standard";
