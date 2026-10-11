import { Check, Share2 } from "lucide-preact";
import { useEffect, useMemo, useState } from "preact/hooks";
import type { ShareCard } from "../core/share.ts";
import { useApp } from "./context.ts";
import {
  renderShareImage,
  type ShareOutcome,
  shareResult,
  sharesThroughSheet,
} from "./shareImage.ts";

interface Props {
  /** Wordle-style text, shared along with the image or copied on its own. */
  text: string;
  card: ShareCard;
  /** File name of the image, without the extension. */
  fileName: string;
}

/**
 * One button to share a finished round or run: the system's share sheet with the image and
 * the text on touch devices, and the text copied to the clipboard elsewhere.
 */
export function ShareButton({ text, card, fileName }: Props) {
  const { t } = useApp();
  const sheet = useMemo(sharesThroughSheet, []);
  // Rendered up front, so that sharing can start right from the click.
  const [image, setImage] = useState<File>();
  const [outcome, setOutcome] = useState<ShareOutcome>();

  useEffect(() => {
    setImage(undefined);
    setOutcome(undefined);
    if (!sheet) return;
    let current = true;
    renderShareImage(card)
      .then((blob) => {
        if (current) setImage(new File([blob], `${fileName}.png`, { type: "image/png" }));
      })
      .catch((e: unknown) => console.error("Could not render the share image", e));
    return () => {
      current = false;
    };
  }, [sheet, card, fileName]);

  const copied = outcome === "copied";
  return (
    <button
      type="button"
      class="icon-button"
      onClick={() => void shareResult(image, text).then(setOutcome)}
    >
      {copied ? <Check size={16} aria-hidden="true" /> : <Share2 size={16} aria-hidden="true" />}
      {copied ? t.copied : t.share}
    </button>
  );
}
