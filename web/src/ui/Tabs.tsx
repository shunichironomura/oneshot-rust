import type { ComponentChildren } from "preact";
import { useId } from "preact/hooks";
import { keyTarget, type Orientation } from "../core/tabs.ts";

export interface TabOption<T> {
  id: T;
  label: ComponentChildren;
}

/** Selects the option a key moves to, and focuses it as WAI-ARIA tabs and radios do. */
function useArrowKeys<T>(
  options: TabOption<T>[],
  current: number,
  orientation: Orientation,
  onSelect: (id: T) => void,
  domId: (i: number) => string,
) {
  return (e: KeyboardEvent) => {
    const target = keyTarget(e.key, current, options.length, orientation);
    if (target === undefined) return;
    e.preventDefault();
    onSelect(options[target].id);
    document.getElementById(domId(target))?.focus();
  };
}

interface TabsProps<T> {
  tabs: TabOption<T>[];
  selected: T;
  onSelect: (id: T) => void;
  label?: string;
  orientation?: "horizontal" | "vertical";
  class?: string;
  panelClass?: string;
  /** The selected tab's panel. */
  children: ComponentChildren;
}

/**
 * Tabs over one panel: the tab list and the panel are siblings, so a grid can place them.
 * Arrow keys, Home and End move between the tabs and select them.
 */
export function Tabs<T extends string | number>({
  tabs,
  selected,
  onSelect,
  label,
  orientation = "horizontal",
  class: cls,
  panelClass,
  children,
}: TabsProps<T>) {
  const base = useId();
  const tabId = (i: number) => `${base}-tab-${i}`;
  const panelId = `${base}-panel`;
  const current = Math.max(
    0,
    tabs.findIndex(({ id }) => id === selected),
  );
  // A vertical list lays out as a row on narrow screens, so it takes both arrow pairs.
  const onKeyDown = useArrowKeys(
    tabs,
    current,
    orientation === "vertical" ? "both" : "horizontal",
    onSelect,
    tabId,
  );
  return (
    <>
      <div
        class={cls === undefined ? "tabs" : `tabs ${cls}`}
        role="tablist"
        aria-label={label}
        aria-orientation={orientation}
        onKeyDown={onKeyDown}
      >
        {tabs.map(({ id, label }, i) => (
          <button
            key={id}
            type="button"
            role="tab"
            id={tabId(i)}
            aria-selected={i === current}
            aria-controls={panelId}
            tabIndex={i === current ? 0 : -1}
            onClick={() => onSelect(id)}
          >
            {label}
          </button>
        ))}
      </div>
      <div role="tabpanel" id={panelId} aria-labelledby={tabId(current)} class={panelClass}>
        {children}
      </div>
    </>
  );
}

interface SegmentedProps<T> {
  options: TabOption<T>[];
  selected: T;
  onSelect: (id: T) => void;
  label: string;
}

/** One of a few views of the same content, as joined segments: a radio group. */
export function Segmented<T extends string>({
  options,
  selected,
  onSelect,
  label,
}: SegmentedProps<T>) {
  const base = useId();
  const optionId = (i: number) => `${base}-option-${i}`;
  const current = Math.max(
    0,
    options.findIndex(({ id }) => id === selected),
  );
  const onKeyDown = useArrowKeys(options, current, "both", onSelect, optionId);
  return (
    <div class="segmented" role="radiogroup" aria-label={label} onKeyDown={onKeyDown}>
      {options.map(({ id, label }, i) => (
        <button
          key={id}
          type="button"
          role="radio"
          id={optionId(i)}
          aria-checked={i === current}
          tabIndex={i === current ? 0 : -1}
          onClick={() => onSelect(id)}
        >
          {label}
        </button>
      ))}
    </div>
  );
}
