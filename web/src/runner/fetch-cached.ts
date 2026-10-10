import { downloadAsset } from "./assets.ts";
import type { ToolchainFile } from "./manifest.ts";

/** Cache Storage bucket for toolchain assets. Their keys contain the toolchain id. */
export const TOOLCHAIN_CACHE = "oneshot-toolchain";

/** Whether Cache Storage is usable (it is not in insecure contexts). */
export function hasCacheStorage(): boolean {
  return typeof caches !== "undefined";
}

/** The Cache Storage key of a decompressed asset. */
export function assetUrl(baseUrl: string, file: ToolchainFile): string {
  return new URL(file.path, baseUrl).href;
}

/** Returns the asset from Cache Storage, downloading it if it is not cached. */
export async function fetchCached(baseUrl: string, file: ToolchainFile): Promise<Response> {
  if (hasCacheStorage()) {
    const cached = await (await caches.open(TOOLCHAIN_CACHE)).match(assetUrl(baseUrl, file));
    if (cached !== undefined) return cached;
  }
  return new Response(await downloadAsset(baseUrl, file));
}
