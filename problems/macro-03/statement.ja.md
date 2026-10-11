バリアントが自分の名前を知っている列挙型を定義する宣言的マクロ `named_enum!` を書いてください。
マクロは `#[macro_export]` 付きでエディタに入っていますが、今のルールは列挙型を定義するだけです。
残りも生成するようにしてください。エラー型 `UnknownName` は用意されています（エディタに入っています）。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownName(pub String);

#[macro_export]
macro_rules! named_enum { /* ... */ }
```

`named_enum! { Name { A, B = "b", C } }` は、列挙型の名前と、波かっこの中にカンマで区切ったバリアントを受け取ります。
バリアントは識別子で、その後に文字列リテラルを使った `= "ラベル"` が続くこともあります。
バリアントはいくつでもよく、1 つもなくてもかまいません（`named_enum! { Never {} }`）。
末尾のカンマがあってもかまいません。呼び出しは次のアイテムに展開されます。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Name { A, B, C }

impl Name {
    pub const ALL: &'static [Name];
    pub fn name(self) -> &'static str;
    pub fn from_name(name: &str) -> Option<Name>;
}

impl std::fmt::Display for Name { /* ... */ }

impl std::str::FromStr for Name {
    type Err = UnknownName;
    /* ... */
}
```

- バリアントの名前は、ラベルがあればそのラベル、なければ識別子です。上の例では、名前は `"A"`、`"b"`、`"C"` です。
  1 つの列挙型の中で、名前が同じバリアントはありません。
- `ALL` はすべてのバリアントを 1 回ずつ、書かれた順に含みます。
- `name()` はバリアントの名前を返します。
- `from_name(s)` は、名前がちょうど `s` であるバリアントを返します（大文字と小文字を区別し、前後の空白も取り除きません）。
  該当するバリアントがなければ `None` です。ラベルのあるバリアントの識別子は名前ではありません。
- `Display` は名前を書き出します。そのため、`to_string()` は `name()` と同じ文字列を返します。
- `s.parse::<Name>()` は、名前がちょうど `s` であるバリアントを `Ok` で返します。なければ
  `Err(UnknownName(s.to_string()))` を返します。中身は `s` そのままです。

テストは、テスト側のモジュールのアイテムを書く位置で、`crate::named_enum! { ... }` のようにパスでマクロを呼び出します
（`#[macro_export]` を付けたマクロはクレートのルートに置かれます）。そのようなモジュールはマクロを何度も呼び出し、
`named_enum` にも `UnknownName` にも標準ライブラリのものにも `use` 宣言がなく、独自の
`type Result<T> = std::result::Result<T, String>;` を定義していることもあります。
生成されたアイテムは、ほかのモジュールからも使われます。

### 例

```rust
crate::named_enum! { Color { Red, Green, Blue } }
crate::named_enum! { Dir { North = "N", East = "E", South = "S", West = "W", } }

Color::ALL                // &[Color::Red, Color::Green, Color::Blue]
Color::Green.name()       // "Green"
Color::from_name("Blue")  // Some(Color::Blue)
Color::from_name("blue")  // None
Color::Red.to_string()    // "Red"
"Green".parse::<Color>()  // Ok(Color::Green)
"Purple".parse::<Color>() // Err(UnknownName("Purple".to_string()))

Dir::ALL                  // &[Dir::North, Dir::East, Dir::South, Dir::West]
Dir::West.name()          // "W"
Dir::from_name("W")       // Some(Dir::West)
Dir::from_name("West")    // None
Dir::East.to_string()     // "E"
"S".parse::<Dir>()        // Ok(Dir::South)
"South".parse::<Dir>()    // Err(UnknownName("South".to_string()))
```
