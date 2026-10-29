あるサービスは、自分の状態を `{:?}` でログに書き出しています。パスワードがログに残っては
いけません。`Credentials` に `Debug` を手で実装してください（型はエディタに入っています）。

```rust
pub struct Credentials {
    pub user: String,
    pub password: String,
    pub scopes: Vec<String>,
}

impl fmt::Debug for Credentials { /* 実装する */ }
```

- `Credentials` は、フィールド `user`・`password`・`scopes` をこの順に持つ `Credentials` という
  名前の構造体に `#[derive(Debug)]` を付けたときと、まったく同じ表示にします。`user` と `scopes` は、
  `{:?}` が `String` と `Vec` を表示するときと同じ形にします。
- `password` の値は、パスワードが何であっても（空でも）、クォートなしの `<redacted>` という
  文字列にします。
- 出力は、derive した `Debug` と同じようにフォーマッタのフラグに従います。`{:#?}` の整形した形も、
  `{:#?}` で表示される別の値の中に入れ子になったときの追加の字下げも同じです。
- 幅・埋め文字・寄せ方・精度や、`+`・`0` のフラグは使いません。

### 例

```rust
let c = Credentials {
    user: "ann".to_string(),
    password: "hunter2".to_string(),
    scopes: vec!["read".to_string(), "write".to_string()],
};
```

`format!("{c:?}")` は

```text
Credentials { user: "ann", password: <redacted>, scopes: ["read", "write"] }
```

で、`format!("{c:#?}")` は次のとおりです。

```text
Credentials {
    user: "ann",
    password: <redacted>,
    scopes: [
        "read",
        "write",
    ],
}
```
