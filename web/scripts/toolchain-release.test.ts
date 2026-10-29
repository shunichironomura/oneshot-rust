import { expect, test } from "vite-plus/test";
import type { ToolchainManifest } from "../src/runner/manifest.ts";
import { assetUrl, placements, toolchainIdOfTag } from "./toolchain-release.ts";

test("toolchainIdOfTag", () => {
  expect(toolchainIdOfTag("toolchain-2c66f92f336b\n")).toBe("2c66f92f336b");
  expect(() => toolchainIdOfTag("v1.0")).toThrow(/invalid toolchain release tag/);
});

test("assetUrl", () => {
  const base = "https://github.com/o/r/releases/download";
  expect(assetUrl(base, "toolchain-abc", "miri.wasm.gz.0")).toBe(
    "https://github.com/o/r/releases/download/toolchain-abc/miri.wasm.gz.0",
  );
  expect(assetUrl(`${base}/`, "toolchain-abc", "a b")).toBe(
    "https://github.com/o/r/releases/download/toolchain-abc/a%20b",
  );
});

test("placements maps flat release assets into the toolchain directory", () => {
  const file = (name: string, parts: number) => ({
    path: `abc/${name}`,
    size: 1,
    compressedSize: 1,
    parts: Array.from({ length: parts }, (_, i) => `abc/${name}.gz.${i}`),
  });
  const manifest: ToolchainManifest = {
    toolchainId: "abc",
    rustCommit: "x",
    target: "t",
    files: { miriWasm: file("miri.wasm", 2), sysroot: file("sysroot.tar", 1) },
  };
  expect(placements(manifest)).toEqual([
    { asset: "miri.wasm.gz.0", path: "abc/miri.wasm.gz.0" },
    { asset: "miri.wasm.gz.1", path: "abc/miri.wasm.gz.1" },
    { asset: "sysroot.tar.gz.0", path: "abc/sysroot.tar.gz.0" },
    { asset: "manifest.json", path: "manifest.json" },
  ]);
});

test("placements includes the licenses when the manifest lists them", () => {
  const file = (name: string) => ({
    path: `abc/${name}`,
    size: 1,
    compressedSize: 1,
    parts: [`abc/${name}.gz.0`],
  });
  const manifest: ToolchainManifest = {
    toolchainId: "abc",
    rustCommit: "x",
    target: "t",
    files: { miriWasm: file("miri.wasm"), sysroot: file("sysroot.tar") },
    licenses: ["abc/licenses/README.txt", "abc/licenses/rust-COPYRIGHT.html"],
  };
  expect(placements(manifest)).toEqual([
    { asset: "miri.wasm.gz.0", path: "abc/miri.wasm.gz.0" },
    { asset: "sysroot.tar.gz.0", path: "abc/sysroot.tar.gz.0" },
    { asset: "README.txt", path: "abc/licenses/README.txt" },
    { asset: "rust-COPYRIGHT.html", path: "abc/licenses/rust-COPYRIGHT.html" },
    { asset: "manifest.json", path: "manifest.json" },
  ]);
});
