import preact from "@preact/preset-vite";
import { defineConfig, lazyPlugins } from "vite-plus";
import { oneshotContent } from "./content/plugin.ts";
import { thirdPartyLicenses } from "./scripts/third-party-licenses.ts";

// https://vite.dev/config/
export default defineConfig({
  fmt: {
    // Fixtures are recorded verbatim by scripts/record-fixtures.ts.
    ignorePatterns: ["src/core/__fixtures__/**", "public/toolchain/**"],
  },
  lint: {
    ignorePatterns: ["public/toolchain/**"],
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: true },
  },
  plugins: lazyPlugins(() => [preact(), oneshotContent(), thirdPartyLicenses()]),
  // The worker bundles' packages are listed in the main build's third-party-licenses.txt.
  worker: { format: "es", plugins: () => [thirdPartyLicenses()] },
});
