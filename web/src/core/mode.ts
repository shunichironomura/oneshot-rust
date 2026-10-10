/** How much of the diagnostics the player gets to see. Records are kept per mode. */
export type Mode = "beginner" | "standard" | "advanced" | "oni";

export const MODES: readonly Mode[] = ["beginner", "standard", "advanced", "oni"];

export const MODE_LABELS: Record<Mode, string> = {
  beginner: "入門",
  standard: "標準",
  advanced: "上級",
  oni: "鬼",
};

export const MODE_DESCRIPTIONS: Record<Mode, string> = {
  beginner: "rustc の出力をすべて表示（help・修正提案つき）。エディタにエラー行マーカーを表示",
  standard: "エラーメッセージ、コード片、note を表示。help と修正提案は非表示",
  advanced: "エラーの件数と行番号のみ",
  oni: "判定の 2 文字のみ",
};
