// Runner Worker: holds the sysroot and runs one Miri invocation per submission.
import { parseTar } from "../core/tar.ts";
import { fetchCached } from "./fetch-cached.ts";
import type { FromWorker, ToWorker } from "./messages.ts";
import { buildSysrootTree, runMiri, type SysrootTree } from "./miri.ts";

let toolchain: { module: WebAssembly.Module; sysroot: SysrootTree } | undefined;

function post(message: FromWorker) {
  self.postMessage(message);
}

async function init(module: WebAssembly.Module, sysrootUrl: string) {
  const start = performance.now();
  const tar = new Uint8Array(await (await fetchCached(sysrootUrl)).arrayBuffer());
  toolchain = { module, sysroot: buildSysrootTree(parseTar(tar)) };
  post({ type: "ready", initMs: performance.now() - start });
}

async function submit(message: Extract<ToWorker, { type: "submit" }>) {
  if (toolchain === undefined) throw new Error("submit before init");
  const { submissionId } = message;
  const start = performance.now();
  const result = await runMiri(toolchain.module, toolchain.sysroot, message.invocation, {
    onStdout: (chunk) => post({ type: "stdout", submissionId, chunk }),
    onStderr: (chunk) => post({ type: "stderr", submissionId, chunk }),
  });
  post({ type: "done", submissionId, result, elapsedMs: performance.now() - start });
}

self.onmessage = (event: MessageEvent<ToWorker>) => {
  const message = event.data;
  const task = message.type === "init" ? init(message.module, message.sysrootUrl) : submit(message);
  task.catch((e: unknown) =>
    post({ type: "fatal", message: e instanceof Error ? e.message : String(e) }),
  );
};
