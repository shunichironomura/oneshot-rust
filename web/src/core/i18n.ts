// UI messages. English is the default; Japanese is available as an alternative.
import type {
  CaseStatus,
  InternalErrorReason,
  RuntimeErrorReason,
  VerdictKind,
} from "./classify.ts";
import type { Mode } from "./mode.ts";
import type { Difficulty, ProblemKind } from "./problem.ts";

export type Locale = "en" | "ja";

export const LOCALES: readonly Locale[] = ["en", "ja"];

export const LOCALE_NAMES: Record<Locale, string> = { en: "English", ja: "日本語" };

/** Text that problems provide per locale. English is required. */
export interface Localized {
  en: string;
  ja?: string;
}

export function localize(text: Localized, locale: Locale): string {
  return text[locale] ?? text.en;
}

export interface Messages {
  // Navigation and app shell
  navProblems: string;
  navChallenge: string;
  navSettings: string;
  language: string;
  storageError: (message: string) => string;
  storageBlocked: string;
  storageOutdated: string;
  reload: string;

  // Toolchain loading
  toolchainDownloading: (loadedMb: string, totalMb: string) => string;
  /** Submit button label while the compiler is not ready yet. */
  toolchainStarting: string;
  toolchainMissing: string;
  toolchainError: (message: string) => string;

  // Landing page
  heroTitle: string;
  heroTagline: string;
  playCardTitle: string;
  playCardStart: (title: string) => string;
  playCardContinue: (title: string) => string;
  playCardAllSolved: (mode: string) => string;
  start: string;
  continue: string;
  browseProblems: (n: number) => string;
  challengeCardDescription: string;
  challengeCardStart: string;
  challengeCardContinue: string;
  demoAccepted: (n: number) => string;
  demoWrongAnswer: (passed: number, n: number) => string;
  demoSubmitHint: string;
  madeBy: string;

  // Problem list
  problemsHeading: string;
  columnProblem: string;
  columnBest: (mode: string) => string;
  bestTooltip: (ceCount: number) => string;
  columnDifficulty: string;
  shotsTooltip: (tries: number) => string;
  difficulties: Record<Difficulty, string>;
  modes: Record<Mode, string>;
  modeDescriptions: Record<Mode, string>;
  kinds: Record<ProblemKind, string>;

  // Play screen
  lives: (n: number) => string;
  modeChip: (mode: string) => string;
  submit: string;
  judging: string;
  history: string;
  startOver: string;
  giveUp: string;
  nextInChallenge: string;
  nextProblem: string;
  tabStatement: string;
  tabHarness: string;
  tabSolution: string;
  practiceChip: string;
  practiceTooltip: string;
  runModeNotice: (mode: string) => string;
  confirmSubmit: (n: number) => string;
  confirmSubmitYes: string;
  cancel: string;
  confirmStartOver: string;
  startOverAbandons: string;
  confirmStartOverYes: string;
  confirmGiveUp: string;
  confirmGiveUpYes: string;
  problemNotFound: (id: string) => string;
  backToProblems: string;
  challengeChip: (position: string) => string;

  // Model solution
  solutionLocked: string;
  solutionGiveUpHint: string;
  solutionGiveUp: string;
  confirmRevealSolution: string;
  confirmRevealSolutionYes: string;
  solutionUnlocked: string;
  showSolution: string;
  solutionLoading: string;
  solutionLoadError: (message: string) => string;
  solutionCode: string;
  solutionDiff: string;
  solutionDiffLegend: string;

  // Result panel
  resultEmpty: string;
  compiling: string;
  runningCase: (i: number, n: number) => string;
  rejected: string;
  forbiddenMacro: (name: string) => string;
  submitError: string;
  output: string;
  hiddenCase: string;
  caseStatus: Record<CaseStatus, string>;
  verdicts: Record<VerdictKind, string>;
  ceSummary: (count: number, locations: string) => string;
  inHarness: string;
  unknownLocation: string;
  hiddenHarnessNote: string;
  harnessInternalsNote: string;
  tleSummary: (caseId: string) => string;
  runtimeError: (reason: RuntimeErrorReason) => string;
  internalError: (reason: InternalErrorReason) => string;

