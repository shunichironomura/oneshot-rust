// Main-thread toolchain loading: download (once, into Cache Storage) and compile miri.wasm.
import { downloadAsset } from "./assets.ts";
import { assetUrl, fetchCached, hasCacheStorage, TOOLCHAIN_CACHE } from "./fetch-cached.ts";
import type { ToolchainFile, ToolchainManifest } from "./manifest.ts";

export interface LoadProgress {
  phase: "download" | "compile";
  /** Compressed bytes downloaded so far. */
  loaded: number;
  total: number;
}

/** `manifest.json` is missing: the toolchain has not been built yet. */
export class ToolchainMissingError extends Error {
  constructor() {
    super("toolchain manifest not found");
    this.name = "ToolchainMissingError";
  }
}

export interface LoadedToolchain {
  manifest: ToolchainManifest;
  /** Base URL that manifest paths are relative to. */
  baseUrl: string;
  module: WebAssembly.Module;
}

const CONTENT_TYPES: Record<keyof ToolchainManifest["files"], string> = {
  miriWasm: "application/wasm",
  sysroot: "application/x-tar",
};

/** Downloads missing assets into Cache Storage and drops other toolchain versions. */
async function ensureCached(
  baseUrl: string,
  manifest: ToolchainManifest,
  onProgress: (p: LoadProgress) => void,
): Promise<void> {
  if (!hasCacheStorage()) return; // Workers then download what they need themselves.
  const cache = await caches.open(TOOLCHAIN_CACHE);
  const entries = Object.entries(manifest.files) as [keyof typeof CONTENT_TYPES, ToolchainFile][];
  const wanted = new Set(entries.map(([, file]) => assetUrl(baseUrl, file)));
  for (const req of await cache.keys()) {
    if (!wanted.has(req.url)) await cache.delete(req);
  }
  const missing: typeof entries = [];
  for (const entry of entries) {
    if ((await cache.match(assetUrl(baseUrl, entry[1]))) === undefined) missing.push(entry);
  }
  const total = missing.reduce((n, [, file]) => n + file.compressedSize, 0);
  let loaded = 0;
  await Promise.all(
    missing.map(async ([key, file]) => {
      const blob = await downloadAsset(baseUrl, file, (bytes) => {
        loaded += bytes;
        onProgress({ phase: "download", loaded, total });
      });
      const headers = { "Content-Type": CONTENT_TYPES[key] };
      await cache.put(assetUrl(baseUrl, file), new Response(blob, { headers }));
    }),
  );
}

async function compileWasm(baseUrl: string, file: ToolchainFile): Promise<WebAssembly.Module> {
  const res = await fetchCached(baseUrl, file);
  try {
    return await WebAssembly.compileStreaming(res.clone());
  } catch {
    // compileStreaming requires `Content-Type: application/wasm`.
    return await WebAssembly.compile(await res.arrayBuffer());
  }
}

export async function loadToolchain(
  baseUrl: string,
  onProgress: (p: LoadProgress) => void,
): Promise<LoadedToolchain> {
  const manifestRes = await fetch(new URL("manifest.json", baseUrl), { cache: "no-cache" });
  // Static hosts may answer a missing file with the SPA's index.html.
  const isJson = manifestRes.headers.get("Content-Type")?.includes("json") ?? false;
  if (!manifestRes.ok || !isJson) throw new ToolchainMissingError();
  const manifest = (await manifestRes.json()) as ToolchainManifest;
  await ensureCached(baseUrl, manifest, onProgress);
  onProgress({ phase: "compile", loaded: 0, total: 0 });
  const module = await compileWasm(baseUrl, manifest.files.miriWasm);
  return { manifest, baseUrl, module };
}
