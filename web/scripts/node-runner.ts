// Runs Miri submissions in Node with the same runner code as the browser.
import { randomBytes } from "node:crypto";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { classify, type Judgement, type RunOutput } from "../src/core/classify.ts";
import { buildSubmission } from "../src/core/harness.ts";
import type { JudgeSources, Problem } from "../src/core/problem.ts";
import { parseTar } from "../src/core/tar.ts";
import type { ToolchainManifest } from "../src/runner/manifest.ts";
import { buildSysrootTree, runMiri, type SysrootTree } from "../src/runner/miri.ts";

const TOOLCHAIN_DIR = path.resolve(import.meta.dirname, "../public/toolchain");

export interface Toolchain {
  module: WebAssembly.Module;
  sysroot: SysrootTree;
}

export async function loadToolchain(): Promise<Toolchain> {
  const manifest = JSON.parse(
    await readFile(path.join(TOOLCHAIN_DIR, "manifest.json"), "utf8"),
  ) as ToolchainManifest;
  const [wasm, tar] = await Promise.all([
    readFile(path.join(TOOLCHAIN_DIR, manifest.files.miriWasm.path)),
    readFile(path.join(TOOLCHAIN_DIR, manifest.files.sysroot.path)),
  ]);
  return {
    module: await WebAssembly.compile(wasm),
    sysroot: buildSysrootTree(parseTar(new Uint8Array(tar))),
  };
}

export async function judge(
  toolchain: Toolchain,
  problem: Problem,
  judgeSources: JudgeSources,
  code: string,
): Promise<{ judgement: Judgement; elapsedMs: number; output: RunOutput; nonce: string }> {
  const nonce = randomBytes(8).toString("hex");
  const submission = buildSubmission(problem, judgeSources, code, nonce);
  let stdout = "";
  let stderr = "";
  const start = performance.now();
  const result = await runMiri(toolchain.module, toolchain.sysroot, submission, {
    onStdout: (s) => (stdout += s),
    onStderr: (s) => (stderr += s),
  });
  const elapsedMs = performance.now() - start;
  const output: RunOutput = {
    stdout,
    stderr,
    exitCode: result.type === "exit" ? result.code : null,
    timedOut: false,
    outputLimitExceeded: result.type === "output-limit",
    crash: result.type === "crash" ? result.message : undefined,
  };
  const judgement = classify(output, {
    nonce,
    caseIds: problem.caseIds,
    stdoutLimitBytes: problem.judge.stdoutLimitBytes,
  });
  return { judgement, elapsedMs, output, nonce };
}
