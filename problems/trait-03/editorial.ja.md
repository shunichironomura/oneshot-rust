デフォルトメソッドの本体は、`Point` や `Tags` ごとにではなく、そのトレイトを実装しうるすべての型に対して
1 回だけ型検査されます。テンプレートの `Outline` は `Self` について何も述べていないので、`outline` の中で
`self.to_string()` や `format!("{self}")` と書いてもコンパイルが通りません。`Self` が `Display` を実装していると
コンパイラは仮定できないからです。`pub trait Outline: fmt::Display` と宣言すると、`Display` が
**スーパートレイト**になります。`Outline` を実装する型は `Display` も実装しなければならない代わりに、
トレイト自身のメソッドの中で `Display` を使えるようになります。`to_string` は標準ライブラリのブランケット実装
`impl<T: Display + ?Sized> ToString for T` から得られます。ハーネスの型も `Display` を実装しているので、
`Point` や `Tags` と同じように、空の `impl Outline` が受け入れられます。

幅は `len()` ではなく `char` の個数です。`len()` は UTF-8 のバイト数を数えます。`é` は 2 バイトなので、
`len()` を使うと `héllo` を囲む枠が 1 列広くなってしまいます。

`Tags` は **ニュータイプ**です。`impl fmt::Display for Vec<String>` は、トレイトも型も別のクレートのものなので、
孤児ルールによって拒否されます（E0117）。`Vec` を包んだタプル構造体はローカルな型なので、`Display` を実装できます。
中では `self.0` がベクタで、`join(", ")` で一覧を作れます。文字列がなければ空になるので、`[]` を特別扱いする
必要はありません。

### 別解

スーパートレイトの代わりに、メソッド自体に境界を付けても通ります。違いは、`Display` を実装していない型にも
`Outline` を実装できるようになること（その型では `outline` を呼べないだけです）と、`T: Outline` だけを境界に
持つジェネリックなコードからは、`outline` を呼ぶことも値を表示することもできないことです。枠の行は、
書式指定の埋め文字と幅でも作れます。

```rust
pub trait Outline {
    fn outline(&self) -> String
    where
        Self: fmt::Display,
    {
        let text = self.to_string();
        let width = text.chars().count();
        let border = format!("{:*<n$}", "", n = width + 4);
        let blank = format!("*{:n$}*", "", n = width + 2);
        format!("{border}\n{blank}\n* {text} *\n{blank}\n{border}")
    }
}
```

コンパイラが提案する `pub trait Outline: ToString` でも通ります。`Display` を実装した型はすべて `ToString` を
実装しているからです。

`Tags` は、つないだ `String` を先に作らずに、部品ごとに書き出すこともできます。

```rust
impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        for (i, tag) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            f.write_str(tag)?;
        }
        f.write_str("]")
    }
}
```
