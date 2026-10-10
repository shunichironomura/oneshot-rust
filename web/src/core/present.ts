// Turns a judgement into what the player gets to see, depending on the mode.
import type { CaseResult, CaseStatus, Judgement, VerdictKind } from "./classify.ts";
import { type Diagnostic, type DiagnosticSpan, primarySpan } from "./diagnostics.ts";
import { SRC_DIR } from "./harness.ts";
import type { Messages } from "./i18n.ts";
import type { Mode } from "./mode.ts";

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
  /** `null` when case results are hidden in this mode. */
  cases: CaseView[] | null;
  markers: EditorMarker[];
  /** The player's own output. Judge-generated panic reports are hidden in strict modes. */
  output: { stdout: string; stderr: string };
}

export interface PresentContext {
  mode: Mode;
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

function involvesHiddenCode(d: Diagnostic, ctx: PresentContext): boolean {
  return [d, ...d.children].some((x) => x.spans.some((s) => isSnippetHidden(s, ctx)));
}

function renderFull(d: Diagnostic, ctx: PresentContext): string {
  // rustc's own rendering includes help and suggestions; it is only used when it
  // does not reveal hidden harness code.
  if (d.rendered !== null && !involvesHiddenCode(d, ctx)) {
    return cleanPaths(d.rendered).replace(/\n+$/, "");
  }
  const helps = d.children
    .filter((c) => c.level === "help")
    .map((c) => `= help: ${cleanPaths(c.message)}`);
  return [renderDiagnostic(d, ctx), ...helps].join("\n");
}

function lineSummary(diagnostics: Diagnostic[], ctx: PresentContext): string {
  const locations = diagnostics.map((d) => {
    const span = primarySpan(d);
    if (span === undefined) return ctx.messages.unknownLocation;
    if (isSnippetHidden(span, ctx))
      return `${displayPath(span.file_name)}${ctx.messages.inHarness}`;
    return `${displayPath(span.file_name)}:${span.line_start}`;
  });
  return [...new Set(locations)].join(", ");
}

function renderDiagnostics(diagnostics: Diagnostic[], ctx: PresentContext): string[] {
  switch (ctx.mode) {
    case "beginner":
      return diagnostics.map((d) => renderFull(d, ctx));
    case "standard":
      return diagnostics.map((d) => renderDiagnostic(d, ctx));
    case "advanced":
    case "oni":
      return [];
  }
}

function presentCases(cases: CaseResult[], mode: Mode): CaseView[] | null {
  switch (mode) {
    case "beginner":
      return cases.map((c) => ({ ...c, message: c.message && cleanPaths(c.message) }));
    case "standard":
      return cases.map((c) => ({
        ...c,
        message: c.visible && c.message ? cleanPaths(c.message) : undefined,
      }));
    case "advanced":
      return cases.map(({ id, visible, status }) => ({ id, visible, status }));
    case "oni":
      return null;
  }
}

function markers(diagnostics: Diagnostic[], mode: Mode): EditorMarker[] {
  if (mode !== "beginner") return [];
  return diagnostics.flatMap((d) => {
    const span = primarySpan(d);
    return span !== undefined && fileRole(span.file_name) === "user"
      ? [{ line: span.line_start, message: d.message }]
      : [];
  });
}

/** Panics outside of test cases are reported on stderr by the judge as `panicked: ...`. */
function visibleStderr(stderr: string, mode: Mode): string {
  const strict = mode === "advanced" || mode === "oni";
  const lines = stderr.split("\n").filter((l) => !(strict && l.startsWith("panicked: ")));
  return cleanPaths(lines.join("\n")).trim();
}

export function present(judgement: Judgement, ctx: PresentContext): Presentation {
  const v = judgement.verdict;
  const { mode, messages } = ctx;
  const detailed = mode === "beginner" || mode === "standard";
  const base = {
    kind: v.kind,
    diagnostics: [] as string[],
    cases: null,
    markers: [] as EditorMarker[],
    output: { stdout: judgement.output, stderr: visibleStderr(judgement.stderr, mode) },
  };

  switch (v.kind) {
    case "CE":
      return {
        ...base,
        summary:
          mode === "oni"
            ? undefined
            : messages.ceSummary(v.diagnostics.length, lineSummary(v.diagnostics, ctx)),
        diagnostics: renderDiagnostics(v.diagnostics, ctx),
        markers: markers(v.diagnostics, mode),
      };
    case "AC":
    case "WA":
      return { ...base, cases: presentCases(v.cases, mode) };
    case "UB":
      return {
        ...base,
        summary: mode === "oni" ? undefined : lineSummary(v.diagnostics, ctx),
        diagnostics: renderDiagnostics(v.diagnostics, ctx),
        cases: presentCases(v.cases, mode),
        markers: markers(v.diagnostics, mode),
      };
    case "RE":
      return {
        ...base,
        summary: detailed ? cleanPaths(messages.runtimeError(v.reason)) : undefined,
        diagnostics: renderDiagnostics(v.diagnostics, ctx),
        cases: presentCases(v.cases, mode),
      };
    case "TLE":
      return {
        ...base,
        summary:
          mode !== "oni" && v.runningCase !== undefined
            ? messages.tleSummary(v.runningCase)
            : undefined,
        cases: presentCases(v.cases, mode),
      };
    case "IE":
      return { ...base, summary: messages.internalError(v.reason) };
  }
}
