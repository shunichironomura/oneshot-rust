import type { ComponentChildren } from "preact";
import { useEffect, useRef } from "preact/hooks";

interface Props {
  open: boolean;
  onClose: () => void;
  children: ComponentChildren;
  class?: string;
}

/** Modal `<dialog>` controlled by the `open` prop. Escape closes it. */
export function Dialog({ open, onClose, children, class: className }: Props) {
  const ref = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    const d = ref.current;
    if (d === null) return;
    if (open && !d.open) d.showModal();
    if (!open && d.open) d.close();
  }, [open]);

  return (
    <dialog ref={ref} class={className} onClose={onClose}>
      {children}
    </dialog>
  );
}
