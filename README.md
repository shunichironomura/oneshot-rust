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

1. ツールチェーン（`miri.wasm` と sysroot）を `web/public/toolchain/` に配置します。公開済みのものを
   ダウンロードする（`gh` が必要）か、自分でビルドします（初回は 20〜30 分）。詳しくは
   [toolchain/README.md](toolchain/README.md) を参照してください。

   ```sh
   cd web && vp run fetch-toolchain   # toolchain/RELEASE の版をダウンロード（gh のログインが必要）
   # または
   ./toolchain/build.nu               # ソースからビルド
   ```

2. 開発サーバーを起動します。

   ```sh
   cd web
   vp install
   vp dev
   ```

   初回アクセス時にツールチェーン（圧縮して約 55 MB）をダウンロードし、Cache Storage に保存します。
   2 回目以降はキャッシュから読み込みます。

## デプロイ（Cloudflare Workers Builds）

Cloudflare の Git 連携（Workers Builds）が、リポジトリへの push を検知してビルドとデプロイを行います。
`main` は本番に、それ以外のブランチとプルリクエストはプレビュー URL にデプロイされます（URL は Cloudflare が PR にコメントします）。
GitHub 側に Cloudflare の認証情報は置きません。配信は Workers の静的アセットで、設定は `web/wrangler.jsonc` にあります。

GitHub Actions（`.github/workflows/ci.yml`）は、チェック・テスト・全問題の検証・ビルドだけを行います。

ツールチェーンはどちらでもビルドしません（20〜30 分かかり、ディスクも 30 GB ほど使うため）。手元でビルドしたものを
GitHub Release として公開し、`toolchain/RELEASE` に記録した版をダウンロードして使います。

### 初回のセットアップ

1. ツールチェーンを GitHub Release として公開します（次の節）。
2. Cloudflare のビルドが Release を読むための GitHub トークンを作ります。fine-grained personal access token で、
   対象リポジトリを `oneshot-rust` に限定し、権限は **Contents: Read-only** だけにします。
3. Cloudflare のダッシュボードで **Workers & Pages → Create → Import a repository** から、このリポジトリを接続します。
   - プロジェクト名：`oneshot-rust`（`wrangler.jsonc` の `name` と一致させる）
   - Root directory：`web`
   - Build command：`pnpm run build:deploy`（ツールチェーンの取得 → ビルド → サイズの確認）
   - Deploy command：既定（`npx wrangler deploy`）のまま
   - 本番以外のブランチのビルドを有効にする（プレビュー URL を出すため）
   - Build variables and secrets：手順 2 のトークンを、シークレット `GITHUB_TOKEN` として登録する。
     **Production と Previews の両方に登録すること**（Production だけだと、PR のプレビュービルドで取得に失敗する）
   - プレビュー URL（Settings → Domains & Routes の Preview / Version URLs）を有効にする
4. 独自ドメイン（oneshotrust.dev）を使う場合は、Worker の **Settings → Domains & Routes** で追加します。

pnpm のバージョンは `package.json` の `packageManager` で固定しています。Node.js はビルド環境の既定（24 系）を使います。

### ツールチェーンを更新したとき

```sh
./toolchain/build.nu     # ビルド（web/public/toolchain/ に配置）
./toolchain/publish.nu   # GitHub Release（toolchain-<id>）を作成し、toolchain/RELEASE を更新
```

更新された `toolchain/RELEASE` をコミットします。

### 手元で確認・デプロイする場合

`web/` で次のスクリプトを使います。

| コマンド | 内容 |
| --- | --- |
| `vp run serve:dist` | ビルドしたものを Cloudflare と同じ配信（`public/_headers` を含む）で `http://localhost:8787` に出す |
| `vp run deploy` | ツールチェーンの取得 → ビルド → サイズ確認 → `wrangler deploy`（本番。要 `wrangler login`） |

サイズの確認（`scripts/check-dist.ts`）は、1 ファイル 25 MiB・20,000 ファイルの制限を超えるとビルドを失敗させます。

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
