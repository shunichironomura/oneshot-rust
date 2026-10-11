どの関数も標準ライブラリを使えば数行で書けますが、コンパイラの助けなしに書くと間違えやすい点が
それぞれ 1 つずつあります。

- `parse_all`：`Result` は `FromIterator` を実装しています。`Result<i32, E>` を返すイテレータを
  `Result<Vec<i32>, E>` に `collect` すると、すべての値の `Ok` か、最初の `Err` になります。`Err` が出た時点で止まり、
  残りの要素は見ません。何を作るかは返り値の型から `collect` に伝わります。`parse` は空白を読み飛ばさないので、
  ルールのとおりの動きになります。`str::parse::<i32>` を `map` に直接渡すとコンパイルできません（E0277）。
  イテレータの要素は `&&str` ですが、この関数が受け取るのは `&str` だからです。クロージャで書けば、メソッド呼び出しが
  レシーバを自動で参照外しします（`map` の前に `.copied()` を挟んでも動きます）。
- `split_parsed`：`&[&'a str]` である `items` を回すと、要素は `&&'a str` になります。`for item in items` と
  `let mut invalid = Vec::new();` で `invalid.push(item)` とすると `invalid` は `Vec<&&str>` になり、返り値の型と
  合いません（E0308）。`for &item in items` のパターンは、スライスから `&'a str` をコピーして取り出します
  （共有参照は `Copy` です）。この参照が借用しているのはスライスではなく文字列で、その期間は `'a` なので、
  スライスがなくなった後も結果を使えます。
- `double_first`：`items.first()` は `Option<&&str>` で、これを `parse` で `map` すると
  `Option<Result<i32, ParseIntError>>` になります。`transpose` は 2 つの層を入れ替えます。`None` は `Ok(None)`、
  `Some(Ok(n))` は `Ok(Some(n))`、`Some(Err(e))` は `Err(e)` になり、`Err` は `?` で返せます。そのあと値を `i64` で
  2 倍します。判定はデバッグビルドと同じくオーバーフローを検査するので、`i32` のまま `2 * n` とすると
  `2147483647` でオーバーフローしてパニックします。`(2 * n) as i64` では変換が遅すぎます。`2 * i64::from(n)` なら
  安全です（`as` は `*` より強く結合するので、`2 * n as i64` も安全です）。`items[0]` で取り出すと、空のスライスで
  パニックします。
- `sum_all`：`Option` は `Sum` を実装していて、`xs.iter().copied().sum::<Option<u32>>()` は `None` の要素があれば
  `None` を返します。しかし足し算は普通の `+` なので、オーバーフローするとパニックします。`try_fold` は、
  クロージャが最初に `None` を返したところで止まります。`x?` は欠けた要素で `None` を返し、`checked_add` は
  オーバーフローで `None` を返します。

### 別解

普通のループでも書けます。ループの中の `?` は、最初に問題が見つかったところで関数から返ります。

```rust
use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    let mut values = Vec::new();
    for item in items {
        values.push(item.parse::<i32>()?);
    }
    Ok(values)
}

pub fn double_first(items: &[&str]) -> Result<Option<i64>, ParseIntError> {
    match items.first() {
        None => Ok(None),
        Some(item) => {
            let n: i32 = item.parse()?;
            Ok(Some(2 * i64::from(n)))
        }
    }
}

pub fn sum_all(xs: &[Option<u32>]) -> Option<u32> {
    let mut total: u32 = 0;
    for &x in xs {
        total = total.checked_add(x?)?;
    }
    Some(total)
}
```

`partition` は、条件によってイテレータを 2 つに分けます。エラーを要素そのものに置き換えておけば
（`map_err(|_| item)`）、どの要素も `Result<i32, &str>` になります。分けた後は、どの `unwrap` と `unwrap_err` も
必ず成功します。

```rust
pub fn split_parsed<'a>(items: &[&'a str]) -> (Vec<i32>, Vec<&'a str>) {
    let (valid, invalid): (Vec<_>, Vec<_>) = items
        .iter()
        .map(|&item| item.parse::<i32>().map_err(|_| item))
        .partition(Result::is_ok);
    (
        valid.into_iter().map(Result::unwrap).collect(),
        invalid.into_iter().map(Result::unwrap_err).collect(),
    )
}
```

`Option` も `FromIterator` を実装しています。`Option<Vec<u32>>` に `collect` すると、欠けた要素があれば `None` に
なります。そのあとの `try_fold` には、`u32::checked_add` をそのまま渡せます。

```rust
pub fn sum_all(xs: &[Option<u32>]) -> Option<u32> {
    let values: Option<Vec<u32>> = xs.iter().copied().collect();
    values?.into_iter().try_fold(0, u32::checked_add)
}
```
