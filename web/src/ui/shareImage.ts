// Draws the share card on a canvas, and shares it with the Web Share API (or copies the text).
import { BOX, digitOf, marksFor } from "../core/marks.ts";
import type { VerdictTone } from "../core/scoring.ts";
import { type ShareCard, type ShareRow, SITE_NAME } from "../core/share.ts";

const WIDTH = 1200;
const HEIGHT = 630;
const PADDING = 72;
const SANS = 'system-ui, -apple-system, "Segoe UI", "Hiragino Sans", "Noto Sans JP", sans-serif';
const DISPLAY = `"Barlow Condensed", "Arial Narrow", ${SANS}`;
const HAND = '"Caveat", "Segoe Print", "Bradley Hand", cursive';

// The light theme of styles.css: the card looks the same whatever the viewer's theme.
const COLORS = {
  paper: "#f4f4f1",
  card: "#fdfdfb",
  navy: "#1e3966",
  soft: "#e4eaf4",
  rule: "#d5dbe5",
  fg: "#1c2230",
  muted: "#5d6676",
  pencil: "#3b3b3e",
};

const TONE_COLORS: Record<VerdictTone, string> = {
  ac: "#1a7f37",
  ce: "#cf222e",
  miss: "#b35900",
  ie: "#6e7781",
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

/** The pencil mark of a hole, `size` wide, with its top left corner at (x, y). */
function drawMark(
  ctx: CanvasRenderingContext2D,
  row: ShareRow,
  x: number,
  y: number,
  size: number,
) {
  const digit = digitOf(row.score);
  if (digit !== undefined) {
    ctx.fillStyle = COLORS.pencil;
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.font = `700 ${Math.round(size * 0.82)}px ${HAND}`;
    ctx.fillText(String(digit), x + size / 2, y + size / 2 - size * 0.04);
  }
  ctx.save();
  ctx.translate(x, y);
  ctx.scale(size / BOX, size / BOX);
  ctx.strokeStyle = COLORS.pencil;
  ctx.lineWidth = 1.7;
  ctx.lineCap = "round";
  ctx.lineJoin = "round";
  ctx.globalAlpha = 0.9;
  for (const d of marksFor(row.score, row.seed)) ctx.stroke(new Path2D(d));
  ctx.restore();
}

/** The verdicts of a hole as a row of small squares centered on `cx`. */
function drawShots(ctx: CanvasRenderingContext2D, row: ShareRow, cx: number, y: number) {
  const side = 18;
  const gap = 6;
  const width = row.shots.length * side + (row.shots.length - 1) * gap;
  row.shots.forEach((shot, i) => {
    ctx.fillStyle = TONE_COLORS[shot.tone];
    ctx.beginPath();
    ctx.roundRect(cx - width / 2 + i * (side + gap), y, side, side, 4);
    ctx.fill();
  });
}

function draw(ctx: CanvasRenderingContext2D, card: ShareCard): void {
  const inner = WIDTH - 2 * PADDING;
  ctx.fillStyle = COLORS.paper;
  ctx.fillRect(0, 0, WIDTH, HEIGHT);
  ctx.fillStyle = COLORS.card;
  ctx.fillRect(24, 24, WIDTH - 48, HEIGHT - 48);
  ctx.strokeStyle = COLORS.navy;
  ctx.lineWidth = 4;
  ctx.strokeRect(24, 24, WIDTH - 48, HEIGHT - 48);
  ctx.textBaseline = "alphabetic";
  ctx.textAlign = "left";

  // Logo, with the site at the far end.
  ctx.fillStyle = COLORS.navy;
  ctx.font = `700 44px ${DISPLAY}`;
  ctx.fillText("◎ OneShot Rust", PADDING, 118);
  ctx.textAlign = "right";
  ctx.fillStyle = COLORS.muted;
  ctx.font = `600 30px ${DISPLAY}`;
  ctx.fillText(SITE_NAME, WIDTH - PADDING, 118);
  ctx.textAlign = "left";

  ctx.fillStyle = COLORS.muted;
  fitFont(ctx, card.title, 500, 30, SANS, inner);
  ctx.fillText(card.title, PADDING, 172);

  ctx.fillStyle = card.solved ? COLORS.navy : COLORS.fg;
  fitFont(ctx, card.headline, 700, 92, DISPLAY, inner);
  ctx.fillText(card.headline, PADDING, 262);

  // One cell per hole, as on the card: the mark, and the verdicts beneath.
  const top = 300;
  const cellHeight = 200;
  const gap = 20;
  const cellWidth = Math.min(170, (inner - gap * (card.rows.length - 1)) / card.rows.length);
  const mark = Math.min(118, cellWidth - 40);
  card.rows.forEach((row, i) => {
    const x = PADDING + i * (cellWidth + gap);
    ctx.fillStyle = COLORS.card;
    ctx.fillRect(x, top, cellWidth, cellHeight);
    ctx.strokeStyle = COLORS.rule;
    ctx.lineWidth = 2;
    ctx.strokeRect(x, top, cellWidth, cellHeight);
    const cx = x + cellWidth / 2;
    drawMark(ctx, row, cx - mark / 2, top + 16, mark);
    drawShots(ctx, row, cx, top + cellHeight - 40);
  });

  ctx.textBaseline = "alphabetic";
  ctx.textAlign = "left";
  ctx.fillStyle = COLORS.navy;
  ctx.font = `600 40px ${DISPLAY}`;
  ctx.fillText(card.detail, PADDING, 566);
}

/** Renders the card as a PNG. */
export async function renderShareImage(card: ShareCard): Promise<Blob> {
  // The card's typefaces, which may not have been needed on screen yet. A face that fails
  // to load falls back, as on screen.
  await Promise.allSettled([
    document.fonts.load(`700 44px ${DISPLAY}`),
    document.fonts.load(`700 100px ${HAND}`),
  ]);
  const canvas = document.createElement("canvas");
  canvas.width = WIDTH;
  canvas.height = HEIGHT;
  const ctx = canvas.getContext("2d");
  if (ctx === null) throw new Error("Canvas 2D is not available");
  draw(ctx, card);
  return new Promise((resolve, reject) =>
    canvas.toBlob(
      (blob) => (blob === null ? reject(new Error("Could not encode the image")) : resolve(blob)),
      "image/png",
    ),
  );
}

export type ShareOutcome = "shared" | "cancelled" | "copied";

/**
 * Whether results are shared through the system's share sheet (with the image), which is
 * the usual way on touch devices. Elsewhere the text is copied, ready to paste.
 */
export function sharesThroughSheet(): boolean {
  return window.matchMedia("(pointer: coarse)").matches && navigator.canShare !== undefined;
}

/**
 * Opens the share sheet with the image and the text when `image` is given and the browser
 * can share it, and copies the text otherwise (or when sharing is refused). Call it right
 * from the click handler: the share sheet needs the user's gesture.
 */
export async function shareResult(image: File | undefined, text: string): Promise<ShareOutcome> {
  const data: ShareData = { files: image === undefined ? [] : [image], text };
  if (image !== undefined && navigator.canShare?.(data) === true) {
    try {
      await navigator.share(data);
      return "shared";
    } catch (e) {
      if (e instanceof DOMException && e.name === "AbortError") return "cancelled";
      // e.g. NotAllowedError: fall back to copying.
    }
  }
  await navigator.clipboard.writeText(text);
  return "copied";
}
