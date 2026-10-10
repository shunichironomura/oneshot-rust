// Parser for the judge protocol emitted by `judge/judge.rs` on stdout.

export interface CaseOutcome {
  id: string;
  pass: boolean;
  message?: string;
}

export interface ParsedStdout {
  started: boolean;
  ended: boolean;
  cases: CaseOutcome[];
  /** Everything on stdout that is not a protocol line (the player's own output). */
  userOutput: string;
}

export function unescapeMessage(s: string): string {
  return s.replace(/\\([\\nr])/g, (_, c: string) => (c === "n" ? "\n" : c === "r" ? "\r" : "\\"));
}

type ProtocolEvent =
  | { type: "start" }
  | { type: "end" }
  | { type: "case"; outcome: CaseOutcome }
  | { type: "unknown" };

function parseBody(body: string): ProtocolEvent {
  if (body === "START") return { type: "start" };
  if (body === "END") return { type: "end" };
  const m = /^CASE:([^:]+):(PASS|FAIL)(?::(.*))?$/s.exec(body);
  if (m === null) return { type: "unknown" };
  const [, id, status, message] = m;
  return status === "PASS"
    ? { type: "case", outcome: { id, pass: true } }
    : { type: "case", outcome: { id, pass: false, message: unescapeMessage(message ?? "") } };
}

export function parseStdout(stdout: string, nonce: string): ParsedStdout {
  const marker = `\x01J:${nonce}:`;
  const result: ParsedStdout = { started: false, ended: false, cases: [], userOutput: "" };
  const lines = stdout.split("\n");
  // A trailing segment without "\n" is either empty or an incomplete line.
  const last = lines.pop() ?? "";
  const userLines: string[] = [];
  for (const line of lines) {
    const at = line.indexOf(marker);
    if (at === -1) {
      userLines.push(line);
      continue;
    }
    // The player may have printed text without a trailing newline right before.
    if (at > 0) userLines.push(line.slice(0, at));
    const event = parseBody(line.slice(at + marker.length));
    switch (event.type) {
      case "start":
        result.started = true;
        break;
      case "end":
        result.ended = true;
        break;
      case "case":
        result.cases.push(event.outcome);
        break;
      case "unknown":
        break;
    }
  }
  // While streaming, the incomplete tail may be the start of a protocol line.
  const lastAt = last.indexOf(marker);
  const tail = lastAt === -1 ? last : last.slice(0, lastAt);
  result.userOutput = userLines.map((l) => `${l}\n`).join("") + tail;
  return result;
}

export type RunProgress =
  | { phase: "compiling" }
  | { phase: "running"; done: number; total: number };

/** Progress of a running submission, from the stdout received so far. */
export function runProgress(stdout: string, nonce: string, totalCases: number): RunProgress {
  const parsed = parseStdout(stdout, nonce);
  return parsed.started
    ? { phase: "running", done: parsed.cases.length, total: totalCases }
    : { phase: "compiling" };
}
