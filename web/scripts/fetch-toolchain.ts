// Downloads the prebuilt toolchain pinned in toolchain/RELEASE (published by
// toolchain/publish.nu) into public/toolchain/. Used by CI, by the Cloudflare
// build, and locally instead of building the toolchain (20-30 minutes).
//
// The repository is private, so a GitHub token with read access to its contents
// is needed: GITHUB_TOKEN or GH_TOKEN, or else the GitHub CLI's login.
//
// Usage: node scripts/fetch-toolchain.ts
import { execFile } from "node:child_process";
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import { promisify } from "node:util";
import type { ToolchainManifest } from "../src/runner/manifest.ts";
import { placements, toolchainIdOfTag } from "./toolchain-release.ts";

const REPOSITORY = process.env["TOOLCHAIN_REPOSITORY"] ?? "shunichironomura/oneshot-rust";
// Also set by GitHub Actions; overridable for GitHub Enterprise or testing.
const API_URL = process.env["GITHUB_API_URL"] ?? "https://api.github.com";
const RELEASE_FILE = path.resolve(import.meta.dirname, "../../toolchain/RELEASE");
const TOOLCHAIN_DIR = path.resolve(import.meta.dirname, "../public/toolchain");

interface ReleaseAsset {
  name: string;
  /** API URL; returns the binary with `Accept: application/octet-stream`. */
  url: string;
}

async function githubToken(): Promise<string | undefined> {
  const fromEnv = process.env["GITHUB_TOKEN"] ?? process.env["GH_TOKEN"];
  if (fromEnv !== undefined && fromEnv !== "") return fromEnv;
  try {
    const { stdout } = await promisify(execFile)("gh", ["auth", "token"]);
    return stdout.trim();
  } catch {
    return undefined; // Fine for a public repository.
  }
}

async function github(url: string, token: string | undefined, accept: string): Promise<Response> {
  const headers: Record<string, string> = { Accept: accept, "X-GitHub-Api-Version": "2022-11-28" };
  if (token !== undefined) headers["Authorization"] = `Bearer ${token}`;
  const res = await fetch(url, { headers });
  if (!res.ok) {
    // GitHub answers 404 for private repositories without (or with an insufficient) token.
    const hint =
      res.status === 404 && token === undefined
        ? "\nNo GitHub token found: set GITHUB_TOKEN (read access to the repository's contents)."
        : "";
    throw new Error(`GET ${url}: ${res.status} ${await res.text()}${hint}`);
  }
  return res;
}

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

  const token = await githubToken();
  if (token === undefined) {
    // Names only (never values), to diagnose misconfigured CI/build environments.
    const names = Object.keys(process.env).filter((n) => /TOKEN|GITHUB/i.test(n));
    console.warn(
      `No GitHub token found. Environment variables named like TOKEN/GITHUB: ${names.join(", ") || "none"}`,
    );
  }
  const release = (await (
    await github(
      `${API_URL}/repos/${REPOSITORY}/releases/tags/${tag}`,
      token,
      "application/vnd.github+json",
    )
  ).json()) as { assets: ReleaseAsset[] };
  const download = async (name: string) => {
    const asset = release.assets.find((a) => a.name === name);
    if (asset === undefined) throw new Error(`release ${tag} has no asset ${name}`);
    return new Uint8Array(
      await (await github(asset.url, token, "application/octet-stream")).arrayBuffer(),
    );
  };

  const manifestBytes = await download("manifest.json");
  const manifest = JSON.parse(new TextDecoder().decode(manifestBytes)) as ToolchainManifest;
  if (manifest.toolchainId !== id) {
    throw new Error(`release ${tag} contains toolchain ${manifest.toolchainId}`);
  }
  await rm(TOOLCHAIN_DIR, { recursive: true, force: true });
  await mkdir(path.join(TOOLCHAIN_DIR, id), { recursive: true });
  await Promise.all(
    placements(manifest).map(async ({ asset, path: dest }) => {
      const bytes = asset === "manifest.json" ? manifestBytes : await download(asset);
      await writeFile(path.join(TOOLCHAIN_DIR, dest), bytes);
    }),
  );
  console.log(`Installed toolchain ${id} into ${TOOLCHAIN_DIR}`);
}

await main();
