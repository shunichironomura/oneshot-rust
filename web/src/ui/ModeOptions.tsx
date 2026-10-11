import { MODES } from "../core/mode.ts";
import { useId } from "preact/hooks";
import { useApp } from "./context.ts";

/** The feedback mode of the next round (and of practice), one option per mode with its description. */
export function ModeOptions() {
  const { settings, updateSettings, t } = useApp();
  // The options appear in more than one place: each group needs its own name.
  const name = useId();
  return (
    <div class="mode-options">
      {MODES.map((m) => (
        <label key={m} class={`mode-option${settings.mode === m ? " selected" : ""}`}>
          <input
            type="radio"
            name={name}
            checked={settings.mode === m}
            onChange={() => updateSettings({ ...settings, mode: m })}
          />
          <strong>{t.modes[m]}</strong>
          <span>{t.modeDescriptions[m]}</span>
        </label>
      ))}
    </div>
  );
}
