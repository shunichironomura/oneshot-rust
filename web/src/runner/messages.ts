import type { ToolchainFile } from "./manifest.ts";
import type { MiriInvocation, MiriResult } from "./miri.ts";

export type ToWorker =
  | { type: "init"; module: WebAssembly.Module; baseUrl: string; sysroot: ToolchainFile }
  | { type: "submit"; submissionId: string; invocation: MiriInvocation };

export type FromWorker =
  | { type: "ready"; initMs: number }
  | { type: "stdout"; submissionId: string; chunk: string }
  | { type: "stderr"; submissionId: string; chunk: string }
  | { type: "done"; submissionId: string; result: MiriResult; elapsedMs: number }
  | { type: "fatal"; message: string };
