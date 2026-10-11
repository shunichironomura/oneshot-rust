import { Copy, Share2 } from "lucide-preact";
import { useEffect, useState } from "preact/hooks";
import type { ShareCard } from "../core/share.ts";
import { useApp } from "./context.ts";
import { renderShareImage, type ShareOutcome, shareImage } from "./shareImage.ts";

interface Props {
  /** Wordle-style text for the clipboard. */
  text: string;
  card: ShareCard;
  /** File name of the image, without the extension. */
  fileName: string;
}

/** "Copy result" and "Share image" for a finished run or round. */
export function ShareButtons({ text, card, fileName }: Props) {
  const { t } = useApp();
  const [copied, setCopied] = useState(false);
  // Rendered up front, so that sharing can start right from the click.
  const [image, setImage] = useState<File>();
  const [shared, setShared] = useState<ShareOutcome>();

  useEffect(() => {
    setImage(undefined);
    setShared(undefined);
    let current = true;
    renderShareImage(card)
      .then((blob) => {
        if (current) setImage(new File([blob], `${fileName}.png`, { type: "image/png" }));
      })
      .catch((e: unknown) => console.error("Could not render the share image", e));
    return () => {
      current = false;
    };
  }, [card, fileName]);

  const copy = async () => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
  };
  const share = async () => {
    if (image !== undefined) setShared(await shareImage(image, text));
  };

  return (
    <>
      <button type="button" class="icon-button" onClick={() => void copy()}>
        <Copy size={16} aria-hidden="true" />
        {copied ? t.copied : t.copyResult}
      </button>
      <button
        type="button"
        class="icon-button"
        onClick={() => void share()}
        disabled={image === undefined}
      >
        <Share2 size={16} aria-hidden="true" />
        {shared === "downloaded" ? t.imageSaved : t.shareImage}
      </button>
    </>
  );
}
