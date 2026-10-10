// Downloads a toolchain asset: fetches its gzipped parts, joins and decompresses them.
import type { ToolchainFile } from "./manifest.ts";

async function fetchPart(url: string, onBytes: (n: number) => void): Promise<Blob> {
  const res = await fetch(url);
  if (!res.ok || res.body === null) throw new Error(`failed to download ${url}: ${res.status}`);
  const chunks: Uint8Array<ArrayBuffer>[] = [];
  const reader = res.body.getReader();
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    chunks.push(value);
    onBytes(value.byteLength);
  }
  return new Blob(chunks);
}

/**
 * Returns the decompressed asset. Parts are downloaded in parallel; `onBytes` is
 * called with the number of (compressed) bytes received so far for progress display.
 */
export async function downloadAsset(
  baseUrl: string,
  file: ToolchainFile,
  onBytes: (n: number) => void = () => {},
): Promise<Blob> {
  const parts = await Promise.all(
    file.parts.map((part) => fetchPart(new URL(part, baseUrl).href, onBytes)),
  );
  const stream = new Blob(parts).stream().pipeThrough(new DecompressionStream("gzip"));
  return new Response(stream).blob();
}
