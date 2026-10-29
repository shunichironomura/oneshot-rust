// UI messages. English is the default; Japanese is available as an alternative.
import type {
  CaseStatus,
  InternalErrorReason,
  RuntimeErrorReason,
  VerdictKind,
} from "./classify.ts";
import type { Difficulty } from "./problem.ts";

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
  navRecords: string;
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
  play: string;
  continueRound: string;
  /** e.g. "Problem 2 of 3" */
  roundPosition: (i: number, n: number) => string;
  /** Leads the difficulties of the problems of a round, by the Play button. */
  roundLadder: (n: number) => string;
  /** Leads the difficulties to play a single problem of, below the Play button. */
  quickPlay: string;
  demoAccepted: (n: number) => string;
  demoWrongAnswer: (passed: number, n: number) => string;
  demoSubmitHint: string;
  madeBy: string;
  sourceAndLicenses: string;
  thirdPartyLicenses: string;
  aiNotice: string;

  // Records
  recordsHeading: string;
  problemsHeading: string;
  noProblemsSeen: string;
  roundsHeading: string;
  noRounds: string;
  bestTooltip: (ceCount: number) => string;
  shotsTooltip: (tries: number) => string;
  difficulties: Record<Difficulty, string>;

  // Play screen
  submit: string;
  judging: string;
  history: string;
  giveUp: string;
  /** Runs the code without scoring, after the run has ended. */
  tryRun: string;
  tryRunTooltip: string;
  nextProblem: string;
  tabStatement: string;
  tabHarness: string;
  tabSolution: string;
  practiceChip: string;
  practiceTooltip: string;
  assistedChip: string;
  assistedTooltip: string;
  toScorecard: string;
  backToRecords: string;
  confirmSubmit: (n: number) => string;
  confirmSubmitYes: string;
  cancel: string;
  confirmGiveUp: string;
  confirmGiveUpYes: string;
  problemNotFound: (id: string) => string;

  // Model solution
  solutionLoading: string;
  solutionLoadError: (message: string) => string;
  solutionCode: string;
  solutionDiff: string;
  /** Names the choice between the solution's code and its diff. */
  solutionView: string;
  solutionDiffLegend: string;
  solutionAiNotice: string;

  // Result panel
  compiling: string;
  runningCase: (i: number, n: number) => string;
  rejected: string;
  forbiddenMacro: (name: string) => string;
  submitError: string;
  /** Marks the result of a trial run. */
  unscoredResult: string;
  output: string;
  hiddenCase: string;
  caseStatus: Record<CaseStatus, string>;
  verdicts: Record<VerdictKind, string>;
  hiddenHarnessNote: string;
  harnessInternalsNote: string;
  tleSummary: (caseId: string) => string;
  runtimeError: (reason: RuntimeErrorReason) => string;
  internalError: (reason: InternalErrorReason) => string;

  // End of a run
  solvedHeadline: string;
  unsolvedHeadline: string;
  newBest: string;
  /** Shares a result (or copies it, where there is no share sheet). */
  share: string;
  copied: string;
  /** Copies a code block or a message to the clipboard. */
  copy: string;
  copyFailed: string;
  /** Copies a submission's whole code (shown next to its diff). */
  copyCode: string;
  seeSolution: string;
  assistedRun: string;
  tryAgain: string;
  close: string;
  shareTries: (n: number) => string;
  unsolved: string;

  // History
  /** Plays the problem on its own. */
  practice: string;
  noSubmissions: string;
  runPlaying: string;
  /** Names the list of a run's submissions. */
  attemptsLabel: string;
  attemptHeading: (i: number, verdict: string) => string;
  changesFromTemplate: string;
  changesFrom: (i: number) => string;

  // Rounds
  roundTotal: (total: number) => string;
  roundOverPerfect: (n: number) => string;
  roundSolved: (solved: number, count: number) => string;
  roundAbandoned: (solved: number, count: number) => string;
  abandonRound: string;
  confirmAbandonRound: string;
  confirmAbandonRoundYes: string;
  oneMoreRound: string;
  roundUnfinished: string;
  roundNotFound: string;
  roundSkipDeleted: (id: string) => string;
  /** The row labels of the scorecard. */
  card: { hole: string; problem: string; level: string; par: string; shots: string; total: string };
}

