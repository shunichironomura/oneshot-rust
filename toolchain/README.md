# ツールチェーン

`build.nu` は、ブラウザで動かす Miri（`miri.wasm`）と、それが読み込む sysroot をビルドします。

## 使い方

```sh
./toolchain/build.nu
```

成果物は `web/public/toolchain/` に配置されます。

```text
web/public/toolchain/
  manifest.json            ツールチェーン ID とファイル一覧（フロントエンドが最初に読む）
  <toolchainId>/miri.wasm  wasm32-wasip1 向けの Miri
  <toolchainId>/sysroot.tar  x86_64-unknown-linux-gnu 向けの std など（MIR 付き rlib）
```

`toolchainId` は 2 つのファイルの内容ハッシュです。ブラウザはこの ID を含む URL で Cache Storage に
キャッシュするため、ツールチェーンを更新したときだけ再ダウンロードされます。

作業ディレクトリは `toolchain/work/`（git 管理外）です。2 回目以降は差分だけがビルドされます。

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
5. `miri.wasm` と sysroot の `.rlib` をまとめて配置します。

## パッチ

`patches/miri-wasm32-wasip1-host.patch` は、Miri を wasm32-wasip1（スレッドなし）で動かすための最小限の変更です。

- rustc：コンパイラスレッドを作らない、dylib をロードしない、`--sysroot` の明示を前提にする。
- bootstrap：Wasm ホストでは rustc の各クレートの rlib を残す（Miri を静的リンクするため）。
- Miri：ホストのソケットとファイルアクセスを、unix/windows 以外では未サポートにする
  （isolation を有効にして動かすので、どちらも使われません）。

[bjorn3 氏の rustc の Wasm 化の取り組み](https://github.com/rust-lang/miri/issues/722)を参考にしています。

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
- 配信サイズが大きくなっています（miri.wasm 約 119 MB、sysroot 約 113 MB。gzip でそれぞれ約 22 MB と 35 MB）。
  Cloudflare Pages の 1 ファイル 25 MiB の上限を超えるので、デプロイ時は分割配信などが必要です。
