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

/**
 * One asset, served gzipped and split into parts (Cloudflare Pages limits files
 * to 25 MiB). Paths are relative to the toolchain directory.
 */
export interface ToolchainFile {
  /** Logical path of the decompressed file; used as its Cache Storage key, not served. */
  path: string;
  /** Decompressed size in bytes. */
  size: number;
  /** Total size of the parts in bytes. */
  compressedSize: number;
  /** Parts of the gzipped file, in order. */
  parts: string[];
}
