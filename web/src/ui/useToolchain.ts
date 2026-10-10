import { useEffect, useState } from "preact/hooks";
import { RunnerPool } from "../runner/pool.ts";
import { type LoadProgress, loadToolchain } from "../runner/toolchain.ts";

export type ToolchainState =
  | { status: "loading"; progress: LoadProgress | null }
  | { status: "warming"; pool: RunnerPool }
  | { status: "ready"; pool: RunnerPool; toolchainId: string }
  | { status: "error"; message: string };

/** Downloads (or loads from cache) the toolchain and starts the runner Workers. */
export function useToolchain(): ToolchainState {
  const [state, setState] = useState<ToolchainState>({ status: "loading", progress: null });

  useEffect(() => {
    let disposed = false;
    let pool: RunnerPool | undefined;
    const baseUrl = new URL(`${import.meta.env.BASE_URL}toolchain/`, window.location.href).href;
    loadToolchain(baseUrl, (progress) => {
      if (!disposed) setState({ status: "loading", progress });
    })
      .then(async (toolchain) => {
        if (disposed) return;
        pool = new RunnerPool(toolchain);
        setState({ status: "warming", pool });
        await pool.ready();
        if (!disposed)
          setState({ status: "ready", pool, toolchainId: toolchain.manifest.toolchainId });
      })
      .catch((e: unknown) => {
        if (!disposed)
          setState({ status: "error", message: e instanceof Error ? e.message : String(e) });
      });
    return () => {
      disposed = true;
      pool?.dispose();
    };
  }, []);

  return state;
}