  // End of a run
  solvedHeadline: string;
  unsolvedHeadline: string;
  submissions: string;
  compileErrors: string;
  mode: string;
  newBest: string;
  copyResult: string;
  copied: string;
  shareImage: string;
  imageSaved: string;
  reviewSubmissions: string;
  seeSolution: string;
  practiceRun: string;
  tryAgain: string;
  close: string;
  shareTries: (n: number) => string;
  unsolved: string;

  // History
  historyTitle: string;
  noSubmissions: string;
  runSolved: (n: number) => string;
  runFailed: string;
  runPlaying: string;
  runPractice: string;
  attemptHeading: (i: number, verdict: string) => string;
  changesFromTemplate: string;
  changesFrom: (i: number) => string;

  // Settings
  settingsTitle: string;
  modeHeading: string;
  modeHint: string;
  rulesHeading: string;
  livesLabel: string;
  livesUnlimited: string;
  confirmLabel: string;
  editorHeading: string;
  syntaxHighlight: string;
  closeBrackets: string;
  autoIndent: string;
  notepadMode: string;
  fontSize: string;
  font: string;
  resetSettings: string;

  // Challenge
  challengeTitle: string;
  challengeIntro: (n: number) => string;
  challengeRules: (mode: string, lives: string) => string;
  challengeStart: string;
  challengePast: string;
  challengeResult: string;
  challengeTotal: (total: number, solved: number, count: number) => string;
  challengeNew: string;
  challengeSkipDeleted: (id: string) => string;
  challengeShareHeader: (mode: string) => string;
}

