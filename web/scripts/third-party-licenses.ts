import { readdir, readFile } from "node:fs/promises";
import path from "node:path";
import type { Plugin } from "vite-plus";

export const LICENSES_FILE = "third-party-licenses.txt";

const LICENSE_FILE = /^(licen[cs]e|copying|notice|ofl)/i;

/** The packages whose modules are in the bundle, shared by the main and worker builds. */
const packageDirs = new Set<string>();

/** The directory of the npm package that a module id belongs to, if any. */
function packageDir(id: string): string | undefined {
  const file = id.split("?")[0].replaceAll("\\", "/");
  const at = file.lastIndexOf("/node_modules/");
  if (id.startsWith("\0") || at < 0) return undefined;
  const rest = file.slice(at + "/node_modules/".length).split("/");
  const name = rest[0].startsWith("@") ? rest.slice(0, 2) : rest.slice(0, 1);
  return [file.slice(0, at), "node_modules", ...name].join("/");
}

async function notice(dir: string): Promise<string> {
  const pkg = JSON.parse(await readFile(path.join(dir, "package.json"), "utf8")) as {
    name: string;
    version: string;
    license?: string;
  };
  const files = (await readdir(dir)).filter((f) => LICENSE_FILE.test(f)).sort();
  if (files.length === 0) throw new Error(`${pkg.name} has no license file in ${dir}`);
  const texts = await Promise.all(files.map((f) => readFile(path.join(dir, f), "utf8")));
  const header = `${pkg.name}@${pkg.version}\nLicense: ${pkg.license ?? "see below"}`;
  return [header, ...texts.map((t) => t.trim())].join("\n\n");
}

/**
 * Collects the npm packages bundled into the app and its workers, and emits their names,
 * versions, licenses and license texts as `third-party-licenses.txt`. Add it to both
 * `plugins` and `worker.plugins`: worker bundles are built before the main bundle is written.
 */
export function thirdPartyLicenses(): Plugin {
  return {
    name: "third-party-licenses",
    apply: "build",
    async generateBundle() {
      for (const id of this.getModuleIds()) {
        const dir = packageDir(id);
        if (dir) packageDirs.add(dir);
      }
      // Only the main build, whose input is index.html, writes the file.
      if (![...this.getModuleIds()].some((id) => id.endsWith(".html"))) return;
      const notices = await Promise.all([...packageDirs].map(notice));
      notices.sort((a, b) => a.localeCompare(b));
      const separator = `\n\n${"=".repeat(80)}\n\n`;
      this.emitFile({
        type: "asset",
        fileName: LICENSES_FILE,
        source:
          "This site bundles the following third-party packages.\n" +
          "The licenses of the code, the problems and the toolchain are listed in\n" +
          "https://github.com/shunichironomura/oneshot-rust#readme." +
          separator +
          notices.join(separator) +
          "\n",
      });
    },
  };
}
