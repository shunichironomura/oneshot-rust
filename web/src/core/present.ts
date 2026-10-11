// Turns a judgement into what the player gets to see. Everyone sees the same: the
// compiler's errors with their code and notes, but not rustc's help and suggested fixes
// (they would work like an IDE's quick fixes), and failure messages of public cases only.
import type { CaseResult, CaseStatus, Judgement, VerdictKind } from "./classify.ts";
import { type Diagnostic, type DiagnosticSpan, primarySpan } from "./diagnostics.ts";
import { SRC_DIR } from "./harness.ts";
import type { Messages } from "./i18n.ts";

export interface CaseView {
  id: string;
  visible: boolean;
  status: CaseStatus;
  message?: string;
}

export interface EditorMarker {
  line: number;
  message: string;
}

export interface Presentation {
  kind: VerdictKind;
  /** One-line explanation below the verdict. */
  summary?: string;
  /** Pre-formatted diagnostic blocks. */
  diagnostics: string[];
  /** `null` for verdicts without test results (compile and internal errors). */
  cases: CaseView[] | null;
  /** Lines of the player's code that the diagnostics point at. */
  markers: EditorMarker[];
  /** The player's own output. */
  output: { stdout: string; stderr: string };
}

export interface PresentContext {
  harnessVisible: boolean;
  messages: Messages;
}

type FileRole = "user" | "harness" | "internal" | "external";

function fileRole(fileName: string): FileRole {
  switch (fileName) {
    case `${SRC_DIR}/user.rs`:
      return "user";
    case `${SRC_DIR}/tests.rs`:
      return "harness";
    case `${SRC_DIR}/main.rs`:
    case `${SRC_DIR}/judge.rs`:
      return "internal";
    default:
      return "external";
  }
}

export function displayPath(fileName: string): string {
  return fileName.startsWith(`${SRC_DIR}/`) ? fileName.slice(SRC_DIR.length + 1) : fileName;
}

/** Rewrites `/src/user.rs` into `user.rs` in free text such as panic messages. */
export function cleanPaths(text: string): string {
  return text.replaceAll(`${SRC_DIR}/`, "");
}

function isSnippetHidden(span: DiagnosticSpan, ctx: PresentContext): boolean {
  const role = fileRole(span.file_name);
  return role === "internal" || (role === "harness" && !ctx.harnessVisible);
}

function location(span: DiagnosticSpan): string {
  return `${displayPath(span.file_name)}:${span.line_start}:${span.column_start}`;
}

function hiddenLocationNote(span: DiagnosticSpan, messages: Messages): string {
  return fileRole(span.file_name) === "internal"
    ? messages.harnessInternalsNote
    : messages.hiddenHarnessNote;
}

function renderSpanSnippet(span: DiagnosticSpan, gutter: number): string[] {
  const pad = " ".repeat(gutter);
  const mark = span.is_primary ? "^" : "-";
  const out: string[] = [];
  span.text.forEach((line, i) => {
    const lineNo = String(span.line_start + i).padStart(gutter);
    out.push(`${lineNo} | ${line.text}`);
    const width = Math.max(1, line.highlight_end - line.highlight_start);
    const isLast = i === span.text.length - 1;
    const label = isLast && span.label ? ` ${span.label}` : "";
    out.push(`${pad} | ${" ".repeat(line.highlight_start - 1)}${mark.repeat(width)}${label}`);
  });
  return out;
}

/** rustc-like rendering without `help` children and suggestions. */
export function renderDiagnostic(d: Diagnostic, ctx: PresentContext): string {
  const code = d.code ? `[${d.code.code}]` : "";
  const lines = [`${d.level}${code}: ${d.message}`];
  const spans = [...d.spans].sort((a, b) => Number(b.is_primary) - Number(a.is_primary));
  const gutter = Math.max(1, ...spans.map((s) => String(s.line_end).length));
  const pad = " ".repeat(gutter);
  const hiddenLines = new Set<string>();
  for (const span of spans) {
    if (isSnippetHidden(span, ctx)) {
      // Secondary spans in hidden code would only repeat the same uninformative line.
      const line = `${pad}--> ${displayPath(span.file_name)} ${hiddenLocationNote(span, ctx.messages)}`;
      if (!hiddenLines.has(line)) lines.push(line);
      hiddenLines.add(line);
      continue;
    }
    lines.push(`${pad}--> ${location(span)}`);
    if (span.text.length > 0) {
      lines.push(`${pad} |`, ...renderSpanSnippet(span, gutter));
    }
  }
  for (const child of d.children) {
    if (child.level !== "note") continue;
    const at = primarySpan(child);
    const where = at === undefined || isSnippetHidden(at, ctx) ? "" : ` (${location(at)})`;
    lines.push(`${pad} = note: ${cleanPaths(child.message)}${where}`);
  }
  return lines.join("\n");
}

/** Failure messages of hidden cases would give the hidden tests away. */
function presentCases(cases: CaseResult[]): CaseView[] {
  return cases.map((c) => ({
    ...c,
    message: c.visible && c.message ? cleanPaths(c.message) : undefined,
  }));
}

function markers(diagnostics: Diagnostic[]): EditorMarker[] {
  return diagnostics.flatMap((d) => {
    const span = primarySpan(d);
    return span !== undefined && fileRole(span.file_name) === "user"
      ? [{ line: span.line_start, message: d.message }]
      : [];
  });
}

export function present(judgement: Judgement, ctx: PresentContext): Presentation {
  const v = judgement.verdict;
  const { messages } = ctx;
  const base = {
    kind: v.kind,
    diagnostics: [] as string[],
    cases: null,
    markers: [] as EditorMarker[],
    output: { stdout: judgement.output, stderr: cleanPaths(judgement.stderr).trim() },
  };
  const rendered = (diagnostics: Diagnostic[]) => diagnostics.map((d) => renderDiagnostic(d, ctx));

  switch (v.kind) {
    case "CE":
      return { ...base, diagnostics: rendered(v.diagnostics), markers: markers(v.diagnostics) };
    case "AC":
    case "WA":
      return { ...base, cases: presentCases(v.cases) };
    case "UB":
      return {
        ...base,
        diagnostics: rendered(v.diagnostics),
        cases: presentCases(v.cases),
        markers: markers(v.diagnostics),
      };
    case "RE":
      return {
        ...base,
        summary: cleanPaths(messages.runtimeError(v.reason)),
        diagnostics: rendered(v.diagnostics),
        cases: presentCases(v.cases),
      };
    case "TLE":
      return {
        ...base,
        summary: v.runningCase === undefined ? undefined : messages.tleSummary(v.runningCase),
        cases: presentCases(v.cases),
      };
    case "IE":
      return { ...base, summary: messages.internalError(v.reason) };
  }
}