const en: Messages = {
  navRecords: "Records",
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
  play: "Play",
  continueRound: "Continue",
  roundPosition: (i, n) => `Problem ${i} of ${n}`,
  roundLadder: (n) => `${n} problems:`,
  quickPlay: "Or just one problem:",
  demoAccepted: (n) => `Accepted · ${n}/${n} tests passed`,
  demoWrongAnswer: (passed, n) => `Wrong answer · ${passed}/${n} tests passed`,
  demoSubmitHint: "Submit",
  madeBy: "Made by",
  sourceAndLicenses: "Source code and licenses",
  thirdPartyLicenses: "Third-party licenses",
  aiNotice:
    "Problems, model solutions and editorials were written with AI assistance. Every model solution is checked to compile and pass all tests under Miri, but the explanations may contain mistakes and the code may not be the most idiomatic.",

  recordsHeading: "Records",
  problemsHeading: "Problems",
  noProblemsSeen: "Problems you meet in rounds will be listed here.",
  roundsHeading: "Rounds",
  noRounds: "No rounds played yet.",
  bestTooltip: (ce) => `${ce} compile error${ce === 1 ? "" : "s"}`,
  shotsTooltip: (tries) => `${tries} ${tries === 1 ? "try" : "tries"} so far · aim for one`,
  difficulties: { 1: "Beginner", 2: "Intermediate", 3: "Advanced" },

  submit: "Submit",
  judging: "Judging…",
  history: "Submission history",
  giveUp: "Give up",
  tryRun: "Run",
  tryRunTooltip:
    "Runs your code without scoring: nothing is recorded. Try the model solution's ideas.",
  nextProblem: "Next problem",
  tabStatement: "Problem",
  tabHarness: "tests.rs",
  tabSolution: "Solution",
  practiceChip: "Practice",
  practiceTooltip: "Played on its own: it counts toward your best, not toward a round.",
  assistedChip: "Solution seen",
  assistedTooltip: "You have seen the model solution, so this run does not update your best.",
  toScorecard: "See the scorecard",
  backToRecords: "Back to records",
  confirmSubmit: (n) => `Submit #${n}?`,
  confirmSubmitYes: "Submit",
  cancel: "Cancel",
  confirmGiveUp: "Give up this problem?",
  confirmGiveUpYes: "Give up",
  problemNotFound: (id) => `Problem “${id}” was not found.`,

  solutionLoading: "Loading…",
  solutionLoadError: (m) => `Could not load the solution: ${m}`,
  solutionCode: "Code",
  solutionDiff: "Diff from your code",
  solutionView: "View",
  solutionDiffLegend: "- your code   + model solution",
  solutionAiNotice:
    "This solution and editorial were written with AI assistance. The explanation may contain mistakes.",

  compiling: "Compiling…",
  runningCase: (i, n) => `Running test ${i} of ${n}…`,
  rejected: "Not submitted (not counted)",
  forbiddenMacro: (m) => `\`${m}!\` is not allowed: it could read the judge's secrets.`,
  submitError: "Error (not counted)",
  unscoredResult: "Not scored",
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
  newBest: "New personal best!",
  share: "Share",
  copied: "Copied!",
  copy: "Copy",
  copyFailed: "Could not copy",
  copyCode: "Copy code",
  seeSolution: "Model solution",
  assistedRun: "Solution seen: your best is not updated.",
  tryAgain: "Try again",
  close: "Close",
  shareTries: (n) => `${n} ${n === 1 ? "try" : "tries"}`,
  unsolved: "unsolved",

  practice: "Practice",
  noSubmissions: "No submissions yet.",
  runPlaying: "In progress",
  attemptsLabel: "Submissions",
  attemptHeading: (i, v) => `Submission #${i} — ${v}`,
  changesFromTemplate: "Changes from the template",
  changesFrom: (i) => `Changes from submission #${i}`,

  roundTotal: (t) => `Total ${t}`,
  roundOverPerfect: (n) => (n === 0 ? "Perfect!" : `+${n} over perfect`),
  roundSolved: (s, c) => `${s}/${c} solved`,
  roundAbandoned: (s, c) => `Abandoned (${s}/${c} solved)`,
  abandonRound: "Abandon this round",
  confirmAbandonRound: "Abandon this round? The remaining problems count as unsolved.",
  confirmAbandonRoundYes: "Abandon",
  oneMoreRound: "One more round",
  roundUnfinished: "This round is not finished yet.",
  roundNotFound: "This round was not found.",
  roundSkipDeleted: (id) => `Problem “${id}” no longer exists and is skipped.`,
  card: {
    hole: "Hole",
    problem: "Problem",
    level: "Level",
    par: "Par",
    shots: "Shots",
    total: "Total",
  },
};

