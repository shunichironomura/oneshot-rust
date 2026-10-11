テキストアドベンチャーでプレイヤーが入力した 1 行を `Command` に変換してください。正しいコマンドでなければ `None` を返します。

```rust
#[derive(Debug, PartialEq)]
pub enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Wait(u32),
    Quit,
}

pub fn parse_command(line: &str) -> Option<Command>
```

行は空白で単語に分けます。大文字と小文字は区別します。

| 単語 | コマンド |
| --- | --- |
| `move X Y`（`X` と `Y` は `i32`） | `Move { x: X, y: Y }` |
| `go north` / `go south` / `go east` / `go west` | 1 歩ぶんの `Move`：`(0, 1)` / `(0, -1)` / `(1, 0)` / `(-1, 0)` |
| `say` の後に **1 つ以上**の単語 | それらの単語を空白 1 つでつないだ `Say` |
| `wait` | `Wait(1)` |
| `wait N`（`N` は 1 以上 60 以下） | `Wait(N)` |
| `quit` または `exit` | `Quit` |

それ以外（単語が多すぎる・足りない場合を含む）はすべて `None` です。

### 例

| 入力 | 出力 |
| --- | --- |
| `"move 3 -2"` | `Some(Move { x: 3, y: -2 })` |
| `"go west"` | `Some(Move { x: -1, y: 0 })` |
| `"say  hello   world "` | `Some(Say("hello world"))` |
| `"wait"` | `Some(Wait(1))` |
| `"wait 61"` | `None` |
| `"exit"` | `Some(Quit)` |
| `"dance"` | `None` |