const en: Messages = {
  navProblems: "Problems",
  navChallenge: "Challenge",
  navSettings: "Settings",
  language: "Language",
  storageError: (m) => `Could not open browser storage (IndexedDB): ${m}`,
  storageBlocked:
    "OneShot Rust has been updated. Close or reload the other tabs where it is open to continue.",
  storageOutdated: "A newer version of OneShot Rust was opened in another tab. Reload to continue.",
  reload: "Reload",

  toolchainDownloading: (l, t) => `Downloading the compiler (first visit only) — ${l} / ${t} MB`,
  toolchainStarting: "Starting compiler…",
  toolchainMissing: "The toolchain is missing. Build it with toolchain/build.nu.",
  toolchainError: (m) => `Could not load the compiler: ${m}`,

  heroTitle: "Rust in one shot.",
  heroTagline: "No autocomplete. No IDE. Just you and the compiler — every compile counts.",
  playCardTitle: "Solve a problem",
  playCardStart: (title) => `Opens “${title}” in the editor.`,
  playCardContinue: (title) => `Pick up “${title}” where you left off.`,
  playCardAllSolved: (mode) => `All problems solved in ${mode} mode. Try to beat your records.`,
  start: "Start",
  continue: "Continue",
  browseProblems: (n) => `All ${n} problems`,
  challengeCardDescription: "5 random problems back to back. Lowest total wins.",
  challengeCardStart: "Start challenge",
  challengeCardContinue: "Continue challenge",
  demoAccepted: (n) => `Accepted · ${n}/${n} tests passed`,
  demoWrongAnswer: (passed, n) => `Wrong answer · ${passed}/${n} tests passed`,
  demoSubmitHint: "Submit",
  madeBy: "Made by",

  problemsHeading: "Problems",
  columnProblem: "Problem",
  columnBest: (mode) => `Best (${mode})`,
  bestTooltip: (ce) => `${ce} compile error${ce === 1 ? "" : "s"}`,
  columnDifficulty: "Difficulty",
  shotsTooltip: (tries) => `${tries} ${tries === 1 ? "try" : "tries"} so far · aim for one`,
  difficulties: { 1: "Beginner", 2: "Intermediate", 3: "Advanced" },
  modes: { beginner: "Verbose", standard: "Standard", advanced: "Terse", oni: "Silent" },
  modeDescriptions: {
    beginner:
      "Full compiler output, including help and suggestions. Errors are marked in the editor.",
    standard: "Error messages, code snippets and notes. No help or suggested fixes.",
    advanced: "Only the number of errors and their line numbers.",
    oni: "Only the two-letter verdict.",
  },
  kinds: { implement: "Implement", repair: "Repair", signature: "Signature" },

  lives: (n) => `${n} ${n === 1 ? "life" : "lives"} left`,
  modeChip: (mode) => mode,
  submit: "Submit",
  judging: "Judging…",
  history: "Submission history",
  startOver: "Start over",
  giveUp: "Give up",
  nextInChallenge: "Next problem",
  nextProblem: "Next problem",
  tabStatement: "Problem",
  tabHarness: "tests.rs (read-only)",
  tabSolution: "Solution",
  practiceChip: "Practice",
  practiceTooltip: "You have seen the model solution, so this run does not update your best.",
  runModeNotice: (mode) =>
    `This run started in ${mode} mode. A new mode applies from the next run.`,
  confirmSubmit: (n) => `Submit this code? This will be submission #${n}.`,
  confirmSubmitYes: "Submit",
  cancel: "Cancel",
  confirmStartOver: "Reset the code to the template and start a new run?",
  startOverAbandons: "The current run will end unsolved.",
  confirmStartOverYes: "Start over",
  confirmGiveUp: "Give up this problem and move on? It is recorded as unsolved.",
  confirmGiveUpYes: "Give up",
  problemNotFound: (id) => `Problem “${id}” was not found.`,
  backToProblems: "Back to problems",
  challengeChip: (p) => `Challenge ${p}`,

  solutionLocked:
    "The model solution unlocks once you finish this problem: solve it or run out of lives.",
  solutionGiveUpHint: "Stuck? You can give up and look at it now.",
  solutionGiveUp: "Give up and show",
  confirmRevealSolution:
    "Show the model solution? You can keep going, but from now on runs on this problem are practice and do not update your best.",
  confirmRevealSolutionYes: "Show solution",
  solutionUnlocked:
    "Compare your code with the model solution. Once you look at it, later runs on this problem are practice and do not update your best.",
  showSolution: "Show solution",
  solutionLoading: "Loading…",
  solutionLoadError: (m) => `Could not load the solution: ${m}`,
  solutionCode: "Code",
  solutionDiff: "Diff from your code",
  solutionDiffLegend: "- your code   + model solution",

  resultEmpty: "Press Ctrl+Enter (⌘+Enter) to submit. Make it count.",
  compiling: "Compiling…",
  runningCase: (i, n) => `Running test ${i} of ${n}…`,
  rejected: "Not submitted (not counted)",
  forbiddenMacro: (m) => `\`${m}!\` is not allowed: it could read the judge's secrets.`,
  submitError: "Error (not counted)",
  output: "Output",
  hiddenCase: "hidden",
  caseStatus: { pass: "PASS", fail: "FAIL", "not-run": "not run" },
  verdicts: {
    AC: "Accepted",
    WA: "Wrong answer",
    CE: "Compile error",
    UB: "Undefined behavior",
    RE: "Runtime error",
    TLE: "Time limit exceeded",
    IE: "Internal error (not counted)",
  },
  ceSummary: (n, locs) => `${n} error${n === 1 ? "" : "s"} (${locs})`,
  inHarness: " (test harness)",
  unknownLocation: "unknown location",
  hiddenHarnessNote: "(in the test harness; code hidden)",
  harnessInternalsNote: "(harness internals)",
  tleSummary: (id) => `Time limit exceeded while running ${id}`,
  runtimeError: (r) => {
    switch (r.type) {
      case "diagnostic":
      case "panic":
        return r.message;
      case "exit-code":
        return `The program exited with code ${r.code}`;
      case "abnormal-exit":
        return "The program terminated abnormally";
      case "output-limit":
        return `Output exceeded the limit of ${r.limitBytes} bytes`;
    }
  },
  internalError: (r) => {
    switch (r.type) {
      case "ice":
        return `Internal compiler error: ${r.message}`;
      case "crash":
        return `Miri crashed: ${r.message}`;
      case "compile-timeout":
        return "Compilation did not finish within the time limit";
      case "no-start":
        return `The judge could not start (exit code ${r.exitCode ?? "none"})`;
    }
  },

  solvedHeadline: "Solved",
  unsolvedHeadline: "Unsolved",
  submissions: "Submissions",
  compileErrors: "Compile errors",
  mode: "Mode",
  newBest: "New personal best!",
  copyResult: "Copy result",
  copied: "Copied!",
  shareImage: "Share image",
  imageSaved: "Image saved",
  reviewSubmissions: "Review submissions",
  seeSolution: "Model solution",
  practiceRun: "Practice run (solution seen): your best is not updated.",
  tryAgain: "Try again",
  close: "Close",
  shareTries: (n) => `${n} ${n === 1 ? "try" : "tries"}`,
  unsolved: "unsolved",

  historyTitle: "Submission history",
  noSubmissions: "No submissions yet.",
  runSolved: (n) => `Solved in ${n}`,
  runFailed: "Ended unsolved",
  runPractice: "Practice",
  runPlaying: "In progress",
  attemptHeading: (i, v) => `Submission #${i} — ${v}`,
  changesFromTemplate: "Changes from the template",
  changesFrom: (i) => `Changes from submission #${i}`,

  settingsTitle: "Settings",
  modeHeading: "Feedback mode",
  modeHint: "Records are kept per mode. A new mode applies from your next run.",
  rulesHeading: "Rules",
  livesLabel: "Lives (maximum submissions per problem)",
  livesUnlimited: "Unlimited",
  confirmLabel: "Ask for confirmation before each submission",
  editorHeading: "Editor",
  syntaxHighlight: "Syntax highlighting",
  closeBrackets: "Auto-close brackets",
  autoIndent: "Auto-indent",
  notepadMode: "Notepad mode (turn all aids off)",
  fontSize: "Font size",
  font: "Font",
  resetSettings: "Reset to defaults",

  challengeTitle: "5-problem challenge",
  challengeIntro: (n) =>
    `Solve ${n} random problems in a row. Your score is the total number of submissions. Giving up records a problem as unsolved.`,
  challengeRules: (mode, lives) => `Mode: ${mode} · Lives: ${lives}`,
  challengeStart: "Start the challenge",
  challengePast: "Past results",
  challengeResult: "Challenge result",
  challengeTotal: (t, s, c) => `${t} submission${t === 1 ? "" : "s"} in total · ${s}/${c} solved`,
  challengeNew: "New challenge",
  challengeSkipDeleted: (id) => `Problem “${id}” no longer exists and is skipped.`,
  challengeShareHeader: (mode) => `OneShot Rust 🦀 Challenge [${mode}]`,
};

