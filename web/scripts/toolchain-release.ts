// Pure helpers for installing a toolchain GitHub Release (see toolchain/publish.nu).
import type { ToolchainManifest } from "../src/runner/manifest.ts";

/** `toolchain/RELEASE` holds a tag of the form `toolchain-<toolchainId>`. */
export function toolchainIdOfTag(tag: string): string {
  const m = /^toolchain-([0-9a-f]+)$/.exec(tag.trim());
  if (m === null) throw new Error(`invalid toolchain release tag: "${tag.trim()}"`);
  return m[1];
}

/** Release assets are flat; the manifest says where each part goes under the toolchain dir. */
export function placements(manifest: ToolchainManifest): { asset: string; path: string }[] {
  const parts = Object.values(manifest.files).flatMap((f) => f.parts);
  return [
    ...[...parts, ...(manifest.licenses ?? [])].map((path) => ({
      asset: path.split("/").at(-1) ?? path,
      path,
    })),
    { asset: "manifest.json", path: "manifest.json" },
  ];
}
