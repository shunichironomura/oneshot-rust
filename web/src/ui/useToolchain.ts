import { useEffect, useState } from "preact/hooks";
import { RunnerPool } from "../runner/pool.ts";
import { type LoadProgress, loadToolchain, ToolchainMissingError } from "../runner/toolchain.ts";

export type ToolchainState =
  | { status: "loading"; progress: LoadProgress | null }
  | { status: "warming" }
  | { status: "ready"; toolchainId: string }
  | { status: "missing" }
  | { status: "error"; message: string };

export interface Toolchain {
  state: ToolchainState;
  /**
   * Resolves once the runner pool exists, so that a submission made while the
   * compiler is still loading simply waits for it. Rejects if loading fails.
   */
  pool: Promise<RunnerPool>;
}

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (reason: unknown) => void;
  const promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });
  // Failures are reported through `state`; awaiting callers handle them themselves.
  promise.catch(() => {});
  return { promise, resolve, reject };
}

/** Downloads (or loads from cache) the toolchain and starts the runner Workers. */
export function useToolchain(): Toolchain {
  const [state, setState] = useState<ToolchainState>({ status: "loading", progress: null });
  const [pool] = useState(() => deferred<RunnerPool>());

  useEffect(() => {
    let disposed = false;
    let created: RunnerPool | undefined;
    const baseUrl = new URL(`${import.meta.env.BASE_URL}toolchain/`, window.location.href).href;
    loadToolchain(baseUrl, (progress) => {
      if (!disposed) setState({ status: "loading", progress });
    })
      .then(async (toolchain) => {
        if (disposed) return;
        created = new RunnerPool(toolchain);
        pool.resolve(created);
        setState({ status: "warming" });
        await created.ready();
        if (!disposed) setState({ status: "ready", toolchainId: toolchain.manifest.toolchainId });
      })
      .catch((e: unknown) => {
        pool.reject(e);
        if (disposed) return;
        if (e instanceof ToolchainMissingError) setState({ status: "missing" });
        else setState({ status: "error", message: e instanceof Error ? e.message : String(e) });
      });
    return () => {
      disposed = true;
      created?.dispose();
    };
  }, [pool]);

  return { state, pool: pool.promise };
}
