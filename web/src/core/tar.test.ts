import { expect, test } from "vite-plus/test";
import { parseTar } from "./tar.ts";

/** Builds a minimal ustar archive. */
function tar(files: { name: string; data: string; type?: string; prefix?: string }[]): Uint8Array {
  const enc = new TextEncoder();
  const blocks: Uint8Array[] = [];
  for (const f of files) {
    const header = new Uint8Array(512);
    const data = enc.encode(f.data);
    header.set(enc.encode(f.name), 0);
    header.set(enc.encode(`${data.length.toString(8).padStart(11, "0")}\0`), 124);
    header.set(enc.encode(f.type ?? "0"), 156);
    header.set(enc.encode("ustar\0"), 257);
    if (f.prefix) header.set(enc.encode(f.prefix), 345);
    blocks.push(header);
    const body = new Uint8Array(Math.ceil(data.length / 512) * 512);
    body.set(data);
    blocks.push(body);
  }
  blocks.push(new Uint8Array(1024));
  const out = new Uint8Array(blocks.reduce((n, b) => n + b.length, 0));
  let offset = 0;
  for (const b of blocks) {
    out.set(b, offset);
    offset += b.length;
  }
  return out;
}

test("parseTar reads regular files and skips directories", () => {
  const archive = tar([
    { name: "lib/", data: "", type: "5" },
    { name: "lib/a.rlib", data: "hello" },
    { name: "b.rlib", data: "x".repeat(600), prefix: "deep/dir" },
  ]);
  const entries = parseTar(archive).map((e) => ({
    path: e.path,
    text: new TextDecoder().decode(e.data),
  }));
  expect(entries).toEqual([
    { path: "lib/a.rlib", text: "hello" },
    { path: "deep/dir/b.rlib", text: "x".repeat(600) },
  ]);
});

test("parseTar rejects truncated archives", () => {
  const archive = tar([{ name: "a", data: "x".repeat(1000) }]);
  expect(() => parseTar(archive.subarray(0, 700))).toThrow(/truncated/);
});
