import { LOCALE_NAMES, LOCALES } from "../core/i18n.ts";
import { MODES } from "../core/mode.ts";
import {
  DEFAULT_SETTINGS,
  type EditorSettings,
  FONT_FAMILIES,
  LIVES_OPTIONS,
  notepadEditor,
} from "../core/settings.ts";
import { useApp } from "./context.ts";

export function SettingsPage() {
  const { settings, updateSettings, t } = useApp();
  const editor = settings.editor;
  const setEditor = (patch: Partial<EditorSettings>) =>
    updateSettings({ ...settings, editor: { ...editor, ...patch } });

  return (
    <section class="settings">
      <h1>{t.settingsTitle}</h1>

      <fieldset>
        <legend>{t.language}</legend>
        <div class="segmented">
          {LOCALES.map((l) => (
            <button
              key={l}
              type="button"
              aria-pressed={settings.locale === l}
              onClick={() => updateSettings({ ...settings, locale: l })}
            >
              {LOCALE_NAMES[l]}
            </button>
          ))}
        </div>
      </fieldset>

      <fieldset>
        <legend>{t.modeHeading}</legend>
        <div class="mode-options">
          {MODES.map((m) => (
            <label key={m} class={`mode-option${settings.mode === m ? " selected" : ""}`}>
              <input
                type="radio"
                name="mode"
                checked={settings.mode === m}
                onChange={() => updateSettings({ ...settings, mode: m })}
              />
              <strong>{t.modes[m]}</strong>
              <span>{t.modeDescriptions[m]}</span>
            </label>
          ))}
        </div>
        <p class="hint">{t.modeHint}</p>
      </fieldset>

      <fieldset>
        <legend>{t.rulesHeading}</legend>
        <label class="row">
          <span>{t.livesLabel}</span>
          <select
            value={String(settings.lives)}
            onChange={(e) => {
              const v = e.currentTarget.value;
              updateSettings({ ...settings, lives: v === "null" ? null : Number(v) });
            }}
          >
            {LIVES_OPTIONS.map((l) => (
              <option key={String(l)} value={String(l)}>
                {l === null ? t.livesUnlimited : String(l)}
              </option>
            ))}
          </select>
        </label>
        <label class="checkbox">
          <input
            type="checkbox"
            checked={settings.confirmSubmit}
            onChange={(e) =>
              updateSettings({ ...settings, confirmSubmit: e.currentTarget.checked })
            }
          />
          {t.confirmLabel}
        </label>
      </fieldset>

      <fieldset>
        <legend>{t.editorHeading}</legend>
        <label class="checkbox">
          <input
            type="checkbox"
            checked={editor.syntaxHighlight}
            onChange={(e) => setEditor({ syntaxHighlight: e.currentTarget.checked })}
          />
          {t.syntaxHighlight}
        </label>
        <label class="checkbox">
          <input
            type="checkbox"
            checked={editor.closeBrackets}
            onChange={(e) => setEditor({ closeBrackets: e.currentTarget.checked })}
          />
          {t.closeBrackets}
        </label>
        <label class="checkbox">
          <input
            type="checkbox"
            checked={editor.autoIndent}
            onChange={(e) => setEditor({ autoIndent: e.currentTarget.checked })}
          />
          {t.autoIndent}
        </label>
        <button
          type="button"
          onClick={() => updateSettings({ ...settings, editor: notepadEditor(editor) })}
        >
          {t.notepadMode}
        </button>
        <label class="row">
          <span>{t.fontSize}</span>
          <input
            type="number"
            min={10}
            max={28}
            value={editor.fontSize}
            onChange={(e) => setEditor({ fontSize: Number(e.currentTarget.value) })}
          />
        </label>
        <label class="row">
          <span>{t.font}</span>
          <select
            value={editor.fontFamily}
            onChange={(e) => setEditor({ fontFamily: e.currentTarget.value })}
          >
            {FONT_FAMILIES.map((f) => (
              <option key={f} value={f}>
                {f.split(",")[0].replaceAll("'", "")}
              </option>
            ))}
          </select>
        </label>
      </fieldset>

      <button
        type="button"
        onClick={() => updateSettings({ ...DEFAULT_SETTINGS, locale: settings.locale })}
      >
        {t.resetSettings}
      </button>
    </section>
  );
}
