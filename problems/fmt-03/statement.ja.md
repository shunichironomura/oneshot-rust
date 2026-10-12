あるサービスは、自分の状態を `{:?}` でログに書き出しています。パスワードがログに残っては
いけませんし、大きすぎて出力する意味のないフィールドもあります。次の 3 つの型に `Debug` を
手で実装してください（型はエディタに入っています）。

```rust
pub struct Credentials {
    pub user: String,
    pub password: String,
    pub scopes: Vec<String>,
}

pub struct Inventory {
    pub items: BTreeMap<String, u32>,
    pub cache: Vec<u8>,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl fmt::Debug for Credentials { /* 実装する */ }
impl fmt::Debug for Inventory { /* 実装する */ }
impl fmt::Debug for Color { /* 実装する */ }
```

- どの型も、以下で説明する形の型に `#[derive(Debug)]` を付けたときと、まったく同じ表示にします。
  中に含まれる文字列・リスト・マップは、`{:?}` が `String`・`Vec`・`BTreeMap` を表示するときと
  同じ形にします。
- `Credentials` は、フィールド `user`・`password`・`scopes` をこの順に持つ、`Credentials` という
  名前の構造体のように表示します。ただし `password` の値は、パスワードが何であっても（空でも）、
  クォートなしの `<redacted>` という文字列にします。
- `Inventory` は、フィールド `items` だけを持つ `Inventory` という名前の構造体のように表示し、
  その後に、省略したものがあることを示す `..` を付けます。`cache` は表示しません。`cache` が空でも
  `..` は必ず付けます。
- `Color` は、`a` が 255（完全に不透明）のときはタプル構造体 `Rgb(r, g, b)` のように、それ以外の
  ときはタプル構造体 `Rgba(r, g, b, a)` のように表示します。
- 出力は、derive した `Debug` と同じようにフォーマッタのフラグに従います。`{:#?}` の整形した形、
  `{:x?}` と `{:X?}` の 16 進数の整数（`#` 付きなら `0x` が付く）、`{:#?}` で表示される別の値の中に
  入れ子になったときの追加の字下げも同じです。
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

`items` が `"pear"` → `1` と `"apple"` → `3` で、`cache` が 64 バイトの `Inventory` を `{:?}` と
`{:#?}` で表示すると、次のようになります。

```text
Inventory { items: {"apple": 3, "pear": 1}, .. }
```

```text
Inventory {
    items: {
        "apple": 3,
        "pear": 1,
    },
    ..
}
```

`red = Color { r: 255, g: 0, b: 16, a: 255 }`、`glass = Color { r: 255, g: 0, b: 16, a: 128 }` のとき、

| コード | 結果 |
| --- | --- |
| `format!("{red:?}")` | `Rgb(255, 0, 16)` |
| `format!("{red:x?}")` | `Rgb(ff, 0, 10)` |
| `format!("{glass:?}")` | `Rgba(255, 0, 16, 128)` |

で、`format!("{red:#X?}")` は次のとおりです。

```text
Rgb(
    0xFF,
    0x0,
    0x10,
)
```
