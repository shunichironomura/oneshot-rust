1 つのルールを、複数のアイテム（列挙型、固有の `impl`、2 つのトレイト実装）に展開できます。その中では、
繰り返しでバリアントごとの部分を書きます。`ALL` には `&[$(Self::$variant),*]`、`name` にはバリアントごとに
1 つずつの `match` のアームです。パターンではバリアントをパス `Self::$variant` で書く必要があります。
`$variant` とだけ書くと、何にでもマッチする新しい変数になり、rustc はこれを拒否します（E0170）。
テストのバリアント `None` では、`Option` の `None` になってしまいます。

`$(...)?` には、ラベルが「なかったとき」の書き方がありません。模範解答は `$($label)?` を
内部ルールに渡しています。内部ルールが受け取るのは `@name Red` か `@name North "N"` のどちらかで、
それぞれに 1 つずつルールがあります。`stringify!($variant)` は識別子を文字列リテラル `"Red"` にします。
トランスクライバーでは、`$label` は `$(...)?` の中に置く必要があります。2 重の繰り返しの中でマッチした
メタ変数なので、アームに `$label` とだけ書くと「variable `label` is still repeating at this depth」になります。

`#[macro_export]` が効いてくるのは、この内部ルールの呼び出しです。`#[macro_export]` はマクロをクレートの
ルートに置くので、呼び出しは `$crate::named_enum!` です。`$crate::user::named_enum!` ではありません
（マクロは `user` のアイテムではありません）。名前だけの `named_enum!` も、テストのモジュールでは
スコープにないので使えません（「cannot find macro `named_enum` in this scope」）。同じ理由で `$crate` は
クレートのルートを指すので、用意された型は、定義された場所の `$crate::user::UnknownName` で指します。

展開結果の中のそれ以外のパスは、マクロを呼び出した場所で解決されます。テストは標準ライブラリのものを
何も `use` していないので、`fmt::Display` は見つからず（「cannot find module or crate `fmt` in this scope」）、
`Result<Self, Self::Err>` はテスト自身の型引数 1 つの別名を指してしまいます
（「type alias takes 1 generic argument but 2 generic arguments were supplied」）。
`::std::fmt::Display` や `::std::result::Result` のような絶対パスなら、どのモジュールでも使えます。

生成したアイテムも、呼び出した側のモジュールのアイテムになります。テストは 1 つのモジュールでマクロを
何度も呼び出すので、展開結果に `use std::fmt;` や決まった名前の補助関数があると、それが 2 回定義されます
（E0252、E0428）。型が違う `impl` どうしはぶつからないので、補助的なものはその中に入れてください
（別解も参照）。

バリアントのない列挙型も受け付けます。`name(self)` の `match self {}` は、`self` が値を 1 つも持たない型の値なので
コンパイルが通ります。ところが `Display::fmt` が受け取るのは `&self` で、参照は常に値を持つものとして扱われます。
そのため、そこで `match self {}` と書くと「non-exhaustive patterns: type `&Never` is non-empty」になります。
`self.name()` を呼べば、値がコピーされ、`name` の `match` を使い回せます（`match *self { ... }` でも動きます）。
同じように `from_name` は `ALL` と `name()` を使い回すので、ラベルの扱いは 1 か所で済み、`from_str` は
`from_name` を使い回します。

### 別解

内部ルールを使わずに、配列リテラルでラベルを選ぶこともできます。ラベルがあるとアームは
`["N", "North"][0]` に、ないと `["Red"][0]` になります。

```rust
pub fn name(self) -> &'static str {
    match self {
        $(Self::$variant => [$($label,)? ::std::stringify!($variant)][0],)*
    }
}
```

`from_name` も文字列の `match` で書けます。マクロ呼び出しはパターンとしても使え、内部ルールは文字列リテラルに
展開されます。

```rust
pub fn from_name(name: &str) -> ::std::option::Option<Self> {
    match name {
        $($crate::named_enum!(@name $variant $($label)?) => {
            ::std::option::Option::Some(Self::$variant)
        })*
        _ => ::std::option::Option::None,
    }
}
```

名前のない定数 `const _` は 1 つのモジュールにいくつでも定義でき、そのブロックの中の `use` 宣言はブロックの中だけで
有効です。中の `impl` はどこからでも有効で、ブロック内の `use ::std::result::Result;` が呼び出し側の別名を隠します。

```rust
const _: () = {
    use ::std::fmt::{self, Display, Formatter};
    use ::std::result::Result;
    use ::std::str::FromStr;
    use $crate::user::UnknownName;

    impl Display for $name {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str(self.name())
        }
    }

    impl FromStr for $name {
        type Err = UnknownName;

        fn from_str(s: &str) -> Result<Self, UnknownName> {
            Self::from_name(s).ok_or_else(|| UnknownName(s.to_string()))
        }
    }
};
```
