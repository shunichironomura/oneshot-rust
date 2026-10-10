import type { ToolchainState } from "./useToolchain.ts";

function megabytes(bytes: number): string {
  return (bytes / 1024 / 1024).toFixed(1);
}

/** Banner shown until the compiler (miri.wasm) is ready. Problems can be browsed meanwhile. */
export function ToolchainStatus({ state }: { state: ToolchainState }) {
  switch (state.status) {
    case "ready":
      return null;
    case "error":
      return (
        <div class="toolchain-status error">コンパイラを読み込めませんでした: {state.message}</div>
      );
    case "warming":
      return <div class="toolchain-status">コンパイラを起動中…</div>;
    case "loading": {
      const p = state.progress;
      if (p === null) return <div class="toolchain-status">コンパイラを準備中…</div>;
      if (p.phase === "compile") return <div class="toolchain-status">コンパイラを初期化中…</div>;
      const ratio = p.total === 0 ? 0 : p.loaded / p.total;
      return (
        <div class="toolchain-status">
          <span>
            コンパイラをダウンロード中（初回のみ）… {megabytes(p.loaded)} / {megabytes(p.total)} MB
          </span>
          <progress value={ratio} max={1} />
        </div>
      );
    }
  }
}
