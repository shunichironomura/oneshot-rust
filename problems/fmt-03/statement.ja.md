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
  同じ形にします。文字列は特殊文字をエスケープしてダブルクォートで囲み、リストは `[...]`、
  マップはキーの順に `{key: value, ...}` と表示します。
- `Credentials` は、フィールド `user`・`password`・`scopes` をこの順に持つ、`Credentials` という
  名前の構造体のように表示します。ただし `password` の値は、パスワードが何であっても（空でも）、
  クォートなしの `<redacted>` という文字列にします。
- `Inventory` は、フィールド `items` だけを持つ `Inventory` という名前の構造体のように表示し、
  その後に、省略したものがあることを示す `..` を付けます。`cache` は表示しません。`cache` が空でも
  `..` は必ず付けます。
- `Color` は、`a` が 255（完全に不透明）のときはタプル構造体 `Rgb(r, g, b)` のように、それ以外の
  ときはタプル構造体 `Rgba(r, g, b, a)` のように表示します。
- 出力は、derive した `Debug` と同じように書式指定に従います。
  - `{:?}` は、例のようにすべてを 1 行で表示します。
  - `{:#?}` は整形した形で表示します。1 行目は名前と開き括弧 `{` または `(` です。続いて
    フィールドを 1 つずつ別の行に、空白 4 つで字下げし、後ろに `,` を付けて書きます。`Inventory` の
    `..` はフィールドの後の独立した行に書き、`,` は付けません。最後の行は閉じ括弧 `}` または `)` です。
    中に含まれる空でないリストやマップも同じように展開し、さらに空白 4 つ分字下げします。
    空のものは `[]` や `{}` のままです。
  - `{:x?}` と `{:X?}` は、中に含まれるすべての整数を、それぞれ小文字・大文字の 16 進数で、接頭辞なしで
    表示します（`ff`、`FF`）。さらに `#` も付けた `{:#x?}` と `{:#X?}` では、整形した形になり、
    すべての整数に接頭辞 `0x` が付きます（`0xff`、`0xFF`、`0x0`）。
  - 値が別の値の一部として表示されるとき（たとえば `Vec` の要素として `{:#?}` で表示されるとき）も、
    ここまでの規則はすべてそのまま成り立ちます。そのときは、ほかの入れ子の値と同じように、
    各行がもう 1 段字下げされます。
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
