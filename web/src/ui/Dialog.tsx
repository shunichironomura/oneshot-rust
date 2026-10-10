import type { ComponentChildren } from "preact";
import { useEffect, useRef } from "preact/hooks";

interface Props {
  open: boolean;
  onClose: () => void;
  /** Called on Enter (or Ctrl/⌘+Enter) while the dialog is open. */
  onEnter?: () => void;
  children: ComponentChildren;
  class?: string;
}

/**
 * Modal `<dialog>` controlled by the `open` prop. Escape closes it.
 *
 * The dialog itself takes focus rather than its first button: a dialog opened by a
 * key press (Ctrl+Enter to submit) would otherwise receive the rest of that key
 * press on the focused button and close again immediately. Enter is handled
 * explicitly via `onEnter` instead.
 */
export function Dialog({ open, onClose, onEnter, children, class: className }: Props) {
  const ref = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    const d = ref.current;
    if (d === null) return;
    if (open && !d.open) {
      d.showModal();
      d.focus();
    }
    if (!open && d.open) d.close();
  }, [open]);

  const onKeyDown = (e: KeyboardEvent) => {
    if (onEnter === undefined || e.key !== "Enter" || e.isComposing) return;
    if (e.target instanceof HTMLButtonElement || e.target instanceof HTMLAnchorElement) return;
    e.preventDefault();
    onEnter();
  };

  return (
    <dialog ref={ref} class={className} onClose={onClose} onKeyDown={onKeyDown} tabIndex={-1}>
      {children}
    </dialog>
  );
}
