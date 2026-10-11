// UI messages. English is the default; Japanese is available as an alternative.
import type {
  CaseStatus,
  InternalErrorReason,
  RuntimeErrorReason,
  VerdictKind,
} from "./classify.ts";
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
  roundHeading: string;
  roundIntro: (n: number) => string;
  courseHeading: string;
  /** e.g. "Beginner course" */
  courseTitle: (name: string) => string;
  /** The course of rounds from before courses existed. */
  courseUnknown: string;
  startRound: string;
  continueRound: string;
  continueRoundDetail: (course: string, position: string) => string;
  demoAccepted: (n: number) => string;
  demoWrongAnswer: (passed: number, n: number) => string;
  demoSubmitHint: string;
  madeBy: string;

  // Records
  recordsHeading: string;
  problemsHeading: string;
  practiceHint: string;
  noProblemsSeen: string;
  columnProblem: string;
  columnBest: string;
  roundsHeading: string;
  noRounds: string;
  bestTooltip: (ceCount: number) => string;
  columnDifficulty: string;
  shotsTooltip: (tries: number) => string;
  difficulties: Record<Difficulty, string>;
  kinds: Record<ProblemKind, string>;

  // Play screen
  submit: string;
  judging: string;
  history: string;
  startOver: string;
  giveUp: string;
  nextProblem: string;
  tabStatement: string;
  tabHarness: string;
  tabSolution: string;
  practiceChip: string;
  practiceTooltip: string;
  assistedChip: string;
  assistedTooltip: string;
  roundChip: (position: string) => string;
  toScorecard: string;
  backToRecords: string;
  confirmSubmit: (n: number) => string;
  confirmSubmitYes: string;
  cancel: string;
  confirmStartOver: string;
  startOverAbandons: string;
  confirmStartOverYes: string;
  confirmGiveUp: string;
  confirmGiveUpYes: string;
  problemNotFound: (id: string) => string;

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
  compiling: string;
  runningCase: (i: number, n: number) => string;
  rejected: string;
  forbiddenMacro: (name: string) => string;
  submitError: string;
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
  copyResult: string;
  copied: string;
  shareImage: string;
  imageSaved: string;
  seeSolution: string;
  assistedRun: string;
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
  attemptHeading: (i: number, verdict: string) => string;
  changesFromTemplate: string;
  changesFrom: (i: number) => string;

  // Rounds
  scorecardTitle: string;
  roundTotal: (total: number) => string;
  roundOverPerfect: (n: number) => string;
  roundSolved: (solved: number, count: number) => string;
  roundAbandoned: (solved: number, count: number) => string;
  abandonRound: string;
  confirmAbandonRound: string;
  confirmAbandonRoundYes: string;
  columnTries: string;
  oneMoreRound: string;
  changeCourse: string;
  roundUnfinished: string;
  roundNotFound: string;
  roundSkipDeleted: (id: string) => string;
  roundShareHeader: (course: string) => string;
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
  roundHeading: "Play a round",
  roundIntro: (n) =>
    `${n} problems from one course, back to back. Your score is the total number of submissions: ${n} is perfect.`,
  courseHeading: "Course",
  courseTitle: (name) => `${name} course`,
  courseUnknown: "Random",
  startRound: "Start round",
  continueRound: "Continue",
  continueRoundDetail: (course, position) => `${course} course · problem ${position}`,
  demoAccepted: (n) => `Accepted · ${n}/${n} tests passed`,
  demoWrongAnswer: (passed, n) => `Wrong answer · ${passed}/${n} tests passed`,
  demoSubmitHint: "Submit",
  madeBy: "Made by",

  recordsHeading: "Records",
  problemsHeading: "Problems",
  practiceHint:
    "Pick a problem to practice it on its own. Practice counts toward your best, not toward a round.",
  noProblemsSeen: "Problems you meet in rounds will be listed here.",
  columnProblem: "Problem",
  columnBest: "Best",
  roundsHeading: "Rounds",
  noRounds: "No rounds played yet.",
  bestTooltip: (ce) => `${ce} compile error${ce === 1 ? "" : "s"}`,
  columnDifficulty: "Difficulty",
  shotsTooltip: (tries) => `${tries} ${tries === 1 ? "try" : "tries"} so far · aim for one`,
  difficulties: { 1: "Beginner", 2: "Intermediate", 3: "Advanced" },
  kinds: { implement: "Implement", repair: "Repair", signature: "Signature" },

  submit: "Submit",
  judging: "Judging…",
  history: "Submission history",
  startOver: "Start over",
  giveUp: "Give up",
  nextProblem: "Next problem",
  tabStatement: "Problem",
  tabHarness: "tests.rs (read-only)",
  tabSolution: "Solution",
  practiceChip: "Practice",
  practiceTooltip: "Played on its own: it counts toward your best, not toward a round.",
  assistedChip: "Solution seen",
  assistedTooltip: "You have seen the model solution, so this run does not update your best.",
  roundChip: (p) => `Round ${p}`,
  toScorecard: "See the scorecard",
  backToRecords: "Back to records",
  confirmSubmit: (n) => `Submit this code? This will be submission #${n}.`,
  confirmSubmitYes: "Submit",
  cancel: "Cancel",
  confirmStartOver: "Reset the code to the template and start a new run?",
  startOverAbandons: "The current run will end unsolved.",
  confirmStartOverYes: "Start over",
  confirmGiveUp:
    "Give up this problem? It is recorded as unsolved, and the model solution unlocks.",
  confirmGiveUpYes: "Give up",
  problemNotFound: (id) => `Problem “${id}” was not found.`,

  solutionLocked: "The model solution unlocks once you finish this problem: solve it or give up.",
  solutionGiveUpHint: "Stuck? You can give up and look at it now.",
  solutionGiveUp: "Give up and show",
  confirmRevealSolution:
    "Show the model solution? You can keep going, but from now on runs on this problem do not update your best.",
  confirmRevealSolutionYes: "Show solution",
  solutionUnlocked:
    "Compare your code with the model solution. Once you look at it, later runs on this problem do not update your best.",
  showSolution: "Show solution",
  solutionLoading: "Loading…",
  solutionLoadError: (m) => `Could not load the solution: ${m}`,
  solutionCode: "Code",
  solutionDiff: "Diff from your code",
  solutionDiffLegend: "- your code   + model solution",

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
  copyResult: "Copy result",
  copied: "Copied!",
  shareImage: "Share image",
  imageSaved: "Image saved",
  seeSolution: "Model solution",
  assistedRun: "Solution seen: your best is not updated.",
  tryAgain: "Try again",
  close: "Close",
  shareTries: (n) => `${n} ${n === 1 ? "try" : "tries"}`,
  unsolved: "unsolved",

  historyTitle: "Submission history",
  noSubmissions: "No submissions yet.",
  runSolved: (n) => `Solved in ${n}`,
  runFailed: "Ended unsolved",
  runPlaying: "In progress",
  attemptHeading: (i, v) => `Submission #${i} — ${v}`,
  changesFromTemplate: "Changes from the template",
  changesFrom: (i) => `Changes from submission #${i}`,

  scorecardTitle: "Scorecard",
  roundTotal: (t) => `Total ${t}`,
  roundOverPerfect: (n) => (n === 0 ? "Perfect!" : `+${n} over perfect`),
  roundSolved: (s, c) => `${s}/${c} solved`,
  roundAbandoned: (s, c) => `Abandoned (${s}/${c} solved)`,
  abandonRound: "Abandon this round",
  confirmAbandonRound:
    "Abandon this round? It ends now, with the problems not solved yet counted as unsolved.",
  confirmAbandonRoundYes: "Abandon",
  columnTries: "Tries",
  oneMoreRound: "One more round",
  changeCourse: "Change course",
  roundUnfinished: "This round is not finished yet.",
  roundNotFound: "This round was not found.",
  roundSkipDeleted: (id) => `Problem “${id}” no longer exists and is skipped.`,
  roundShareHeader: (course) => `OneShot Rust 🦀 ${course}`,
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

  heroTitle: "Rust を、一発で。",
  heroTagline:
    "補完なし、IDE なし。あなたとコンパイラだけ。コンパイルは 1 回ごとにカウントされます。",
  roundHeading: "ラウンドに挑戦",
  roundIntro: (n) =>
    `コースから ${n} 問を続けて解きます。スコアは合計の提出回数で、${n} 回なら完璧です。`,
  courseHeading: "コース",
  courseTitle: (name) => `${name}コース`,
  courseUnknown: "ランダム",
  startRound: "ラウンドを始める",
  continueRound: "続きから",
  continueRoundDetail: (course, position) => `${course}コース · ${position} 問目`,
  demoAccepted: (n) => `正解 · ${n}/${n} テスト通過`,
  demoWrongAnswer: (passed, n) => `不正解 · ${passed}/${n} テスト通過`,
  demoSubmitHint: "提出",
  madeBy: "作者：",

  recordsHeading: "記録",
  problemsHeading: "問題",
  practiceHint:
    "問題を選ぶと、その問題だけを練習できます。練習の結果はベストに残りますが、ラウンドにはなりません。",
  noProblemsSeen: "ラウンドで出会った問題がここに並びます。",
  columnProblem: "問題",
  columnBest: "ベスト",
  roundsHeading: "ラウンド",
  noRounds: "まだラウンドを遊んでいません。",
  bestTooltip: (ce) => `うち CE ${ce} 回`,
  columnDifficulty: "難易度",
  shotsTooltip: (tries) => `現在 ${tries} 回（目標は 1 回）`,
  difficulties: { 1: "初級", 2: "中級", 3: "上級" },
  kinds: { implement: "実装", repair: "修理", signature: "シグネチャ" },

  submit: "提出",
  judging: "判定中…",
  history: "提出履歴",
  startOver: "最初から",
  giveUp: "ギブアップ",
  nextProblem: "次の問題へ",
  tabStatement: "問題文",
  tabHarness: "tests.rs（読み取り専用）",
  tabSolution: "模範解答",
  practiceChip: "練習",
  practiceTooltip: "単独の練習：ベストには残りますが、ラウンドにはなりません。",
  assistedChip: "解答閲覧済み",
  assistedTooltip: "模範解答を見たあとの挑戦なので、自己ベストは更新されません。",
  roundChip: (p) => `ラウンド ${p}`,
  toScorecard: "スコアカードへ",
  backToRecords: "記録へ戻る",
  confirmSubmit: (n) => `この内容で提出しますか？（${n} 回目の提出）`,
  confirmSubmitYes: "提出する",
  cancel: "やめる",
  confirmStartOver: "コードをテンプレートに戻し、新しいランを始めますか？",
  startOverAbandons: "進行中のランは未解決で終了します。",
  confirmStartOverYes: "最初からやり直す",
  confirmGiveUp:
    "この問題をギブアップしますか？（未解決として記録され、模範解答を見られるようになります）",
  confirmGiveUpYes: "ギブアップする",
  problemNotFound: (id) => `問題「${id}」は見つかりませんでした。`,

  solutionLocked:
    "この問題を終える（正解するか、ギブアップする）と、模範解答を見られるようになります。",
  solutionGiveUpHint: "どうしても解けないときは、ギブアップして今すぐ見ることもできます。",
  solutionGiveUp: "ギブアップして見る",
  confirmRevealSolution:
    "模範解答を表示しますか？ 挑戦はこのまま続けられますが、以降この問題では自己ベストが更新されません。",
  confirmRevealSolutionYes: "表示する",
  solutionUnlocked:
    "模範解答と見比べてみましょう。表示すると、以降この問題では自己ベストが更新されません。",
  showSolution: "模範解答を表示",
  solutionLoading: "読み込み中…",
  solutionLoadError: (m) => `模範解答を読み込めませんでした: ${m}`,
  solutionCode: "コード",
  solutionDiff: "自分のコードとの差分",
  solutionDiffLegend: "- 自分のコード   + 模範解答",

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
  copyResult: "結果をコピー",
  copied: "コピーしました",
  shareImage: "画像でシェア",
  imageSaved: "画像を保存しました",
  seeSolution: "模範解答",
  assistedRun: "模範解答を閲覧済みのため、自己ベストは更新されません。",
  tryAgain: "もう一度",
  close: "閉じる",
  shareTries: (n) => `${n}回`,
  unsolved: "未解決",

  historyTitle: "提出履歴",
  noSubmissions: "まだ提出がありません。",
  runSolved: (n) => `${n} 回で正解`,
  runFailed: "未解決で終了",
  runPlaying: "挑戦中",
  attemptHeading: (i, v) => `${i} 回目の提出 — ${v}`,
  changesFromTemplate: "テンプレートからの変更",
  changesFrom: (i) => `${i} 回目からの変更`,

  scorecardTitle: "スコアカード",
  roundTotal: (t) => `合計 ${t}`,
  roundOverPerfect: (n) => (n === 0 ? "完璧！" : `完璧まで +${n}`),
  roundSolved: (s, c) => `${s}/${c} 問正解`,
  roundAbandoned: (s, c) => `棄権（${s}/${c} 問正解）`,
  abandonRound: "このラウンドを棄権",
  confirmAbandonRound:
    "このラウンドを棄権しますか？ラウンドはここで終了し、まだ解いていない問題は未解決として記録されます。",
  confirmAbandonRoundYes: "棄権する",
  columnTries: "回数",
  oneMoreRound: "もう 1 ラウンド",
  changeCourse: "コースを変える",
  roundUnfinished: "このラウンドはまだ終わっていません。",
  roundNotFound: "ラウンドが見つかりませんでした。",
  roundSkipDeleted: (id) => `問題「${id}」は削除されたため、スキップします。`,
  roundShareHeader: (course) => `OneShot Rust 🦀 ${course}`,
};

export const MESSAGES: Record<Locale, Messages> = { en, ja };
