複数のワーカースレッドが何かを計測し（たとえば、リクエストごとにかかったミリ秒数や温度）、
計測値をすべて共有の `Stats` に記録します。`Stats` を**ロックを使わずに**実装してください。

```rust
pub struct Stats { /* フィールドは自由 */ }

impl Stats {
    pub const fn new() -> Stats;
    pub fn record(&self, value: f64);
    pub fn count(&self) -> u64;
    pub fn sum(&self) -> f64;
    pub fn max(&self) -> Option<f64>;
    pub fn min(&self) -> Option<f64>;
}

pub static GLOBAL: Stats = Stats::new();
```

- `new` はサンプルが 1 つもない `Stats` を返します。
- `record(value)` はサンプル `value` を 1 つ追加します。値はどれも有限の数（NaN や無限大には
  なりません）で、`-1_000_000.0` 以上 `1_000_000.0` 以下です。負の値も `0.0` も、ほかの値と
  同じく 1 つのサンプルです。
- `count` はサンプルの個数、`sum` はその合計、`max` は最大値、`min` は最小値を返します。
  サンプルがないときは、`count` は `0`、`sum` は `0.0`、`max` と `min` は `None` です。
- テストで使う値は `2.5` や `-0.75` のように、`f64` で合計を誤差なく計算できるものだけです。
  そのため、サンプルを足す順番によって `sum` が変わることはありません。
- `Stats` は（`GLOBAL` も含めて）それぞれ自分のサンプルを持ちます。
- 複数のスレッドが、同じ `Stats` の `record` を（`&Stats` を通して）同時に呼びます。
  タイミングにかかわらず、サンプルが失われてはいけません。
- `count`、`sum`、`max`、`min` は、同じ `Stats` の `record` が実行中のときには呼ばれません。
  それまでに終わったすべての `record` の呼び出しが反映されます。`join` したスレッド
  （たとえば `std::thread::scope` の終わりで `join` されたスレッド）が行った呼び出しも
  含みます。
- `new` は `const fn` のままにしてください。テストは `GLOBAL` と同じように `Stats` を `static` に置きます。

公開されているシグネチャと `GLOBAL` は変えないでください。`Stats` のフィールド（エディタにあるものは
たたき台です）と `new` の本体は自由に決めてかまいません。`Mutex` や `RwLock` のようなロックは
使わないでください。

### 例

```rust
let stats = Stats::new();
// count() == 0, sum() == 0.0, max() == None, min() == None
stats.record(2.5);
stats.record(-1.0);
stats.record(4.0);
// count() == 3, sum() == 5.5, max() == Some(4.0), min() == Some(-1.0)
```

4 つのスレッドが、`0.25`、`0.5`、`0.75`、…、`10.0` の数を 1 つの `Stats` に記録します。

```rust
let stats = Stats::new();
thread::scope(|s| {
    for t in 0..4 {
        let stats = &stats;
        s.spawn(move || {
            for i in 1..=10 {
                stats.record((t * 10 + i) as f64 / 4.0);
            }
        });
    }
});
// count() == 40, sum() == 205.0, max() == Some(10.0), min() == Some(0.25)
```
