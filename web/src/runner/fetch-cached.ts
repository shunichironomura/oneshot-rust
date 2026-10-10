/** Cache Storage bucket for toolchain assets. Their URLs contain the toolchain id. */
export const TOOLCHAIN_CACHE = "oneshot-toolchain";

/** Whether Cache Storage is usable (it is not in insecure contexts). */
export function hasCacheStorage(): boolean {
  return typeof caches !== "undefined";
}

/** Returns the cached response for `url`, falling back to the network. */
export async function fetchCached(url: string): Promise<Response> {
  if (hasCacheStorage()) {
    const cached = await (await caches.open(TOOLCHAIN_CACHE)).match(url);
    if (cached !== undefined) return cached;
  }
  const res = await fetch(url);
  if (!res.ok) throw new Error(`failed to fetch ${url}: ${res.status}`);
  return res;
}
