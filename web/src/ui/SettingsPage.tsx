import { MODE_DESCRIPTIONS, MODE_LABELS, MODES } from "../core/mode.ts";
import { DEFAULT_SETTINGS, FONT_FAMILIES, LIVES_OPTIONS, notepadEditor } from "../core/settings.ts";
import { useApp } from "./context.ts";

export function SettingsPage() {
  const { settings, updateSettings } = useApp();
  const editor = settings.editor;
  const setEditor = (patch: Partial<typeof editor>) =>
    updateSettings({ ...settings, editor: { ...editor, ...patch } });

  return (
    <section class="settings">
      <h1>設定</h1>

      <fieldset>
        <legend>診断表示モード</legend>
        {MODES.map((m) => (
          <label key={m} class="radio">
            <input
              type="radio"
              name="mode"
              checked={settings.mode === m}
              onChange={() => updateSettings({ ...settings, mode: m })}
            />
            <strong>{MODE_LABELS[m]}</strong> — {MODE_DESCRIPTIONS[m]}
          </label>
        ))}
        <p class="hint">
          記録はモードごとに管理されます。挑戦中のランには、次のランから反映されます。
        </p>
      </fieldset>

      <fieldset>
        <legend>ルール</legend>
        <label>
          残機（1 問あたりの提出回数の上限）{" "}
          <select
            value={String(settings.lives)}
            onChange={(e) => {
              const v = e.currentTarget.value;
              updateSettings({ ...settings, lives: v === "null" ? null : Number(v) });
            }}
          >
            {LIVES_OPTIONS.map((l) => (
              <option key={String(l)} value={String(l)}>
                {l === null ? "なし" : `${l} 回`}
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
          提出前に確認ダイアログを出す
        </label>
      </fieldset>

      <fieldset>
        <legend>エディタ</legend>
        <label class="checkbox">
          <input
            type="checkbox"
            checked={editor.syntaxHighlight}
            onChange={(e) => setEditor({ syntaxHighlight: e.currentTarget.checked })}
          />
          シンタックスハイライト
        </label>
        <label class="checkbox">
          <input
            type="checkbox"
            checked={editor.closeBrackets}
            onChange={(e) => setEditor({ closeBrackets: e.currentTarget.checked })}
          />
          括弧の自動補完
        </label>
        <label class="checkbox">
          <input
            type="checkbox"
            checked={editor.autoIndent}
            onChange={(e) => setEditor({ autoIndent: e.currentTarget.checked })}
          />
          自動インデント
        </label>
        <button
          type="button"
          onClick={() => updateSettings({ ...settings, editor: notepadEditor(editor) })}
        >
          メモ帳モード（すべて OFF）
        </button>
        <label>
          フォントサイズ{" "}
          <input
            type="number"
            min={10}
            max={28}
            value={editor.fontSize}
            onChange={(e) => setEditor({ fontSize: Number(e.currentTarget.value) })}
          />
        </label>
        <label>
          フォント{" "}
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

      <button type="button" onClick={() => updateSettings(DEFAULT_SETTINGS)}>
        既定値に戻す
      </button>
    </section>
  );
}
