`Box<dyn Error>` にはどんなエラー型でも入れられます。しかも `?` が自動でボックス化してくれます。
`Error` を実装したすべての型 `E` について、標準ライブラリに `From<E> for Box<dyn Error>` があるからです。
そのため `line.parse()?` は `ParseIntError` そのものを箱に入れて返し、呼び出し側は
`downcast_ref::<ParseIntError>()` で取り出せます。

難しいのはただのメッセージのほうです。`&str` や `String` は `Error` を実装していないので、
`Err(Box::new("overflow"))` はコンパイルできません。`Err("no numbers")` も `Result<_, &str>` になるので
だめです。ただし `Box<dyn Error>` には `From<&str>`（と `From<String>`）もあり、文字列を内部のエラー型で
包んでくれます。そのため `"no numbers".into()` と書けます。`?` も同じ変換を使うので、
`checked_add(n).ok_or("overflow")?` は `None` を `"overflow"` のエラーにして返します。

`checked_add` はオーバーフローする代わりに `None` を返します。ただの `sum += n` は、デバッグビルドでは
オーバーフローでパニックします。解釈と同じループで、足すたびに確かめれば、2 つの規則が自然に満たされます。
途中までの合計を確かめることになり、最初の問題が優先されます（先に全部の行を解釈すると、前にある
オーバーフローより後ろの不正な行を報告してしまいます）。合計が `0` でも整数がなかったとは限らないので、
`seen` が必要です。

ついやりたくなる `line.parse::<i64>().map_err(|e| e.to_string())?` も `From<String>` を通して
コンパイルできますが、箱の中身はメッセージだけになります。表示は正しく見えても、
`downcast_ref::<ParseIntError>()` は `None` を返します。

### 別解

メッセージ用に自前のエラー型を定義する方法です。`Debug` と `Display`、それに（中身が空の）
`impl Error` があれば、`?` や `Box::new` で `Box<dyn Error>` にできます。

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Problem(&'static str);

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl Error for Problem {}

pub fn total(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut sum: i64 = 0;
    let mut seen = false;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let n: i64 = line.parse()?;
        sum = sum.checked_add(n).ok_or(Problem("overflow"))?;
        seen = true;
    }
    if !seen {
        return Err(Box::new(Problem("no numbers")));
    }
    Ok(sum)
}
```

フラグ `seen` の代わりに `Option<i64>` を使うこともできます。`None` は「まだ整数がない」という意味で、
最後に `ok_or_else` で `no numbers` のエラーに変えます。

```rust
use std::error::Error;

pub fn total(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut sum: Option<i64> = None;
    for line in input.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let n: i64 = line.parse()?;
        sum = Some(match sum {
            None => n,
            Some(s) => s.checked_add(n).ok_or("overflow")?,
        });
    }
    sum.ok_or_else(|| "no numbers".into())
}
```
