// Fails if the build output exceeds the limits of Cloudflare Workers static assets
// (https://developers.cloudflare.com/workers/platform/limits/).
//
// Usage: node scripts/check-dist.ts [dist-dir]
import { readdir, stat } from "node:fs/promises";
import path from "node:path";

const MAX_FILE_BYTES = 25 * 1024 * 1024;
const MAX_FILES = 20_000;

const dist = path.resolve(process.argv[2] ?? path.join(import.meta.dirname, "../dist"));
const files = (await readdir(dist, { recursive: true, withFileTypes: true }))
  .filter((e) => e.isFile())
  .map((e) => path.join(e.parentPath, e.name));
const sizes = await Promise.all(files.map(async (f) => ({ f, size: (await stat(f)).size })));
const tooLarge = sizes.filter(({ size }) => size > MAX_FILE_BYTES);

for (const { f, size } of tooLarge) {
  console.error(`too large (${(size / 1024 / 1024).toFixed(1)} MiB): ${path.relative(dist, f)}`);
}
if (files.length > MAX_FILES) console.error(`too many files: ${files.length} > ${MAX_FILES}`);
const total = sizes.reduce((n, { size }) => n + size, 0);
console.log(`${files.length} files, ${(total / 1024 / 1024).toFixed(1)} MiB in ${dist}`);
process.exitCode = tooLarge.length > 0 || files.length > MAX_FILES ? 1 : 0;
