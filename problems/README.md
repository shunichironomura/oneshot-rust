# 問題の書き方

1 問を 1 ディレクトリにまとめます。ディレクトリ名が問題 ID になります。

```text
problems/<id>/
  problem.toml      メタデータ
  statement.md      問題文（英語、必須）
  statement.ja.md   問題文（日本語、任意。ない場合は英語版を表示）
  template.rs       初期コード
  tests.rs          テストハーネス
  solution.rs       模範解答（検証専用。配信されません）
```

## problem.toml

```toml
id = "lifetime-01"             # ディレクトリ名と同じにする
title = { en = "Longest word", ja = "最長の単語" }   # en は必須
par = 2                        # 想定提出回数（ゴルフのパー）。難易度の指標を兼ね、一覧はこの順に並ぶ
concepts = ["lifetimes", "iterators"]
kind = "implement"             # implement | repair | signature

[judge]                        # 省略した項目は以下の既定値になる
forbid_unsafe = true           # プレイヤーのコードに #[forbid(unsafe_code)] を付ける
miri_checks = "full"           # full | fast（fast は Stacked Borrows と validation を無効化）
time_limit_ms = 15000
stdout_limit_bytes = 65536
harness_visible = false        # tests.rs をプレイヤーに見せるか
```

## 問題の種類

| kind | 内容 | template.rs |
| --- | --- | --- |
| `implement` | 仕様とシグネチャが与えられ、本体を実装する | コンパイルが通り、AC にはならないこと |
| `repair` | コンパイルの通らないコードを直す | CE になること |
| `signature` | 本体は与えられ、ハーネスが通るシグネチャを書く | CE になること。`harness_visible = true` が必須 |

## tests.rs

プレイヤーのコードは `crate::user`、判定ライブラリは `crate::judge` です。

```rust
use crate::judge::{case, check, check_eq};
use crate::user::longest_word;

pub fn run() {
    case("sample-1", || check_eq(longest_word("a bb ccc"), "ccc"));
    case("hidden-1", || check(longest_word("").is_empty(), "should be empty"));
}
```

- `case(id, f)`：1 つのテストケース。`f` は `Result<(), String>` を返します。パニックは FAIL として扱われ、後続のケースは続行します。
- `check_eq(got, expected)` / `check(cond, message)`：判定用のヘルパーです。
- ID が `sample-` で始まるケースが公開ケースです。それ以外は非公開ケースとして扱われ、標準モードでは失敗理由が表示されません。
- ケース ID は `tests.rs` から `case("...")` を抽出して取得します。ID は文字列リテラルで直接書いてください。

## 検証

`web/` で次を実行し、模範解答が AC になること、テンプレートが上の表の条件を満たすこと、
模範解答が制限時間の半分以内で終わることを確認します。

```sh
node scripts/verify-problems.ts            # 全問題
node scripts/verify-problems.ts iter-01    # 指定した問題だけ
```