const ja: Messages = {
  navProblems: "問題一覧",
  navChallenge: "連続チャレンジ",
  navSettings: "設定",
  language: "言語",
  storageError: (m) => `保存領域（IndexedDB）を開けませんでした: ${m}`,
  storageBlocked:
    "OneShot Rust が更新されました。続けるには、ほかのタブで開いている OneShot Rust を閉じるか再読み込みしてください。",
  storageOutdated:
    "別のタブで新しい版の OneShot Rust が開かれました。続けるにはこのページを再読み込みしてください。",
  reload: "再読み込み",

  toolchainDownloading: (l, t) => `コンパイラをダウンロード中（初回のみ）… ${l} / ${t} MB`,
  toolchainStarting: "コンパイラ起動中…",
  toolchainMissing: "ツールチェーンが見つかりません。toolchain/build.nu でビルドしてください。",
  toolchainError: (m) => `コンパイラを読み込めませんでした: ${m}`,

  heroTitle: "Rust を、一発で。",
  heroTagline:
    "補完なし、IDE なし。あなたとコンパイラだけ。コンパイルは 1 回ごとにカウントされます。",
  playCardTitle: "問題を解く",
  playCardStart: (title) => `「${title}」をエディタで開きます。`,
  playCardContinue: (title) => `「${title}」の続きから再開します。`,
  playCardAllSolved: (mode) => `${mode}モードの問題はすべて解きました。記録更新を狙いましょう。`,
  start: "はじめる",
  continue: "続きから",
  browseProblems: (n) => `全 ${n} 問の一覧`,
  challengeCardDescription: "ランダムな 5 問を連続で解き、合計の提出回数を競います。",
  challengeCardStart: "チャレンジ開始",
  challengeCardContinue: "チャレンジの続きへ",
  demoAccepted: (n) => `正解 · ${n}/${n} テスト通過`,
  demoWrongAnswer: (passed, n) => `不正解 · ${passed}/${n} テスト通過`,
  demoSubmitHint: "提出",
  madeBy: "作者：",

  problemsHeading: "問題一覧",
  columnProblem: "問題",
  columnBest: (mode) => `ベスト（${mode}）`,
  bestTooltip: (ce) => `うち CE ${ce} 回`,
  columnDifficulty: "難易度",
  shotsTooltip: (tries) => `現在 ${tries} 回（目標は 1 回）`,
  difficulties: { 1: "初級", 2: "中級", 3: "上級" },
  modes: { beginner: "親切", standard: "標準", advanced: "簡潔", oni: "鬼" },
  modeDescriptions: {
    beginner: "rustc の出力をすべて表示（help・修正提案つき）。エディタにエラー行マーカーを表示",
    standard: "エラーメッセージ、コード片、note を表示。help と修正提案は非表示",
    advanced: "エラーの件数と行番号のみ",
    oni: "判定の 2 文字のみ",
  },
  kinds: { implement: "実装", repair: "修理", signature: "シグネチャ" },

  lives: (n) => `残機 ${n}`,
  modeChip: (mode) => `${mode}モード`,
  submit: "提出",
  judging: "判定中…",
  history: "提出履歴",
  startOver: "最初から",
  giveUp: "ギブアップ",
  nextInChallenge: "次の問題へ",
  nextProblem: "次の問題へ",
  tabStatement: "問題文",
  tabHarness: "tests.rs（読み取り専用）",
  tabSolution: "模範解答",
  practiceChip: "練習",
  practiceTooltip: "模範解答を見たあとの挑戦なので、自己ベストは更新されません。",
  runModeNotice: (mode) =>
    `このランは「${mode}」モードで開始されています。モードの変更は次のランから反映されます。`,
  confirmSubmit: (n) => `この内容で提出しますか？（${n} 回目の提出）`,
  confirmSubmitYes: "提出する",
  cancel: "やめる",
  confirmStartOver: "コードをテンプレートに戻し、新しいランを始めますか？",
  startOverAbandons: "進行中のランは未解決で終了します。",
  confirmStartOverYes: "最初からやり直す",
  confirmGiveUp: "この問題をあきらめて次に進みますか？（未解決として記録されます）",
  confirmGiveUpYes: "ギブアップする",
  problemNotFound: (id) => `問題「${id}」は見つかりませんでした。`,
  backToProblems: "問題一覧へ",
  challengeChip: (p) => `連続チャレンジ ${p}`,

  solutionLocked:
    "この問題を終える（正解するか、ライフを使い切る）と、模範解答を見られるようになります。",
  solutionGiveUpHint: "どうしても解けないときは、ギブアップして今すぐ見ることもできます。",
  solutionGiveUp: "ギブアップして見る",
  confirmRevealSolution:
    "模範解答を表示しますか？ 挑戦はこのまま続けられますが、以降この問題の記録は練習扱いになり、自己ベストは更新されません。",
  confirmRevealSolutionYes: "表示する",
  solutionUnlocked:
    "模範解答と見比べてみましょう。表示すると、以降この問題の記録は練習扱いになり、自己ベストは更新されません。",
  showSolution: "模範解答を表示",
  solutionLoading: "読み込み中…",
  solutionLoadError: (m) => `模範解答を読み込めませんでした: ${m}`,
  solutionCode: "コード",
  solutionDiff: "自分のコードとの差分",
  solutionDiffLegend: "- 自分のコード   + 模範解答",

  resultEmpty: "Ctrl+Enter（⌘+Enter）で提出します。一発で通しましょう。",
  compiling: "コンパイル中…",
  runningCase: (i, n) => `実行中（ケース ${i}/${n}）`,
  rejected: "提出できません（カウントされません）",
  forbiddenMacro: (m) => `\`${m}!\` は使用できません（判定の改ざん防止のため）`,
  submitError: "エラー（カウントされません）",
  output: "出力",
  hiddenCase: "非公開",
  caseStatus: { pass: "PASS", fail: "FAIL", "not-run": "未実行" },
  verdicts: {
    AC: "正解",
    WA: "不正解",
    CE: "コンパイルエラー",
    UB: "未定義動作",
    RE: "実行時エラー",
    TLE: "時間切れ",
    IE: "内部エラー（カウントされません）",
  },
  ceSummary: (n, locs) => `エラー ${n} 件（${locs}）`,
  inHarness: "（ハーネス側）",
  unknownLocation: "位置不明",
  hiddenHarnessNote: "（ハーネス側でエラー。コードは非公開）",
  harnessInternalsNote: "（ハーネス内部）",
  tleSummary: (id) => `ケース ${id} の実行中に制限時間を超えました`,
  runtimeError: (r) => {
    switch (r.type) {
      case "diagnostic":
      case "panic":
        return r.message;
      case "exit-code":
        return `終了コード ${r.code}`;
      case "abnormal-exit":
        return "プログラムが異常終了しました";
      case "output-limit":
        return `標準出力が上限（${r.limitBytes} バイト）を超えました`;
    }
  },
  internalError: (r) => {
    switch (r.type) {
      case "ice":
        return `コンパイラの内部エラー: ${r.message}`;
      case "crash":
        return `Miri が異常終了しました: ${r.message}`;
      case "compile-timeout":
        return "コンパイルが制限時間内に終わりませんでした";
      case "no-start":
        return `判定を開始できませんでした（終了コード ${r.exitCode ?? "なし"}）`;
    }
  },

  solvedHeadline: "正解",
  unsolvedHeadline: "未解決",
  submissions: "提出回数",
  compileErrors: "うち CE",
  mode: "モード",
  newBest: "自己ベスト更新！",
  copyResult: "結果をコピー",
  copied: "コピーしました",
  shareImage: "画像でシェア",
  imageSaved: "画像を保存しました",
  reviewSubmissions: "振り返る",
  seeSolution: "模範解答",
  practiceRun: "練習（模範解答を閲覧済み）のため、自己ベストは更新されません。",
  tryAgain: "もう一度",
  close: "閉じる",
  shareTries: (n) => `${n}回`,
  unsolved: "未解決",

  historyTitle: "提出履歴",
  noSubmissions: "まだ提出がありません。",
  runSolved: (n) => `${n} 回で正解`,
  runFailed: "未解決で終了",
  runPractice: "練習",
  runPlaying: "挑戦中",
  attemptHeading: (i, v) => `${i} 回目の提出 — ${v}`,
  changesFromTemplate: "テンプレートからの変更",
  changesFrom: (i) => `${i} 回目からの変更`,

  settingsTitle: "設定",
  modeHeading: "診断表示モード",
  modeHint: "記録はモードごとに管理されます。挑戦中のランには、次のランから反映されます。",
  rulesHeading: "ルール",
  livesLabel: "残機（1 問あたりの提出回数の上限）",
  livesUnlimited: "なし",
  confirmLabel: "提出前に確認ダイアログを出す",
  editorHeading: "エディタ",
  syntaxHighlight: "シンタックスハイライト",
  closeBrackets: "括弧の自動補完",
  autoIndent: "自動インデント",
  notepadMode: "メモ帳モード（すべて OFF）",
  fontSize: "フォントサイズ",
  font: "フォント",
  resetSettings: "既定値に戻す",

  challengeTitle: "連続チャレンジ",
  challengeIntro: (n) =>
    `ランダムに選ばれた ${n} 問を続けて解き、合計の提出回数を競います。ギブアップした問題は未解決として記録されます。`,
  challengeRules: (mode, lives) => `モード: ${mode} ・ 残機: ${lives}`,
  challengeStart: "チャレンジを始める",
  challengePast: "これまでの記録",
  challengeResult: "連続チャレンジの結果",
  challengeTotal: (t, s, c) => `合計 ${t} 回 ・ ${s}/${c} 問正解`,
  challengeNew: "新しいチャレンジ",
  challengeSkipDeleted: (id) => `問題「${id}」は削除されたため、スキップします。`,
  challengeShareHeader: (mode) => `OneShot Rust 🦀 連続チャレンジ [${mode}]`,
};

export const MESSAGES: Record<Locale, Messages> = { en, ja };
