# OneShot Rust

English | [日本語](README.ja.md)

Write Rust that compiles in one shot — no LSP, as few compiles as possible.

Play at <https://oneshotrust.dev>.

OneShot Rust is a browser game where you implement a given task in Rust and compete to **pass all the tests with as
few submissions (= compiles) as possible**. There is no completion and no LSP. Judging is done in the browser by
[Miri](https://github.com/rust-lang/miri) compiled to Wasm, so type checking and borrow checking are the same as the
real rustc, and undefined behavior (UB) is detected too.
The UI and problem statements default to English and can be switched to Japanese.

## Layout

```text
toolchain/   Scripts and patches that build Miri for wasm32-wasip1
judge/       Judge library (judge.rs) and fixed template (main.rs)
problems/    Problems (one directory per problem)
web/         Frontend (Vite+ / Preact / CodeMirror 6)
```

## Playing locally

Requirements: [Nushell](https://www.nushell.sh/), Python 3, git, Node.js 24 or later,
[Vite+](https://viteplus.dev/) (`vp`).

1. Place the toolchain (`miri.wasm` and the sysroot) in `web/public/toolchain/`. Either download the published one
   or build it yourself (the first build takes 20–30 minutes). See [toolchain/README.md](toolchain/README.md)
   for details.

   ```sh
   cd web && vp run fetch-toolchain   # download the version in toolchain/RELEASE
   # or
   ./toolchain/build.nu               # build from source
   ```

2. Start the dev server.

   ```sh
   cd web
   vp install
   vp dev
   ```

   On first access, the app downloads the toolchain (about 55 MB compressed) and stores it in Cache Storage.
   Later visits load it from the cache.

## Deployment (Cloudflare Workers Builds)

Cloudflare's Git integration (Workers Builds) detects pushes to the repository and builds and deploys them.
`main` is deployed to production; other branches and pull requests are deployed to preview URLs (Cloudflare posts
the URL as a comment on the PR). No Cloudflare credentials are stored on the GitHub side. The site is served as
Workers static assets, configured in `web/wrangler.jsonc`.

GitHub Actions (`.github/workflows/ci.yml`) only runs checks, tests, verification of all problems, and the build.

Neither builds the toolchain (it takes 20–30 minutes and about 30 GB of disk). Instead, a locally built toolchain is
published as a GitHub Release, and the version recorded in `toolchain/RELEASE` is downloaded and used.

### Initial setup

1. Publish the toolchain as a GitHub Release (see the next section).
2. In the Cloudflare dashboard, connect this repository via **Workers & Pages → Create → Import a repository**.
   - Project name: `oneshot-rust` (must match `name` in `wrangler.jsonc`)
   - Root directory: `web`
   - Build command: `pnpm run build:deploy` (fetch the toolchain → build → check sizes)
   - Deploy command: leave the default (`npx wrangler deploy`)
   - Enable builds for non-production branches (to get preview URLs)
   - Enable preview URLs (Preview / Version URLs under Settings → Domains & Routes)
   - No build variables or secrets are needed (the toolchain is fetched from the Release's public download URLs)
3. To use the custom domain (oneshotrust.dev), add it under the Worker's **Settings → Domains & Routes**.

The pnpm version is pinned by `packageManager` in `package.json`. Node.js uses the build environment's default (24.x).

### Web app dependencies

The names, versions, licenses, and license texts of the npm packages bundled in the web app (Preact, CodeMirror,
fonts, etc.) are collected into `web/dist/third-party-licenses.txt` at build time and published at
`/third-party-licenses.txt` on the site (linked from the footer of the home page). The fonts (Barlow Condensed,
Caveat) are licensed under the SIL Open Font License 1.1.

### Updating the toolchain

```sh
./toolchain/build.nu     # build (placed in web/public/toolchain/)
./toolchain/publish.nu   # create a GitHub Release (toolchain-<id>) and update toolchain/RELEASE
```

Commit the updated `toolchain/RELEASE`.

### Checking and deploying locally

Use the following scripts in `web/`.

| Command | Description |
| --- | --- |
| `vp run serve:dist` | Serve the build at `http://localhost:8787` the same way Cloudflare does (including `public/_headers`) |
| `vp run deploy` | Fetch the toolchain → build → check sizes → `wrangler deploy` (production; requires `wrangler login`) |

The size check (`scripts/check-dist.ts`) fails the build if it exceeds the limits of 25 MiB per file or 20,000 files.

## Development

Run these in the `web/` directory.

| Command | Description |
| --- | --- |
| `vp check` | Formatting, lint, and type checking |
| `vp test` | Unit tests (pure functions such as the judging logic) |
| `node scripts/verify-problems.ts [id...]` | Verify all problems (e.g. that the reference solution gets AC) |
| `node scripts/judge.ts <id> <file.rs>` | Judge arbitrary code and show what the player would see |
| `node scripts/record-fixtures.ts` | Re-record Miri's actual output for the `classify` tests |

See [problems/README.md](problems/README.md) for how to write problems.

## License

- Code (everything outside `problems/`): your choice of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
- Problems (statements, explanations, templates, tests, and reference solutions in `problems/`):
  [CC BY 4.0](problems/LICENSE). Parts derived from the sources below remain under their original licenses.

Contributions submitted via pull requests or otherwise are treated as provided under the same licenses.

### Sources

The following problems are based on examples or code from
[The Rust Programming Language](https://github.com/rust-lang/book) (The Book) or
[The Rustonomicon](https://github.com/rust-lang/nomicon). Both are Copyright (c) 2010 The Rust Project Developers
and are provided under MIT or Apache-2.0.

| Source | Problems |
| --- | --- |
| The Book | `bufread-01`, `bufread-02`, `generics-01`, `guard-01`, `minigrep-01`, `pool-01`, `refcell-01`, `signature-01`, `state-02`, `string-02`, `string-05` |
| The Rustonomicon | `arc-01`, `itermut-01`, `rawvec-01` |

### Toolchain

The distributed toolchain (`miri.wasm` and the sysroot) is built from [rust-lang/rust](https://github.com/rust-lang/rust)
(MIT or Apache-2.0) and its dependency crates, plus wasi-libc and compiler-rt from
[wasi-sdk](https://github.com/WebAssembly/wasi-sdk). The license texts are distributed with the toolchain as
`licenses/` (see [toolchain/README.md](toolchain/README.md#license)).
