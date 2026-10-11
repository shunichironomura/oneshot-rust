次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

文字列を扱う小さな関数が 3 つあります。

```rust
pub fn initials(full_name: &str) -> String
pub fn join_path(dir: String, file: String) -> String
pub fn shout(msg: String) -> String
```

- `initials` は、`full_name` の各単語の最初の文字の後ろに `.` を付け、順につなげて返します。単語は
  任意の個数の空白文字（スペース、タブ、改行）で区切られ、先頭と末尾の空白文字は無視します。
  文字はそのまま使い、大文字・小文字は変えません。単語は ASCII の英字に限らず、どの Unicode 文字で
  始まっても構いません。単語が 1 つもなければ `""` を返します。
- `join_path` は `dir`、`/`、`file` をこの順につなげて返します。`dir` が空なら `file` をそのまま返します。
  それ以外の特別扱いはありません。`file` が空でも `/` は付きます（`"usr"` と `""` なら `"usr/"`）。
- `shout` は、`msg` が空なら `""` を返します。そうでなければ、`msg` を大文字にして後ろに `!` を付けて
  返します。大文字への変換は `str::to_uppercase` と同じく Unicode の規則に従うので、`ß` は `SS` に
  なります。

シグネチャは変えないでください。

### 例

- `initials("Grace Brewster Hopper")` は `"G.B.H."`、`initials("émile zola")` は `"é.z."` です。
- `join_path("usr".into(), "bin".into())` は `"usr/bin"` です。
- `shout("hey".into())` は `"HEY!"` です。
