ジャッジはデバッグビルドと同じく、オーバーフロー検査を有効にしてコードをビルドします。演算の結果が型に
収まらないと、リリースビルドのように黙って一周するのではなく、パニック（`attempt to add with overflow`）します。
そのため、素直な式は端の値で失敗します。`xs.iter().sum::<u8>()`、`(level + amount).min(100)`、
`(items + per_page - 1) / per_page`、`(a + b) / 2`、`(a - b).abs() as u64` は、どれも許された入力の
どこかでパニックします。整数型にはそれぞれの場面に合ったメソッドがあり、どの関数も 1 行で書けます。

- `wrapping_add` は一周して戻ります（`u8` なら 256 で割った余り）。`checked_add` はオーバーフローすると
  `None` を返し、`try_fold` は最初の `None` で止まります。`saturating_add` は `u8::MAX` で止まるので、
  その後で `.min(100)` が本来の上限をかけます。
- `/` は 0 の方向に丸め、`%` はそれに対応する余りを返すので、余りの符号は割られる数と同じになります。
  `-1 % 5` は `-1` で、これを `as usize` すると巨大な数になります。`rem_euclid` は余りが負にならないように
  選ぶので、割る数が正なら `0..len` の範囲の余りになります。
- `div_ceil` は切り上げの割り算（安定版では符号なし整数だけ）、`midpoint` はオーバーフローせずに
  2 つの数の平均を求め（符号なし整数では切り捨て）、`abs_diff` は差の大きさを符号なしの型で返します。
  `i64::MAX - i64::MIN` は `i64` に収まらないので、符号なしの型が必要です。

惜しいけれどパニックする書き方が 2 つあります。`(a - b).unsigned_abs()` では、`unsigned_abs` に
たどり着く前に引き算がオーバーフローします。`xs.iter().map(|&x| Some(x)).sum::<Option<u8>>()` が
`None` を返すのは要素に `None` があるときだけで、足し算自体は普通の足し算なので、オーバーフローすると
パニックします。

### 別解

`sum_checked` はループでも書けます。`checked_add` が `None` を返したら、`?` がすぐに `None` を返します。
`std::num::Wrapping` を使うと、`+`（と `sum`）が一周するようになります。

```rust
use std::num::Wrapping;

pub fn sum_wrapping(xs: &[u8]) -> u8 {
    xs.iter().map(|&x| Wrapping(x)).sum::<Wrapping<u8>>().0
}

pub fn sum_checked(xs: &[u8]) -> Option<u8> {
    let mut total: u8 = 0;
    for &x in xs {
        total = total.checked_add(x)?;
    }
    Some(total)
}
```

専用のメソッドを使わない場合は、より広い型で計算するか、途中の値がオーバーフローしないように式を
組み替えます。ただし、広い型は十分に広くなければなりません。長いスライスのバイトを足すと、`u16` でも
オーバーフローします。

```rust
pub fn charge(level: u8, amount: u8) -> u8 {
    (u16::from(level) + u16::from(amount)).min(100) as u8
}

pub fn wrap_index(i: i64, len: usize) -> usize {
    let n = len as i64;
    // `i % n` は `-n` より大きく `n` より小さいので、`n` を足してもオーバーフローしない
    ((i % n + n) % n) as usize
}

pub fn page_count(items: u64, per_page: u64) -> u64 {
    items / per_page + u64::from(items % per_page != 0)
}

pub fn middle(a: u64, b: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) / 2) as u64
}

pub fn distance(a: i64, b: i64) -> u64 {
    (i128::from(a) - i128::from(b)).unsigned_abs() as u64
}
```

`middle` は、広い型を使わずに `a / 2 + b / 2 + (a & b & 1)` とも書けます。先にそれぞれを半分にすると、
奇数からは 0.5 ずつ失われます。最後の項は、両方が奇数のときに足りない 1 を足します。
