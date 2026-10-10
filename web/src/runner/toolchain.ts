// Main-thread toolchain loading: download (once, into Cache Storage) and compile miri.wasm.
import { fetchCached, hasCacheStorage, TOOLCHAIN_CACHE } from "./fetch-cached.ts";
import type { ToolchainFile, ToolchainManifest } from "./manifest.ts";

export interface LoadProgress {
  phase: "download" | "compile";
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
  module: WebAssembly.Module;
  sysrootUrl: string;
}

async function downloadWithProgress(
  url: string,
  onChunk: (bytes: number) => void,
): Promise<Response> {
  const res = await fetch(url);
  if (!res.ok || res.body === null) throw new Error(`failed to download ${url}: ${res.status}`);
  const chunks: Uint8Array<ArrayBuffer>[] = [];
  const reader = res.body.getReader();
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    chunks.push(value);
    onChunk(value.byteLength);
  }
  return new Response(new Blob(chunks), { headers: res.headers });
}

async function ensureCached(
  files: { url: string; file: ToolchainFile }[],
  onProgress: (p: LoadProgress) => void,
): Promise<void> {
  if (!hasCacheStorage()) return; // Workers fall back to plain fetch (HTTP cache).
  const cache = await caches.open(TOOLCHAIN_CACHE);
  // Drop assets of other toolchain versions.
  const wanted = new Set(files.map((f) => f.url));
  for (const req of await cache.keys()) {
    if (!wanted.has(req.url)) await cache.delete(req);
  }
  const missing: typeof files = [];
  for (const f of files) {
    if ((await cache.match(f.url)) === undefined) missing.push(f);
  }
  const total = missing.reduce((n, f) => n + f.file.size, 0);
  let loaded = 0;
  for (const { url } of missing) {
    const res = await downloadWithProgress(url, (bytes) => {
      loaded += bytes;
      onProgress({ phase: "download", loaded, total });
    });
    await cache.put(url, res);
  }
}

async function compileWasm(url: string): Promise<WebAssembly.Module> {
  const res = await fetchCached(url);
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
  if (!manifestRes.ok) throw new ToolchainMissingError();
  const manifest = (await manifestRes.json()) as ToolchainManifest;
  const miriUrl = new URL(manifest.files.miriWasm.path, baseUrl).href;
  const sysrootUrl = new URL(manifest.files.sysroot.path, baseUrl).href;
  await ensureCached(
    [
      { url: miriUrl, file: manifest.files.miriWasm },
      { url: sysrootUrl, file: manifest.files.sysroot },
    ],
    onProgress,
  );
  onProgress({ phase: "compile", loaded: 0, total: 0 });
  const module = await compileWasm(miriUrl);
  return { manifest, module, sysrootUrl };
}
