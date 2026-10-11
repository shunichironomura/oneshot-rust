`record` は `&self` を受け取り、複数のスレッドが同時に `&Stats` を持っているので、エディタにある
普通のフィールドは更新できません。共有参照越しに `self.count += 1` と書いてもコンパイルが通りません。
`Cell` は `Sync` ではないので `static GLOBAL: Stats` がコンパイルできなくなりますし、ロックは
使えません。残るのはアトミック型です。個数は簡単で、`AtomicU64` と `fetch_add` で済みます。

**落とし穴は 3 つの `f64` のフィールドです。標準ライブラリには浮動小数点数のアトミック型がありません。**
`AtomicF64` はありませんが、`f64` は 64 ビットなので、`f64::to_bits` と `f64::from_bits` を使えば、
ビットを 1 つも変えずに `u64` との間で変換できます。そこで、各 `f64` はビットパターンとして
`AtomicU64` に入れます。どちらの変換も `const fn` なので、`new` は `const fn` のままにできます。

ただし、アトミック型が知っているのは整数だけなので、その演算は、ビットが表す数にとっては
間違った結果になります。

- `sum.fetch_add(value.to_bits(), ..)` は 2 つのビットパターンを整数として足します。`2.5` のビットと
  `4.0` のビットを足しても `6.5` のビットにはならず、近い数にさえなりません。
- 0 以上の数どうしなら、ビットパターンの大小はたまたま数の大小と一致するので、
  `max.fetch_max(value.to_bits(), ..)` は簡単なテストには通ります。しかし符号は最上位のビットなので、
  負の数のビットはどれも正の数のビットより大きく、負の数どうしでは 0 から遠いほうのビットが大きく
  なります。`-1.25` と `-8.0` では、この「最大値」は `-8.0` になってしまいます。

そのため、新しい値は `f64` として計算し、1 回のアトミックな操作で書き戻す必要があります。
つまり compare-and-swap のループです。`AtomicU64::update`（Rust 1.95 で安定化）がこのループを
回してくれます。ビットを読み込んでクロージャを呼び、アトミック変数がまだ読み込んだときのビットを
持っている場合だけ結果を書き込みます。間にほかのスレッドが値を変えていたら、新しい値で
クロージャを呼び直します。模範解答では、両方向の変換をする `update_f64` でこれを包み、合計
（`sum + value`）、最大値（`max.max(value)`）、最小値（`min.min(value)`）に使っています。

詰まりやすい点：

- `let s = f64::from_bits(sum.load(..)); sum.store((s + value).to_bits(), ..)` は、どちらの操作も
  アトミックなのでデータ競合ではありませんが、競合状態（レースコンディション）です。読み込みと
  書き込みの間にほかのスレッドが記録すると、そのサンプルが失われます。読み込んで比べてから
  書き込む最大値も同じです。`SeqCst` にしても解決しません。順序付けで 2 つの操作を 1 つにすることは
  できないからです。
- アトミック型は `None` を持てないので、最大値と最小値には初期値が必要です。最大値は `-inf` から、
  最小値は `+inf` から始めるので、最初のサンプルで置き換わります。`0.0` から始めた最大値は、
  サンプルがすべて負なら `0.0` のままです。`max` と `min` は、個数が `0` の間は `None` を返します。
  合計（や最大値）が `0.0` でも「サンプルなし」とは限りません。`0.0` も正しいサンプルです。
- `f64` の足し算は一般には結合法則を満たしません。任意の値では、スレッドがたまたま足した順番によって
  合計が変わってしまいます。問題文が、合計を誤差なく計算できる値だけを使っているのはこのためです。

順序付けはすべて `Ordering::Relaxed` で十分です。1 つのアトミック変数への変更はすべて 1 つの順序
（変更順序）に並び、成功した compare-and-swap は、その順序で自分が読んだ値の直後に書き込みます。
そのため更新が失われることはありません。結果を読むのは記録したスレッドを `join` した後で、
スレッドを `join` すると、そのスレッドが行ったことはすべて `join` したスレッドから見えるように
なります。

4 つのフィールドは 1 つずつ順に更新されるので、`record` と同時に読むと、新しい個数と古い合計が
見えることがあります。問題文の条件でそれは起きません。記録が続いている間も一貫した値を読みたいなら、
4 つの数をまとめて 1 つの `Mutex` に入れます。

### 別解

`update` が回しているループを手で書いたものです。`compare_exchange_weak` は、値が変わっていなくても
失敗することがあります（見せかけの失敗）が、ループがもう 1 周するだけです。

```rust
fn update_f64(cell: &AtomicU64, f: impl Fn(f64) -> f64) {
    let mut current = cell.load(Ordering::Relaxed);
    loop {
        let new = f(f64::from_bits(current)).to_bits();
        match cell.compare_exchange_weak(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            // Another thread changed the value in between (or the exchange failed spuriously):
            // try again with the value it holds now.
            Err(actual) => current = actual,
        }
    }
}
```

整数としての大小が数の大小と一致するキーに変換すれば、`fetch_max` と `fetch_min` を使えます。
0 以上の数は符号ビットを反転してすべての負の数より上に移し、負の数はすべてのビットを反転します。
これで負の数どうしの順序も逆になります（`f64::total_cmp` も似た方法で比較しています）。
キー `0` はどの数のキーよりも小さく、`u64::MAX` はどの数のキーよりも大きいので、これらを
「サンプルなし」の意味に使えます。合計にはやはり compare-and-swap のループが必要で、ここでは
`update` を使っています。

```rust
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Stats {
    count: AtomicU64,
    sum: AtomicU64,
    /// The key of the largest sample, or `0` while there are no samples.
    max_key: AtomicU64,
    /// The key of the smallest sample, or `u64::MAX` while there are no samples.
    min_key: AtomicU64,
}

const SIGN: u64 = 1 << 63;

/// Maps a number (not NaN) to a `u64` that is ordered like the numbers.
fn key(x: f64) -> u64 {
    let bits = x.to_bits();
    if bits & SIGN == 0 { bits | SIGN } else { !bits }
}

/// The inverse of `key`.
fn from_key(key: u64) -> f64 {
    f64::from_bits(if key & SIGN != 0 { key & !SIGN } else { !key })
}

impl Stats {
    pub const fn new() -> Stats {
        Stats {
            count: AtomicU64::new(0),
            sum: AtomicU64::new(0),
            max_key: AtomicU64::new(0),
            min_key: AtomicU64::new(u64::MAX),
        }
    }

    pub fn record(&self, value: f64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum.update(Ordering::Relaxed, Ordering::Relaxed, |bits| {
            (f64::from_bits(bits) + value).to_bits()
        });
        self.max_key.fetch_max(key(value), Ordering::Relaxed);
        self.min_key.fetch_min(key(value), Ordering::Relaxed);
    }

    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    pub fn sum(&self) -> f64 {
        f64::from_bits(self.sum.load(Ordering::Relaxed))
    }

    pub fn max(&self) -> Option<f64> {
        let max_key = self.max_key.load(Ordering::Relaxed);
        (max_key != 0).then(|| from_key(max_key))
    }

    pub fn min(&self) -> Option<f64> {
        let min_key = self.min_key.load(Ordering::Relaxed);
        (min_key != u64::MAX).then(|| from_key(min_key))
    }
}

pub static GLOBAL: Stats = Stats::new();
```
