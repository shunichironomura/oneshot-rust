// rustc / Miri diagnostics in `--error-format=json`.

export interface DiagnosticSpanLine {
  text: string;
  highlight_start: number;
  highlight_end: number;
}

export interface DiagnosticSpan {
  file_name: string;
  line_start: number;
  line_end: number;
  column_start: number;
  column_end: number;
  is_primary: boolean;
  text: DiagnosticSpanLine[];
  label: string | null;
  suggested_replacement: string | null;
}

export type DiagnosticLevel =
  | "error"
  | "warning"
  | "note"
  | "help"
  | "failure-note"
  | "error: internal compiler error";

export interface Diagnostic {
  message: string;
  code: { code: string } | null;
  level: DiagnosticLevel;
  spans: DiagnosticSpan[];
  children: Diagnostic[];
  rendered: string | null;
}

export interface ParsedStderr {
  diagnostics: Diagnostic[];
  /** Lines that are not JSON diagnostics (e.g. the player's `eprintln!`). */
  text: string;
}

function isDiagnostic(value: unknown): value is Diagnostic {
  if (typeof value !== "object" || value === null) return false;
  const v = value as Record<string, unknown>;
  return (
    typeof v["message"] === "string" &&
    typeof v["level"] === "string" &&
    Array.isArray(v["spans"]) &&
    Array.isArray(v["children"])
  );
}

function tryParseDiagnostic(line: string): Diagnostic | null {
  if (!line.startsWith("{")) return null;
  try {
    const value: unknown = JSON.parse(line);
    return isDiagnostic(value) ? value : null;
  } catch {
    return null;
  }
}

export function parseStderr(stderr: string): ParsedStderr {
  const diagnostics: Diagnostic[] = [];
  const textLines: string[] = [];
  for (const line of stderr.split("\n")) {
    const d = tryParseDiagnostic(line);
    if (d === null) textLines.push(line);
    else diagnostics.push(d);
  }
  return { diagnostics, text: textLines.join("\n").replace(/\n+$/, "") };
}

/** The trailing "aborting due to N previous errors" summary is not an error of its own. */
function isSummary(d: Diagnostic): boolean {
  return d.spans.length === 0 && d.message.startsWith("aborting due to ");
}

export function errorsOf(diagnostics: Diagnostic[]): Diagnostic[] {
  return diagnostics.filter((d) => d.level === "error" && !isSummary(d));
}

export function warningsOf(diagnostics: Diagnostic[]): Diagnostic[] {
  return diagnostics.filter(
    (d) => d.level === "warning" && !/^\d+ warnings? emitted$/.test(d.message),
  );
}

export function isInternalCompilerError(d: Diagnostic): boolean {
  return d.level === "error: internal compiler error";
}

export function isUndefinedBehavior(d: Diagnostic): boolean {
  return d.level === "error" && d.message.startsWith("Undefined Behavior");
}

export function primarySpan(d: Diagnostic): DiagnosticSpan | undefined {
  return d.spans.find((s) => s.is_primary) ?? d.spans[0];
}
