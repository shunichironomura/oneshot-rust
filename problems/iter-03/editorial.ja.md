どの関数も、合ったアダプタを使えば短く書けますが、正しそうに見えるだけのアダプタを使うと
少しだけ間違えます。

`max_by_key` は、最大の要素が複数あるとき**最後の**要素を返します（`min_by_key` は最初の
要素を返します）。そのため `words.iter().max_by_key(|w| w.chars().count())` は、
`["hi", "hello", "world"]` に対して `"world"` を返してしまいます。先にイテレータを逆順にすれば、
「最後」が「最初」になります。文字数は `chars().count()` で数えます。`len()` はバイト数で、
`"éé"` は 4 バイトですが 2 文字です。

`take_while` は、どこで止まるかを知るために、条件を満たさない最初の行まで見る必要があります。
その行は元のイテレータからはもう取り出されています。次のコードでは `"body"` が失われます。

```rust
let mut it = lines.iter().copied();
let header: Vec<_> = it.by_ref().take_while(|l| l.starts_with('#')).collect();
let body: Vec<_> = it.collect(); // ["# x"]
```

代わりに `position` で境目を見つけ、`split_at` でスライスを切り分けます。（`filter` や
`partition` だと `"# x"` がヘッダに入ってしまいます。）

`scan` は可変の状態を持ち回る `map` です。クロージャは `Option` を返し、`None` を返すと
反復が終わります。これがちょうど「適用できない最初の取引で止まる」を表します。`filter` だと
その取引だけを飛ばして続けてしまい、`[70, 120, 130]` になります。

`iter::successors` は、各要素を 1 つ前の要素から計算する列を作り、クロージャが `None` を
返すと終わります。`checked_mul` はかけ算がオーバーフローするときにちょうど `None` を返すので、
収まる最大の累乗の後で列が終わります。`p * base` と書くと、ジャッジのようなデバッグビルドでは
オーバーフローでパニック（"attempt to multiply with overflow"）し、リリースビルドでは黙って値が
一周します。初期値は `Some(1u64)` と書きます。`Some(1)` だと、クロージャが `checked_mul` を
呼ぶ時点で `p` の型がまだ決まっておらず、コンパイルエラーになります。浮動小数点数の対数に
逃げるのもうまくいきません。`u64::MAX as f64` はちょうど 2⁶⁴ に切り上げられるので、`2` や `16`
のような底では、個数が丸め誤差しだいで変わってしまいます。

### 別解

`min_by_key` は最初の最小値を返すので、`std::cmp::Reverse` を使って最大の文字数を最小の
キーにする方法もあります。

```rust
use std::cmp::Reverse;

pub fn longest<'a>(words: &[&'a str]) -> Option<&'a str> {
    words.iter().copied().min_by_key(|w| Reverse(w.chars().count()))
}
```

`Peekable::next_if` は、次の要素が条件を満たすときだけそれを返し、満たさないときはそのまま残すので、
何も失われません。

```rust
pub fn split_header<'a>(lines: &[&'a str]) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut it = lines.iter().copied().peekable();
    let mut header = Vec::new();
    while let Some(line) = it.next_if(|line| line.starts_with('#')) {
        header.push(line);
    }
    (header, it.collect())
}
```

途中の和を計算して、最初に負になるところまでを残す方法もあります。状態には適用できない取引も
足されますが、`take_while` はその和を捨て、それ以降は `scan` に次の要素を求めないので、
後の取引が足されることはありません。

```rust
pub fn running_balance(start: i64, txns: &[i64]) -> Vec<i64> {
    txns.iter()
        .scan(start, |balance, &t| {
            *balance += t;
            Some(*balance)
        })
        .take_while(|&b| b >= 0)
        .collect()
}
```

`checked_pow` と `map_while`（最初の `None` で止まる `map`）でも書けます。指数の `0..` は、
`checked_pow` が受け取る型の `u32` になります。

```rust
pub fn powers_of(base: u64) -> Vec<u64> {
    (0..).map_while(|e| base.checked_pow(e)).collect()
}
```

正確な整数の対数 `ilog` で、先に最大の指数を求める方法もあります。

```rust
pub fn powers_of(base: u64) -> Vec<u64> {
    (0..=u64::MAX.ilog(base)).map(|e| base.pow(e)).collect()
}
```
