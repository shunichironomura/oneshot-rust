import { LOCALE_NAMES, LOCALES } from "../core/i18n.ts";
import {
  DEFAULT_SETTINGS,
  type EditorSettings,
  FONT_FAMILIES,
  notepadEditor,
} from "../core/settings.ts";
import { X } from "lucide-preact";
import { useApp } from "./context.ts";
import { Dialog } from "./Dialog.tsx";

/** Settings as a drawer over the current page. Changes apply immediately. */
export function SettingsPanel({ open, onClose }: { open: boolean; onClose: () => void }) {
  const { settings, updateSettings, t } = useApp();
  const editor = settings.editor;
  const setEditor = (patch: Partial<EditorSettings>) =>
    updateSettings({ ...settings, editor: { ...editor, ...patch } });

  return (
    <Dialog open={open} onClose={onClose} class="drawer settings">
      <div class="drawer-head">
        <h2>{t.settingsTitle}</h2>
        <button type="button" class="icon-button icon-only" aria-label={t.close} onClick={onClose}>
          <X size={18} aria-hidden="true" />
        </button>
      </div>

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
    </Dialog>
  );
}
