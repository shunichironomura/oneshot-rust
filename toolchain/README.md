# Toolchain

English | [日本語](README.ja.md)

`build.nu` builds Miri for the browser (`miri.wasm`) and the sysroot it loads.

## Usage

```sh
./toolchain/build.nu
```

The output is placed in `web/public/toolchain/`.

```text
web/public/toolchain/
  manifest.json                    Toolchain ID and file list (the frontend reads it first)
  <toolchainId>/miri.wasm.gz.<n>   Miri for wasm32-wasip1 (gzip-compressed and split)
  <toolchainId>/sysroot.tar.gz.<n> std etc. for x86_64-unknown-linux-gnu (rlibs with MIR; same as above)
  <toolchainId>/licenses/          License texts of the components in the two above (uncompressed)
```

- `toolchainId` is a content hash of the two files. The browser stores the decompressed files in Cache Storage under
  keys that include this ID, so they are downloaded again only when the toolchain changes.
- To fit within the 25 MiB per-file limit of Cloudflare (Workers static assets), each file is gzip-compressed and
  then split into 20 MiB chunks. The browser downloads the chunks in parallel and decompresses them with
  `DecompressionStream` (about 55 MiB transferred in total).

The working directory is `toolchain/work/` (not tracked by git). Later builds rebuild only what changed.

## Publishing and fetching

Building takes a long time, so the output is shared through GitHub Releases.

- `./toolchain/publish.nu`: uploads the contents of `web/public/toolchain/` as the Release
  `toolchain-<toolchainId>` and writes its tag to `toolchain/RELEASE`. Commit `RELEASE`.
- `vp run fetch-toolchain` in `web/` (`scripts/fetch-toolchain.ts`): downloads the Release in `toolchain/RELEASE` and
  places it in `web/public/toolchain/`. CI and the Cloudflare build use this too. It does nothing if the same version
  is already in place. It fetches from the Release's public download URLs, so no token is needed.

## Requirements

- Nushell, git, Python 3 (for `x.py`)
- Tested on macOS (aarch64). It is expected to work on Linux too, but this is untested.
- The first build takes about 20–30 minutes (on a 16-core machine) and uses about 30 GB of disk.

## How it works

1. Check out `rust-lang/rust` at `RUST_COMMIT` and apply `patches/*.patch`.
2. Download wasi-sdk.
3. Expand `bootstrap.toml` and build Miri for a wasm32-wasip1 host with `x.py` (stage 2).
4. Build native Miri and cargo-miri from the same source tree, and use them to create the sysroot for the
   interpreted target (x86_64-unknown-linux-gnu) with `cargo miri setup`.
   Building from the same commit keeps the metadata compatible.
5. Collect the license texts (see "License" below).
6. Place `miri.wasm`, the sysroot's `.rlib` files, and the license texts together.

## Patches

`patches/miri-wasm32-wasip1-host.patch` is the minimal change needed to run Miri on wasm32-wasip1 (no threads).

- rustc: don't spawn compiler threads, don't load dylibs, and assume `--sysroot` is given explicitly.
- bootstrap: keep the rlibs of rustc's crates on Wasm hosts (to link Miri statically).
- Miri: make host socket and file access unsupported on targets other than unix/windows
  (neither is used, since Miri runs with isolation enabled).

It is based on [bjorn3's work on compiling rustc to Wasm](https://github.com/rust-lang/miri/issues/722).

## License

The distributed binaries ship with the following license texts as `licenses/`. `build.nu` collects them into
`toolchain/work/licenses/` and lists them under `licenses` in `manifest.json`. `publish.nu` also includes them in
the Release assets.

| File | Contents |
| --- | --- |
| `README.txt` | Description of each file |
| `rust-COPYRIGHT`, `rust-LICENSE-MIT`, `rust-LICENSE-APACHE` | License of Rust itself (MIT or Apache-2.0) |
| `rust-COPYRIGHT.html` | Licenses of rustc, Miri, the standard library, and all their dependency crates (for `miri.wasm`) |
| `rust-COPYRIGHT-library.html` | Licenses of the standard library and its dependency crates only (for the sysroot) |
| `wasi-libc-*` | Licenses of wasi-libc (including musl, cloudlibc, and musl-fts), linked into `miri.wasm` |
| `compiler-rt-LICENSE.TXT` | License of the compiler-rt builtins, linked into `miri.wasm` |

- `rust-COPYRIGHT*.html` is generated with `./x.py run src/tools/generate-copyright` in rust-lang/rust, the same as
  the files shipped with official Rust releases. It runs `cargo vendor` on the dependency crates, so it needs network
  access.
- The license texts of wasi-libc and compiler-rt are downloaded from the commit listed in wasi-sdk's `VERSION`
  (for LLVM, the version tag).

Releases cannot be changed after publishing (they are immutable), so `publish.nu` does nothing if a Release with the
same ID already exists.

## Updating `RUST_COMMIT`

1. Change `RUST_COMMIT` to a newer upstream merge commit.
2. Run `./toolchain/build.nu`. If the patch doesn't apply, fix it up in `toolchain/work/rust` and regenerate the
   patch with:

   ```sh
   git -C toolchain/work/rust diff > toolchain/patches/miri-wasm32-wasip1-host.patch
   ```

3. Run `node scripts/verify-problems.ts` in `web/` and confirm that all problems pass.

`download-ci-llvm = true` works only for commits whose CI LLVM artifacts are still available (within a few months).
To rebuild an older commit, set `download-ci-llvm = false` in `bootstrap.toml`
(LLVM is then built locally, which needs ninja and takes longer).

## Known limitations

- rustc on WASI is panic=abort, so a compile error ends in a Wasm trap rather than exit code 1.
  The judge (`web/src/core/classify.ts`) accounts for this.
- When decompressed, miri.wasm is about 119 MB and the sysroot about 113 MB. They occupy that much browser memory.
