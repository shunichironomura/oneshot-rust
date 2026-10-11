# 問題の書き方

1 問を 1 ディレクトリにまとめます。ディレクトリ名が問題 ID になります。

```text
problems/<id>/
  problem.toml      メタデータ
  statement.md      問題文（英語、必須）
  statement.ja.md   問題文（日本語、任意。ない場合は英語版を表示）
  template.rs       初期コード
  tests.rs          テストハーネス
  solution.rs       模範解答（問題を終えたプレイヤーに表示。問題ごとに別チャンクで配信）
  editorial.md      解説（英語、任意。模範解答と一緒に表示）
  editorial.ja.md   解説（日本語、任意。editorial.md がある場合のみ）
```

## problem.toml

```toml
id = "lifetime-01"             # ディレクトリ名と同じにする
title = { en = "Longest word", ja = "最長の単語" }   # en は必須
difficulty = 2                 # 難易度ティア：1（初級）| 2（中級）| 3（上級）。出題と一覧の並び順に使う
concepts = ["lifetimes", "iterators"]  # 作者向けのメモ。ネタバレになるのでプレイヤーには表示しない
kind = "implement"             # implement | repair | signature

[judge]                        # 省略した項目は以下の既定値になる
forbid_unsafe = true           # プレイヤーのコードに #[forbid(unsafe_code)] を付ける
miri_checks = "full"           # full | fast（fast は Stacked Borrows と validation を無効化）
time_limit_ms = 15000
stdout_limit_bytes = 65536
harness_visible = false        # tests.rs をプレイヤーに見せるか
```

1 ラウンドは初級 2 問・中級 2 問・上級 1 問を、この順に出題します（`web/src/core/round.ts` の `ROUND_LADDER`）。
各ティアでは未出題の問題を優先するので、どのティアにも十分な数の問題があるようにしてください。

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
- ID が `sample-` で始まるケースが公開ケースです。それ以外は非公開ケースとして扱われ、失敗理由は表示されません。
- ケース ID は `tests.rs` から `case("...")` を抽出して取得します。ID は文字列リテラルで直接書いてください。

## 模範解答と解説

`solution.rs` はプレイヤーにも表示します。プレイヤーが読むことを前提に、慣用的で読みやすいコードにしてください。
`editorial.md` には、要点（なぜそう書くか、詰まりやすい点）と別解を簡潔に書きます。別解のコードも
コンパイルが通りテストに合格するものにしてください。

表示の条件は次のとおりです（`web/src/core/solution.ts`）。

- プレイ画面では、その挑戦を終える（AC またはギブアップ）と「模範解答」タブが現れる。挑戦の途中では見られない。
- 問題の履歴ページ（記録ページから開く）では、その問題の挑戦を一度でも終えていれば見られる。
- 一度表示すると、それ以降の挑戦（と中断している挑戦）は「解答閲覧済み」になり、自己ベストを更新しない。

模範解答はメインのバンドルには含めず、表示するときに問題ごとのチャンクを読み込みます。
判定はブラウザ内で行うので、隠しテストと同じく、技術的に秘匿されているわけではありません。

## 検証

`web/` で次を実行し、模範解答が AC になること、テンプレートが上の表の条件を満たすこと、
模範解答が制限時間の半分以内で終わることを確認します。

```sh
node scripts/verify-problems.ts            # 全問題
node scripts/verify-problems.ts iter-01    # 指定した問題だけ
```
