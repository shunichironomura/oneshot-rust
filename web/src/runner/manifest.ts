/** `public/toolchain/manifest.json`, written by `toolchain/build.nu`. */
export interface ToolchainManifest {
  toolchainId: string;
  rustCommit: string;
  target: string;
  files: {
    miriWasm: ToolchainFile;
    sysroot: ToolchainFile;
  };
}

export interface ToolchainFile {
  /** Path relative to the toolchain directory. */
  path: string;
  /** Uncompressed size in bytes. */
  size: number;
}
