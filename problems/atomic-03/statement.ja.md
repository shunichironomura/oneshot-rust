複数のワーカースレッドが何かを計測し（たとえば、リクエストごとにかかったマイクロ秒数）、
計測値をすべて共有の `Stats` に記録します。`Stats` を**ロックを使わずに**実装してください。

```rust
pub struct Stats { /* フィールドは自由 */ }

impl Stats {
    pub const fn new() -> Stats;
    pub fn record(&self, value: u64);
    pub fn count(&self) -> u64;
    pub fn sum(&self) -> u64;
    pub fn max(&self) -> Option<u64>;
    pub fn min(&self) -> Option<u64>;
}

pub static GLOBAL: Stats = Stats::new();
```

- `new` はサンプルが 1 つもない `Stats` を返します。
- `record(value)` はサンプル `value` を 1 つ追加します。値はどれも `0` 以上 `1_000_000` 以下で、
  `0` もほかの値と同じく 1 つのサンプルです。
- `count` はサンプルの個数、`sum` はその合計、`max` は最大値、`min` は最小値を返します。
  サンプルがないときは、`count` と `sum` は `0`、`max` と `min` は `None` です。
  オーバーフローは考えなくてかまいません。
- `Stats` は（`GLOBAL` も含めて）それぞれ自分のサンプルを持ちます。
- 複数のスレッドが、同じ `Stats` の `record` を（`&Stats` を通して）同時に呼びます。
  タイミングにかかわらず、サンプルが失われてはいけません。
- `count`、`sum`、`max`、`min` は、同じ `Stats` の `record` が実行中のときには呼ばれません。
  それまでに終わったすべての `record` の呼び出しが反映されます。`join` したスレッド
  （たとえば `std::thread::scope` の終わりで `join` されたスレッド）が行った呼び出しも含みます。
- `new` は `const fn` のままにしてください。テストは `GLOBAL` と同じように `Stats` を `static` に置きます。

公開されているシグネチャと `GLOBAL` は変えないでください。`Stats` のフィールド（エディタにあるものは
たたき台です）と `new` の本体は自由に決めてかまいません。`Mutex` や `RwLock` のようなロックは
使わないでください。

### 例

```rust
let stats = Stats::new();
// count() == 0, sum() == 0, max() == None, min() == None
stats.record(5);
stats.record(1);
stats.record(9);
// count() == 3, sum() == 15, max() == Some(9), min() == Some(1)
```

4 つのスレッドが、1 から 100 までの数を 1 つの `Stats` に記録します。

```rust
let stats = Stats::new();
thread::scope(|s| {
    for t in 0..4 {
        let stats = &stats;
        s.spawn(move || {
            for i in 1..=25 {
                stats.record(t * 25 + i);
            }
        });
    }
});
// count() == 100, sum() == 5050, max() == Some(100), min() == Some(1)
```