const ja: Messages = {
  navRecords: "記録",
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

  // The zero-width space is the only place the title may wrap (see `.hero h1`).
  heroTitle: "Rust を、\u200b一発で。",
  heroTagline:
    "補完なし、IDE なし。あなたとコンパイラだけ。コンパイルは 1 回ごとにカウントされます。",
  play: "プレイ",
  continueRound: "続きから",
  roundPosition: (i, n) => `${n} 問中 ${i} 問目`,
  roundLadder: (n) => `全 ${n} 問：`,
  quickPlay: "1 問だけ解く：",
  demoAccepted: (n) => `正解 · ${n}/${n} テスト通過`,
  demoWrongAnswer: (passed, n) => `不正解 · ${passed}/${n} テスト通過`,
  demoSubmitHint: "提出",
  madeBy: "作者：",
  sourceAndLicenses: "ソースコードとライセンス",
  thirdPartyLicenses: "サードパーティのライセンス",
  aiNotice:
    "問題・模範解答・解説は AI の支援を受けて作成しています。模範解答はすべて Miri でコンパイルが通り、全テストに合格することを確認していますが、解説の説明に誤りがあったり、より慣用的な書き方があったりするかもしれません。",

  recordsHeading: "記録",
  problemsHeading: "問題",
  noProblemsSeen: "ラウンドで出会った問題がここに並びます。",
  roundsHeading: "ラウンド",
  noRounds: "まだラウンドを遊んでいません。",
  bestTooltip: (ce) => `うち CE ${ce} 回`,
  shotsTooltip: (tries) => `現在 ${tries} 回（目標は 1 回）`,
  difficulties: { 1: "初級", 2: "中級", 3: "上級" },

  submit: "提出",
  judging: "判定中…",
  history: "提出履歴",
  giveUp: "ギブアップ",
  tryRun: "実行",
  tryRunTooltip:
    "採点なしで実行します。記録には残りません。模範解答を参考に手を動かしてみましょう。",
  nextProblem: "次の問題へ",
  tabStatement: "問題文",
  tabHarness: "tests.rs",
  tabSolution: "模範解答",
  practiceChip: "練習",
  practiceTooltip: "単独の練習：ベストには残りますが、ラウンドにはなりません。",
  assistedChip: "解答閲覧済み",
  assistedTooltip: "模範解答を見たあとの挑戦なので、自己ベストは更新されません。",
  toScorecard: "スコアカードへ",
  backToRecords: "記録へ戻る",
  confirmSubmit: (n) => `${n} 回目の提出をしますか？`,
  confirmSubmitYes: "提出する",
  cancel: "やめる",
  confirmGiveUp: "この問題をギブアップしますか？",
  confirmGiveUpYes: "ギブアップする",
  problemNotFound: (id) => `問題「${id}」は見つかりませんでした。`,

  solutionLoading: "読み込み中…",
  solutionLoadError: (m) => `模範解答を読み込めませんでした: ${m}`,
  solutionCode: "コード",
  solutionDiff: "自分のコードとの差分",
  solutionView: "表示",
  solutionDiffLegend: "- 自分のコード   + 模範解答",
  solutionAiNotice:
    "この模範解答と解説は AI の支援を受けて作成しています。解説には誤りがあるかもしれません。",

  compiling: "コンパイル中…",
  runningCase: (i, n) => `実行中（ケース ${i}/${n}）`,
  rejected: "提出できません（カウントされません）",
  forbiddenMacro: (m) => `\`${m}!\` は使用できません（判定の改ざん防止のため）`,
  submitError: "エラー（カウントされません）",
  unscoredResult: "採点対象外",
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
  newBest: "自己ベスト更新！",
  share: "シェア",
  copied: "コピーしました",
  copy: "コピー",
  copyFailed: "コピーできませんでした",
  copyCode: "コードをコピー",
  seeSolution: "模範解答",
  assistedRun: "模範解答を閲覧済みのため、自己ベストは更新されません。",
  tryAgain: "もう一度",
  close: "閉じる",
  shareTries: (n) => `${n}回`,
  unsolved: "未解決",

  practice: "練習する",
  noSubmissions: "まだ提出がありません。",
  runPlaying: "挑戦中",
  attemptsLabel: "提出",
  attemptHeading: (i, v) => `${i} 回目の提出 — ${v}`,
  changesFromTemplate: "テンプレートからの変更",
  changesFrom: (i) => `${i} 回目からの変更`,

  roundTotal: (t) => `合計 ${t}`,
  roundOverPerfect: (n) => (n === 0 ? "完璧！" : `完璧まで +${n}`),
  roundSolved: (s, c) => `${s}/${c} 問正解`,
  roundAbandoned: (s, c) => `棄権（${s}/${c} 問正解）`,
  abandonRound: "このラウンドを棄権",
  confirmAbandonRound: "このラウンドを棄権しますか？残りの問題は未解決になります。",
  confirmAbandonRoundYes: "棄権する",
  oneMoreRound: "もう 1 ラウンド",
  roundUnfinished: "このラウンドはまだ終わっていません。",
  roundNotFound: "ラウンドが見つかりませんでした。",
  roundSkipDeleted: (id) => `問題「${id}」は削除されたため、スキップします。`,
  card: {
    hole: "ホール",
    problem: "問題",
    level: "難易度",
    par: "パー",
    shots: "打数",
    total: "合計",
  },
};

export const MESSAGES: Record<Locale, Messages> = { en, ja };
