// Downloads the prebuilt toolchain pinned in toolchain/RELEASE (published by
// toolchain/publish.nu) into public/toolchain/. Used by CI, by the Cloudflare
// build, and locally instead of building the toolchain (20-30 minutes).
//
// Assets come from the release download URLs (github.com, not the REST API), so
// no token is needed and the API's rate limit for anonymous requests doesn't apply.
//
// Usage: node scripts/fetch-toolchain.ts
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import type { ToolchainManifest } from "../src/runner/manifest.ts";
import { assetUrl, placements, toolchainIdOfTag } from "./toolchain-release.ts";

const REPOSITORY = process.env["TOOLCHAIN_REPOSITORY"] ?? "shunichironomura/oneshot-rust";
// Overridable for testing against a local server.
const DOWNLOAD_URL =
  process.env["TOOLCHAIN_DOWNLOAD_URL"] ?? `https://github.com/${REPOSITORY}/releases/download`;
const RELEASE_FILE = path.resolve(import.meta.dirname, "../../toolchain/RELEASE");
const TOOLCHAIN_DIR = path.resolve(import.meta.dirname, "../public/toolchain");

async function installedId(): Promise<string | undefined> {
  try {
    const manifest = JSON.parse(
      await readFile(path.join(TOOLCHAIN_DIR, "manifest.json"), "utf8"),
    ) as ToolchainManifest;
    await Promise.all(placements(manifest).map((p) => readFile(path.join(TOOLCHAIN_DIR, p.path))));
    return manifest.toolchainId;
  } catch {
    return undefined;
  }
}

async function main() {
  const tag = (await readFile(RELEASE_FILE, "utf8")).trim();
  const id = toolchainIdOfTag(tag);
  if ((await installedId()) === id) {
    console.log(`Toolchain ${id} is already installed`);
    return;
  }

  const download = async (name: string) => {
    const url = assetUrl(DOWNLOAD_URL, tag, name);
    const res = await fetch(url);
    if (!res.ok) throw new Error(`GET ${url}: ${res.status} ${res.statusText}`);
    return new Uint8Array(await res.arrayBuffer());
  };

  const manifestBytes = await download("manifest.json");
  const manifest = JSON.parse(new TextDecoder().decode(manifestBytes)) as ToolchainManifest;
  if (manifest.toolchainId !== id) {
    throw new Error(`release ${tag} contains toolchain ${manifest.toolchainId}`);
  }
  await rm(TOOLCHAIN_DIR, { recursive: true, force: true });
  await Promise.all(
    placements(manifest).map(async ({ asset, path: dest }) => {
      const bytes = asset === "manifest.json" ? manifestBytes : await download(asset);
      await mkdir(path.dirname(path.join(TOOLCHAIN_DIR, dest)), { recursive: true });
      await writeFile(path.join(TOOLCHAIN_DIR, dest), bytes);
    }),
  );
  console.log(`Installed toolchain ${id} into ${TOOLCHAIN_DIR}`);
}

await main();
