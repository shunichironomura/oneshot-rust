`record` は `&self` を受け取り、複数のスレッドが同時に `&Stats` を持っているので、エディタにある
普通のフィールドは更新できません。共有参照越しに `self.count += 1` と書いてもコンパイルが通りません。
`&mut self` に変えるのも逃げ道になりません。`static` は、複数のスレッドが同時に使う値と同じく、
共有参照を通してしか触れないからです。フィールドには、スレッドをまたいでも安全な内部可変性が必要です。
`Cell<u64>` は `Sync` ではないので、`static GLOBAL: Stats` がコンパイルできなくなります。
ロックを使わないなら、残るのはアトミック型です。`AtomicU64` は `&self` 越しに変更でき、`Sync` で、
`AtomicU64::new` は `const fn` なので、`new` は引き続き `static` で使えます
（`AtomicU64::default()` はそこではコンパイルできません。`Default::default` は `const fn` の中で
呼べないからです）。

`record` の中の更新は、どれも 1 つの read-modify-write 操作にします。個数と合計には `fetch_add`、
最大値と最小値には `fetch_max` と `fetch_min` を使います。どれも値を読み、組み合わせ、書き込むまでを
分割できない 1 つの操作として行うので、ほかのスレッドの変更が間に割り込んで上書きされることはありません。

アトミック型は `None` を持てないので、最大値と最小値には初期値が必要です。`min` はどのサンプルよりも
大きい `u64::MAX` から始めるので、最初の `fetch_min` で置き換わります（`0` から始めた最小値は
`0` のままです）。`max` は `0` から始めますが、`0` も正しいサンプルなので、`0` を「サンプルなし」の
意味には使えません。`max` と `min` は代わりに個数を見て判断します。

順序付けはすべて `Ordering::Relaxed` で十分です。1 つのアトミック変数への変更はすべて 1 つの順序
（変更順序）に並び、read-modify-write 操作は、順序付けが何であっても、その順序で自分の変更の
直前にある値を読みます。そのため更新が失われることはありません。結果を読むのは記録したスレッドを
`join` した後で、スレッドを `join` すると、そのスレッドが行ったことはすべて `join` したスレッドから
見えるようになります。より強い順序付けが必要になるのは、スピンロックのフラグのように、
あるアトミック変数を使って**ほかの**データの準備ができたことを別のスレッドに伝えるときです。

詰まりやすい点：

- `if value > max.load(..) { max.store(value, ..) }` は、どちらの操作もアトミックなのでデータ競合では
  ありませんが、競合状態（レースコンディション）です。読み込みと書き込みの間にほかのスレッドが
  もっと大きい値を書き込むと、この書き込みがそれを小さい値で置き換えてしまいます。
  `let c = count.load(..); count.store(c + 1, ..)` も同じように加算を失います。`SeqCst` にしても
  解決しません。順序付けで 2 つの操作を 1 つにすることはできないからです。
- 「`max` が `0`」や「`sum` が `0`」を「サンプルなし」とみなすと、サンプルがすべて `0` のときに間違えます。
- 4 つのフィールドは 1 つずつ順に更新されるので、`record` と同時に読むと、新しい個数と古い合計が
  見えることがあります。問題文の条件でそれは起きません。記録が続いている間も一貫した値を読みたいなら、
  4 つの数をまとめて 1 つの `Mutex` に入れます。

### 別解

最大値に 1 を足した値を記録すれば、`0` を「サンプルなし」の意味に使えます。最小値は `u64::MAX` を
「サンプルなし」とします（値は `1_000_000` 以下なので、`u64::MAX` がサンプルになることはありません）。
こうすると `max` と `min` は個数を見なくて済みます。`checked_sub(1)` は `0` を `None` にします。

```rust
pub struct Stats {
    count: AtomicU64,
    sum: AtomicU64,
    /// The largest sample plus one, or `0` while there are no samples.
    max_plus_one: AtomicU64,
    /// The smallest sample, or `u64::MAX` while there are no samples.
    min: AtomicU64,
}

impl Stats {
    pub const fn new() -> Stats {
        Stats {
            count: AtomicU64::new(0),
            sum: AtomicU64::new(0),
            max_plus_one: AtomicU64::new(0),
            min: AtomicU64::new(u64::MAX),
        }
    }

    pub fn record(&self, value: u64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum.fetch_add(value, Ordering::Relaxed);
        self.max_plus_one.fetch_max(value + 1, Ordering::Relaxed);
        self.min.fetch_min(value, Ordering::Relaxed);
    }

    // count and sum as in the solution

    pub fn max(&self) -> Option<u64> {
        self.max_plus_one.load(Ordering::Relaxed).checked_sub(1)
    }

    pub fn min(&self) -> Option<u64> {
        let min = self.min.load(Ordering::Relaxed);
        (min != u64::MAX).then_some(min)
    }
}
```

`fetch_max` を使わないなら、compare-and-swap ループを書きます。読んでから書くまでの間に
ほかのスレッドが値を変えていない状態で書けるまで（または、入っている値がもう小さくなくなるまで）
やり直します。`record` では `raise_to(&self.max, value)` を呼びます。最小値用の `lower_to` は
`<` を使う以外は同じです。

```rust
fn raise_to(max: &AtomicU64, value: u64) {
    let mut current = max.load(Ordering::Relaxed);
    while value > current {
        match max.compare_exchange_weak(current, value, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            // Another thread changed the value in between (or the exchange failed spuriously):
            // try again with the value it holds now.
            Err(actual) => current = actual,
        }
    }
}
```

`update`（Rust 1.95 で安定化）は、クロージャの周りでこのようなループを回します。専用の
`fetch_*` メソッドがない更新にも使えます。

```rust
self.max.update(Ordering::Relaxed, Ordering::Relaxed, |max| max.max(value));
self.min.update(Ordering::Relaxed, Ordering::Relaxed, |min| min.min(value));
```
