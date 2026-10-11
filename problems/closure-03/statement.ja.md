次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`tally` はイベントの列を順に見ていき、ログを作ります。`make_greeter` はあいさつをする関数を作ります。

```rust
pub fn tally(events: &[&str], limit: usize) -> (usize, Vec<String>)
pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String
```

- `tally` は `events` を先頭から順に処理し、各イベントをログの末尾に追加します。`ERR` で始まるイベントはエラーです。
  大文字と小文字は区別し、先頭だけを見ます。`"ERROR 7"` はエラーですが、`"err"`、`"Err"`、`" ERR"` は
  エラーではありません。
- イベントを追加した直後に、その時点までのエラーの数（そのイベントも含みます）が `limit` と
  等しければ、ログに `"abort"` を追加して、残りのイベントは無視します。`limit` は 1 以上です。
- `tally` はエラーの数とログを返します。
- `make_greeter(greeting)` は、`name` を `"<greeting>, <name>!"` に変える関数を返します。`<greeting>` は
  `greeting` の両端の空白を取り除いたもの（`str::trim` と同じ）です。`name` はそのまま使います。
  返した関数は何回でも呼べます。

シグネチャは変えないでください。関数の本体の中は自由に変えて構いません。

### 例

```rust
tally(&["ok", "ERR a", "ERR b", "ok"], 2)
// (2, ["ok", "ERR a", "ERR b", "abort"])

tally(&["ok"], 1)
// (0, ["ok"])

let hi = make_greeter(String::from("  Hi "));
hi("Ann")  // "Hi, Ann!"
```
