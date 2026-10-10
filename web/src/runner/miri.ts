// Runs one Miri invocation on top of browser_wasi_shim. Works in Web Workers and in Node.
import {
  ConsoleStdout,
  Directory,
  File,
  type Inode,
  OpenFile,
  PreopenDirectory,
  WASI,
  WASIProcExit,
} from "@bjorn3/browser_wasi_shim";
import { SRC_DIR, SYSROOT_DIR, type SubmissionFiles, TMP_DIR } from "../core/harness.ts";
import type { TarEntry } from "../core/tar.ts";

/** The read-only sysroot as an in-memory directory tree, shared by all invocations. */
export type SysrootTree = Map<string, Inode>;

export function buildSysrootTree(entries: TarEntry[]): SysrootTree {
  const root: SysrootTree = new Map();
  for (const { path, data } of entries) {
    const parts = path.split("/").filter((p) => p !== "");
    const fileName = parts.pop();
    if (fileName === undefined) continue;
    let dir = root;
    for (const part of parts) {
      let next = dir.get(part);
      if (next === undefined) {
        next = new Directory(new Map());
        dir.set(part, next);
      }
      if (!(next instanceof Directory)) throw new Error(`sysroot: ${part} is not a directory`);
      dir = next.contents;
    }
    dir.set(fileName, new File(data, { readonly: true }));
  }
  return root;
}

export interface MiriInvocation {
  files: SubmissionFiles;
  args: string[];
  stdoutLimitBytes: number;
}

export interface MiriCallbacks {
  onStdout: (chunk: string) => void;
  onStderr: (chunk: string) => void;
}

export type MiriResult =
  | { type: "exit"; code: number }
  | { type: "output-limit" }
  | { type: "crash"; message: string };

class OutputLimitExceeded extends Error {}

const encoder = new TextEncoder();

function sourceDir(files: SubmissionFiles): Map<string, Inode> {
  return new Map(
    Object.entries(files).map(([name, text]): [string, Inode] => [
      name,
      new File(encoder.encode(text)),
    ]),
  );
}

function asWasiCommand(instance: WebAssembly.Instance) {
  const { memory, _start } = instance.exports;
  if (!(memory instanceof WebAssembly.Memory) || typeof _start !== "function") {
    throw new Error("miri.wasm is not a WASI command module");
  }
  return { exports: { memory, _start: _start as () => unknown } };
}

export async function runMiri(
  module: WebAssembly.Module,
  sysroot: SysrootTree,
  invocation: MiriInvocation,
  callbacks: MiriCallbacks,
): Promise<MiriResult> {
  const stdoutDecoder = new TextDecoder();
  const stderrDecoder = new TextDecoder();
  let stdoutBytes = 0;

  const stdout = new ConsoleStdout((buf) => {
    stdoutBytes += buf.byteLength;
    if (stdoutBytes > invocation.stdoutLimitBytes) throw new OutputLimitExceeded();
    callbacks.onStdout(stdoutDecoder.decode(buf, { stream: true }));
  });
  const stderr = new ConsoleStdout((buf) => {
    callbacks.onStderr(stderrDecoder.decode(buf, { stream: true }));
  });
  const fds = [
    new OpenFile(new File(new Uint8Array(), { readonly: true })),
    stdout,
    stderr,
    new PreopenDirectory(SYSROOT_DIR, sysroot),
    new PreopenDirectory(SRC_DIR, sourceDir(invocation.files)),
    new PreopenDirectory(TMP_DIR, new Map()),
  ];
  // RUSTC_ICE=0: never try to write ICE reports to the (virtual) disk.
  const wasi = new WASI(["miri", ...invocation.args], ["RUSTC_ICE=0"], fds, { debug: false });
  const instance = await WebAssembly.instantiate(module, {
    wasi_snapshot_preview1: wasi.wasiImport,
  });

  try {
    const code = wasi.start(asWasiCommand(instance));
    return { type: "exit", code };
  } catch (e) {
    if (e instanceof OutputLimitExceeded) return { type: "output-limit" };
    if (e instanceof WASIProcExit) return { type: "exit", code: e.code };
    return { type: "crash", message: e instanceof Error ? `${e.name}: ${e.message}` : String(e) };
  } finally {
    const restOut = stdoutDecoder.decode();
    if (restOut !== "") callbacks.onStdout(restOut);
    const restErr = stderrDecoder.decode();
    if (restErr !== "") callbacks.onStderr(restErr);
  }
}
