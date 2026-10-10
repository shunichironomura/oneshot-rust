import { useApp } from "./context.ts";
import type { ToolchainState } from "./useToolchain.ts";

function megabytes(bytes: number): string {
  return (bytes / 1024 / 1024).toFixed(0);
}

/**
 * Slim banner for the first-visit download and for errors. Short phases (loading from
 * the cache, starting Workers) are only reflected in the Submit button.
 */
export function ToolchainStatus({ state }: { state: ToolchainState }) {
  const { t } = useApp();
  switch (state.status) {
    case "ready":
      return null;
    case "missing":
      return <div class="toolchain-status error">{t.toolchainMissing}</div>;
    case "error":
      return <div class="toolchain-status error">{t.toolchainError(state.message)}</div>;
    case "warming":
      return null;
    case "loading": {
      const p = state.progress;
      if (p === null || p.phase === "compile") return null;
      return (
        <div class="toolchain-status">
          <span>{t.toolchainDownloading(megabytes(p.loaded), megabytes(p.total))}</span>
          <progress value={p.total === 0 ? 0 : p.loaded / p.total} max={1} />
        </div>
      );
    }
  }
}
