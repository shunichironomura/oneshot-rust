import { Check, Copy, X } from "lucide-preact";
import type { ComponentChildren } from "preact";
import { useEffect, useState } from "preact/hooks";
import { useApp } from "./context.ts";
import { IconButton } from "./IconButton.tsx";

type Outcome = "copied" | "failed";

/** How long the outcome of a copy is shown before the button goes back to "Copy". */
const OUTCOME_MS = 2000;

/** Copies `text` to the clipboard, and says for a moment whether that worked. */
export function CopyButton({ text, label }: { text: string; label?: string }) {
  const { t } = useApp();
  const [outcome, setOutcome] = useState<Outcome>();

  useEffect(() => {
    if (outcome === undefined) return;
    const timer = setTimeout(() => setOutcome(undefined), OUTCOME_MS);
    return () => clearTimeout(timer);
  }, [outcome]);

  const copy = () =>
    void navigator.clipboard.writeText(text).then(
      () => setOutcome("copied"),
      (e: unknown) => {
        console.error("Could not copy to the clipboard", e);
        setOutcome("failed");
      },
    );
  const icon = outcome === "copied" ? Check : outcome === "failed" ? X : Copy;
  const status =
    outcome === "copied" ? t.copied : outcome === "failed" ? t.copyFailed : (label ?? t.copy);
  return (
    <IconButton icon={icon} label={status} onClick={copy} class="copy-button">
      {label === undefined ? undefined : status}
    </IconButton>
  );
}

/** A block (code, a message) with an icon-only copy button in its top-right corner. */
export function Copyable({
  text,
  children,
  class: cls,
}: {
  text: string;
  children: ComponentChildren;
  class?: string;
}) {
  return (
    <div class={cls === undefined ? "copyable" : `copyable ${cls}`}>
      {children}
      <CopyButton text={text} />
    </div>
  );
}
