罠は、`Display` のよくある書き方です。`write!(f, "{sign}${dollars}.{cents:02}")` は正しい文字列を出力しますが、
余白が消えます。呼び出し側が指定した幅・埋め文字・寄せ方（`{:>10}`）は `f` に入っています。ところが
`write!` は自分の書式文字列に書かれた指定だけで各部品を書式化するので、`f` の指定はどこにも使われません。
`{:>10}` でも `$0.05` だけが出力されます。

`str` 自身が使っているのが `Formatter::pad` です。`f` の幅・埋め文字・寄せ方（と、最大の長さとしての精度）を
適用して文字列を書き込みます。そこで、符号も含めた表示全体を先に組み立て、`f.pad(&text)` に渡します。
一部だけに余白を付けるのはうまくいきません。先に `-` を書いてから残りを `pad` すると、`-` と `$` の
間に埋め文字が入ってしまいます。

数の部分のポイントです。

- `self.cents.abs()` は `i64::MIN` でオーバーフローします（デバッグビルドではパニック）。`i64::MAX` は
  その絶対値より 1 小さいからです。`unsigned_abs()` は `u64` を返し、どの値でも動きます。
- 符号はドル部分ではなく `self.cents` から取ります。`-5 / 100` は `0` になり、符号が残りません。
- 桁は右から区切ります。左から順に見る場合は、先頭以外の位置 `i` で、そこから末尾までの桁数 `len - i` が
  3 の倍数なら、その桁の前に `,` を入れます。
- `{:02}` でセント部分を 0 埋めの 2 桁で出力します。

### 別解

算術で区切る方法です。`% 1000` で 3 桁ずつ取り出し、`005` のような 0 が消えないよう `{:03}` で出力します。
いちばん左のグループだけは 0 埋めせずに出力します。
`f.pad` の代わりに `fmt::Display::fmt(&text, f)` でも動きます。同じフォーマッタで `String` の `Display` を
呼ぶので、余白が付きます。

```rust
fn group_thousands(mut n: u64) -> String {
    let mut groups = Vec::new();
    while n >= 1000 {
        groups.push(format!("{:03}", n % 1000));
        n /= 1000;
    }
    groups.push(n.to_string());
    groups.reverse();
    groups.join(",")
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abs = self.cents.unsigned_abs();
        let sign = if self.cents < 0 { "-" } else { "" };
        let text = format!("{sign}${}.{:02}", group_thousands(abs / 100), abs % 100);
        fmt::Display::fmt(&text, f)
    }
}
```

`f.pad(&text)` の代わりに、`f.width()`・`f.fill()`・`f.align()` を使って自分で余白を付けることもできます。
幅が表示より小さいと `width - len` がアンダーフローするので、`saturating_sub` を使います。

```rust
let len = text.chars().count();
let padding = f.width().unwrap_or(0).saturating_sub(len);
let (left, right) = match f.align() {
    Some(fmt::Alignment::Right) => (padding, 0),
    Some(fmt::Alignment::Center) => (padding / 2, padding - padding / 2),
    Some(fmt::Alignment::Left) | None => (0, padding),
};
let fill = f.fill().to_string();
write!(f, "{}{text}{}", fill.repeat(left), fill.repeat(right))
```
