import type { LucideIcon } from "lucide-preact";
import type { ComponentChildren } from "preact";

interface Props {
  icon: LucideIcon;
  /** Accessible name; also shown as a tooltip. */
  label: string;
  onClick?: () => void;
  href?: string;
  disabled?: boolean;
  /** Visible text next to the icon. Omit for an icon-only button. */
  children?: ComponentChildren;
  class?: string;
}

/** A button (or link) with an icon. Icon-only buttons expose `label` as tooltip and aria-label. */
export function IconButton({
  icon: Icon,
  label,
  onClick,
  href,
  disabled,
  children,
  class: cls,
}: Props) {
  const className = `icon-button${children === undefined ? " icon-only" : ""}${cls ? ` ${cls}` : ""}`;
  const content = (
    <>
      <Icon size={18} aria-hidden="true" />
      {children}
    </>
  );
  return href !== undefined ? (
    <a class={className} href={href} title={label} aria-label={label}>
      {content}
    </a>
  ) : (
    <button
      type="button"
      class={className}
      title={label}
      aria-label={label}
      onClick={onClick}
      disabled={disabled}
    >
      {content}
    </button>
  );
}
