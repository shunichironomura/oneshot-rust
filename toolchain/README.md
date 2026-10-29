# ツールチェーン

`build.nu` は、ブラウザで動かす Miri（`miri.wasm`）と、それが読み込む sysroot をビルドします。

## 使い方

```sh
./toolchain/build.nu
```

成果物は `web/public/toolchain/` に配置されます。

```text
web/public/toolchain/
  manifest.json                    ツールチェーン ID とファイル一覧（フロントエンドが最初に読む）
  <toolchainId>/miri.wasm.gz.<n>   wasm32-wasip1 向けの Miri（gzip 圧縮して分割）
  <toolchainId>/sysroot.tar.gz.<n> x86_64-unknown-linux-gnu 向けの std など（MIR 付き rlib、同上）
  <toolchainId>/licenses/          上の 2 つに含まれるコンポーネントのライセンス文（圧縮なし）
```

- `toolchainId` は 2 つのファイルの内容ハッシュです。ブラウザは展開後のファイルを、この ID を含むキーで Cache Storage に
  保存するため、ツールチェーンを更新したときだけ再ダウンロードされます。
- Cloudflare（Workers の静的アセット）の 1 ファイル 25 MiB の上限に収めるため、各ファイルは gzip で圧縮したうえで 20 MiB ごとに分割しています。
  ブラウザは断片を並列にダウンロードし、`DecompressionStream` で展開します（転送量は合計で約 55 MiB）。

作業ディレクトリは `toolchain/work/`（git 管理外）です。2 回目以降は差分だけがビルドされます。

## 公開と取得

ビルドには時間がかかるので、成果物は GitHub Release で共有します。

- `./toolchain/publish.nu`：`web/public/toolchain/` の中身を Release `toolchain-<toolchainId>` としてアップロードし、
  そのタグを `toolchain/RELEASE` に書き込みます。`RELEASE` はコミットしてください。
- `web/` で `vp run fetch-toolchain`（`scripts/fetch-toolchain.ts`）：`toolchain/RELEASE` の Release をダウンロードし、
  `web/public/toolchain/` に配置します。CI と Cloudflare のビルドもこれを使います。同じ版が配置済みなら何もしません。
  Release の公開ダウンロード URL から取得するので、トークンは要りません。

## 必要なもの

- Nushell、git、Python 3（`x.py` 用）
- macOS（aarch64）で動作を確認しています。Linux でも動く想定ですが、未検証です。
- 初回ビルドには 20〜30 分ほどかかります（16 コアのマシンでの目安）。ディスクは 30 GB 程度使います。

## 処理の流れ

1. `rust-lang/rust` を `RUST_COMMIT` でチェックアウトし、`patches/*.patch` を適用します。
2. wasi-sdk をダウンロードします。
3. `bootstrap.toml` を展開し、`x.py` で Miri を wasm32-wasip1 ホスト向けにビルドします（stage 2）。
4. 同じソースツリーからネイティブ版の Miri と cargo-miri をビルドします。それを使い、
   `cargo miri setup` で解釈対象ターゲット（x86_64-unknown-linux-gnu）の sysroot を作ります。
   同じコミットからビルドするので、メタデータの互換性が保たれます。
5. ライセンス文を集めます（下の「ライセンス」を参照）。
6. `miri.wasm` と sysroot の `.rlib`、ライセンス文をまとめて配置します。

## パッチ

`patches/miri-wasm32-wasip1-host.patch` は、Miri を wasm32-wasip1（スレッドなし）で動かすための最小限の変更です。

- rustc：コンパイラスレッドを作らない、dylib をロードしない、`--sysroot` の明示を前提にする。
- bootstrap：Wasm ホストでは rustc の各クレートの rlib を残す（Miri を静的リンクするため）。
- Miri：ホストのソケットとファイルアクセスを、unix/windows 以外では未サポートにする
  （isolation を有効にして動かすので、どちらも使われません）。

[bjorn3 氏の rustc の Wasm 化の取り組み](https://github.com/rust-lang/miri/issues/722)を参考にしています。

## ライセンス

配信するバイナリには、次のライセンス文を `licenses/` として同梱します。`build.nu` が `toolchain/work/licenses/` に集め、
`manifest.json` の `licenses` に列挙します。`publish.nu` はそれらを Release のアセットにも含めます。

| ファイル | 内容 |
| --- | --- |
| `README.txt` | 各ファイルの説明 |
| `rust-COPYRIGHT`、`rust-LICENSE-MIT`、`rust-LICENSE-APACHE` | Rust 本体のライセンス（MIT または Apache-2.0） |
| `rust-COPYRIGHT.html` | rustc・Miri・標準ライブラリと、その依存クレートすべてのライセンス（`miri.wasm` 用） |
| `rust-COPYRIGHT-library.html` | 標準ライブラリとその依存クレートだけのライセンス（sysroot 用） |
| `wasi-libc-*` | `miri.wasm` にリンクされる wasi-libc（musl、cloudlibc、musl-fts を含む）のライセンス |
| `compiler-rt-LICENSE.TXT` | `miri.wasm` にリンクされる compiler-rt builtins のライセンス |

- `rust-COPYRIGHT*.html` は、Rust の公式リリースに同梱されるものと同じく、rust-lang/rust の
  `./x.py run src/tools/generate-copyright` で生成します。依存クレートを `cargo vendor` するため、ネットワークが必要です。
- wasi-libc と compiler-rt のライセンス文は、wasi-sdk の `VERSION` に書かれたコミット（LLVM はバージョンのタグ）から
  ダウンロードします。

licenses を集める前に作った Release を更新するには、`./toolchain/build.nu` を実行し直してから `./toolchain/publish.nu` を
実行します。同じ ID の Release がすでにあれば、`manifest.json` とライセンス文だけをアップロードし直します
（ビルドが再現せず ID が変わった場合は、新しい Release ができます）。

## `RUST_COMMIT` を更新するとき

1. `RUST_COMMIT` を新しい upstream のマージコミットに変更します。
2. `./toolchain/build.nu` を実行します。パッチが当たらない場合は、`toolchain/work/rust` で手直ししてから、
   次のコマンドでパッチを作り直します。

   ```sh
   git -C toolchain/work/rust diff > toolchain/patches/miri-wasm32-wasip1-host.patch
   ```

3. `web/` で `node scripts/verify-problems.ts` を実行し、全問題が通ることを確認します。

`download-ci-llvm = true` は、CI の LLVM アーティファクトが残っている（数か月以内の）コミットでしか使えません。
古いコミットをビルドし直す場合は、`bootstrap.toml` で `download-ci-llvm = false` にしてください
（LLVM を自前でビルドするため ninja が必要で、時間もかかります）。

## 既知の制約

- WASI 上の rustc は panic=abort なので、コンパイルエラー時は終了コード 1 ではなく Wasm の trap で終わります。
  判定側（`web/src/core/classify.ts`）で考慮しています。
- 展開後のサイズは miri.wasm が約 119 MB、sysroot が約 113 MB です。ブラウザのメモリにはこのサイズで載ります。
