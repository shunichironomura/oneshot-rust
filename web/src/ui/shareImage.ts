// Draws the share card on a canvas, and hands the image to the Web Share API or a download.
import type { VerdictTone } from "../core/scoring.ts";
import { layoutShots, type ShareCard, SITE_NAME } from "../core/share.ts";

const WIDTH = 1200;
const HEIGHT = 630;
const PADDING = 72;
const SANS = 'system-ui, -apple-system, "Segoe UI", "Hiragino Sans", "Noto Sans JP", sans-serif';
const MONO = "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace";

// The dark theme of styles.css: the card looks the same whatever the viewer's theme.
const COLORS = {
  bg: "#111317",
  panel: "#181b20",
  border: "#2d333b",
  fg: "#e6edf3",
  muted: "#8d96a0",
  accent: "#f0883e",
  onTone: "#111317",
};

const TONE_COLORS: Record<VerdictTone, string> = {
  ac: "#3fb950",
  ce: "#f85149",
  miss: "#d29922",
  ie: "#8b949e",
};

/** Sets the largest font up to `maxSize` px in which `text` fits in `maxWidth`. */
function fitFont(
  ctx: CanvasRenderingContext2D,
  text: string,
  weight: number,
  maxSize: number,
  family: string,
  maxWidth: number,
): void {
  ctx.font = `${weight} ${maxSize}px ${family}`;
  const width = ctx.measureText(text).width;
  if (width > maxWidth) {
    ctx.font = `${weight} ${Math.floor((maxSize * maxWidth) / width)}px ${family}`;
  }
}

function draw(ctx: CanvasRenderingContext2D, card: ShareCard): void {
  const inner = WIDTH - 2 * PADDING;
  ctx.fillStyle = COLORS.bg;
  ctx.fillRect(0, 0, WIDTH, HEIGHT);
  ctx.strokeStyle = COLORS.border;
  ctx.lineWidth = 2;
  ctx.strokeRect(24, 24, WIDTH - 48, HEIGHT - 48);
  ctx.textBaseline = "alphabetic";

  // Logo
  ctx.font = `700 36px ${SANS}`;
  ctx.fillStyle = COLORS.accent;
  ctx.fillText("◎", PADDING, 118);
  ctx.fillStyle = COLORS.fg;
  ctx.fillText("OneShot Rust", PADDING + 48, 118);

  ctx.fillStyle = COLORS.muted;
  fitFont(ctx, card.problemTitle, 500, 40, SANS, inner);
  ctx.fillText(card.problemTitle, PADDING, 200);

  ctx.fillStyle = card.solved ? COLORS.accent : TONE_COLORS.ce;
  fitFont(ctx, card.headline, 800, 96, SANS, inner);
  ctx.fillText(card.headline, PADDING, 310);

  const row = layoutShots(card.shots.length, inner, 96);
  const top = 360;
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.font = `700 ${Math.round(row.size * 0.34)}px ${MONO}`;
  card.shots.forEach((shot, i) => {
    const x = PADDING + row.xs[i];
    ctx.fillStyle = TONE_COLORS[shot.tone];
    ctx.beginPath();
    ctx.roundRect(x, top, row.size, row.size, row.size * 0.14);
    ctx.fill();
    ctx.fillStyle = COLORS.onTone;
    ctx.fillText(shot.label, x + row.size / 2, top + row.size / 2);
  });

  ctx.textBaseline = "alphabetic";
  ctx.textAlign = "left";
  ctx.fillStyle = COLORS.fg;
  ctx.font = `500 34px ${SANS}`;
  ctx.fillText(card.detail, PADDING, 552);
  ctx.textAlign = "right";
  ctx.fillStyle = COLORS.muted;
  ctx.font = `500 28px ${SANS}`;
  ctx.fillText(SITE_NAME, WIDTH - PADDING, 552);
}

/** Renders the card as a PNG. */
export function renderShareImage(card: ShareCard): Promise<Blob> {
  const canvas = document.createElement("canvas");
  canvas.width = WIDTH;
  canvas.height = HEIGHT;
  const ctx = canvas.getContext("2d");
  if (ctx === null) return Promise.reject(new Error("Canvas 2D is not available"));
  draw(ctx, card);
  return new Promise((resolve, reject) =>
    canvas.toBlob(
      (blob) => (blob === null ? reject(new Error("Could not encode the image")) : resolve(blob)),
      "image/png",
    ),
  );
}

export type ShareOutcome = "shared" | "cancelled" | "downloaded";

function download(file: File): void {
  const url = URL.createObjectURL(file);
  const a = document.createElement("a");
  a.href = url;
  a.download = file.name;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 0);
}

/**
 * Opens the share sheet with the image where the browser can share files, and downloads
 * it otherwise (or when sharing is refused). Call it right from the click handler: the
 * share sheet needs the user's gesture.
 */
export async function shareImage(file: File, text: string): Promise<ShareOutcome> {
  const data: ShareData = { files: [file], text };
  if (navigator.canShare?.(data) === true) {
    try {
      await navigator.share(data);
      return "shared";
    } catch (e) {
      if (e instanceof DOMException && e.name === "AbortError") return "cancelled";
      // e.g. NotAllowedError: fall back to a download.
    }
  }
  download(file);
  return "downloaded";
}
