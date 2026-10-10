# OneShot Rust

Write Rust that compiles in one shot — no LSP, as few compiles as possible.

OneShot Rust は、指定された処理を Rust で実装し、**できるだけ少ない提出（＝コンパイル）回数で全テストを通す**ことを競う
ブラウザゲームです。補完も LSP もありません。判定は Wasm 化した [Miri](https://github.com/rust-lang/miri) が
ブラウザ内で行うので、型検査・借用検査は本物の rustc と同じで、未定義動作（UB）も検出されます。
UI と問題文は英語が既定で、日本語にも切り替えられます。

## 構成

```text
toolchain/   Miri を wasm32-wasip1 向けにビルドするスクリプトとパッチ
judge/       判定ライブラリ（judge.rs）と固定テンプレート（main.rs）
problems/    問題（1 問 1 ディレクトリ）
web/         フロントエンド（Vite+ / Preact / CodeMirror 6）
```

## ローカルで遊ぶ

必要なもの：[Nushell](https://www.nushell.sh/)、Python 3、git、Node.js 24 以降、
[Vite+](https://viteplus.dev/)（`vp`）。

1. ツールチェーン（`miri.wasm` と sysroot）をビルドします。初回は 20〜30 分ほどかかり、
   `web/public/toolchain/` に配置されます。詳しくは [toolchain/README.md](toolchain/README.md) を参照してください。

   ```sh
   ./toolchain/build.nu
   ```

2. 開発サーバーを起動します。

   ```sh
   cd web
   vp install
   vp dev
   ```

   初回アクセス時にツールチェーン（圧縮して約 55 MB）をダウンロードし、Cache Storage に保存します。
   2 回目以降はキャッシュから読み込みます。

## Cloudflare Pages へのデプロイ

`web/dist/` を Wrangler で直接アップロードします（Pages 側でのビルドはしません）。ツールチェーンのビルドに
20〜30 分かかり、Pages のビルド制限時間（20 分）を超えるためです。

1. 初回のみ、ログインしてプロジェクトを作ります。

   ```sh
   cd web
   vp exec wrangler login
   vp exec wrangler pages project create oneshot-rust --production-branch main
   ```

2. ビルドしてデプロイします。`deploy` は本番、`deploy:preview` はプレビュー用の URL に配置します。

   ```sh
   vp run deploy
   ```

どちらのスクリプトも、ビルドのあとに `scripts/check-dist.ts` で Pages の制限（1 ファイル 25 MiB、20,000 ファイル）を
確認し、超えていればデプロイを中止します。`vp run pages:dev` を使うと、Pages と同じ配信
（`public/_headers` のキャッシュ設定を含む）を手元の `http://localhost:8788` で確認できます。

CI から実行する場合は、`CLOUDFLARE_ACCOUNT_ID` と `CLOUDFLARE_API_TOKEN`（Cloudflare Pages の Edit 権限）を設定してください。

## 開発

`web/` ディレクトリで実行します。

| コマンド | 内容 |
| --- | --- |
| `vp check` | フォーマット、lint、型検査 |
| `vp test` | ユニットテスト（判定ロジックなどの純粋関数） |
| `node scripts/verify-problems.ts [id...]` | 全問題の検証（模範解答が AC になるかなど） |
| `node scripts/judge.ts <id> <file.rs> [--mode <mode>]` | 任意のコードを判定し、プレイヤーに見える内容を表示 |
| `node scripts/record-fixtures.ts` | `classify` のテスト用に、Miri の実際の出力を記録し直す |

問題の書き方は [problems/README.md](problems/README.md) を参照してください。
