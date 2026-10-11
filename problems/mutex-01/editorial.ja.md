一時値は、その*一時スコープ*（temporary scope）の終わりまで生きます。`match` のスクルーティー
（マッチ対象の式）は一時スコープではないので、その中の一時値は `match` 全体が終わるまで破棄されません。
テンプレートでは、`self.cache.lock()` が返すガードがこの一時値です。`.copied()` で値はコピーして
取り出していますが、ガード自体はすべてのアームで生きたままなので、`None` のアームで
`f(self, key)` を呼ぶときもキャッシュはロックされたままです。`fib` はすぐにまた `get_or_compute` を
呼び、その `lock()` は、同じスレッドがすでに持っているロックが空くのを永遠に待ちます。
`std::sync::Mutex` は再入可能ではありません（再帰がなくても、同じアームの `insert` で 2 回目の
ロックを取ろうとします）。

`while let` も同じ規則に従います。条件の中の一時値は、ループ本体の終わりまで生きます。`run_all` では
キーを処理している間もキューがロックされたままなので、`self.schedule(key / 2)` がデッドロックします。
`f` が `schedule` を呼んだ場合も同じです。

直し方は、ガードをそれぞれ独立した文の中で破棄させることです。
`let cached = self.cache.lock().unwrap().get(&key).copied();` では、ガードは `;` で破棄され、
`cached` はただの `Option<u64>` になります。こうすれば `f` はロックを持たない状態で実行されます。
`f` はメモを呼び返すことがあるので、どのみちそうする必要があります。値を挿入するために後でもう一度
ロックを取るのは問題ありません。`run_all` では `let ... else` の文でキーを 1 つ取り出せば、キューを
ロックするのは `pop` の間だけになります。

正しそうに見えて、うまくいかない直し方もあります。

- `*cache.entry(key).or_insert_with(|| f(self, key))` は Rust らしい書き方ですが、ガードが生きている
  間に `f` を呼んでいます。名前を付けたガード（`let mut cache = self.cache.lock().unwrap();`）を
  `f` の呼び出しをまたいで持ち続けるのも同じです。
- `run_all` で、キューのガードを変数に入れたまま `key / 2` をそのガード経由で追加すれば、2 回目の
  `lock()` はなくなり、例にも通ります。しかし `f` が `schedule` を呼ぶと、やはりデッドロックします。
  最初にキュー全体を（`std::mem::take` で）取り出してローカルなコピーで処理すると、`f` が追加した
  キーを取りこぼします。

検索と挿入の間でロックを手放すので、2 つのスレッドが同じキーを同時に計算することがあります。
他人のコードを実行する間はロックを持たない、という方針の代償で、問題文でもそれを許しています。

### 別解

Rust 2024 では、`if let` の条件の中の一時値は `else` ブロックの前に破棄されるので、ガードを条件の中に
置いたままでも動きます。Rust 2021 では、ガードが `if let ... else` 全体の終わりまで生きていたので、
この書き方でもデッドロックしていました。

```rust
pub fn get_or_compute(&self, key: u64, f: impl FnOnce(&Memo, u64) -> u64) -> u64 {
    if let Some(&value) = self.cache.lock().unwrap().get(&key) {
        value
    } else {
        let value = f(self, key);
        self.cache.lock().unwrap().insert(key, value);
        self.misses.fetch_add(1, Ordering::Relaxed);
        value
    }
}
```

小さな非公開のヘルパーを作れば、ガードはその関数の中だけで生きるので、`while let` のループは
そのままにできます。

```rust
fn pop(&self) -> Option<u64> {
    self.queue.lock().unwrap().pop()
}

pub fn run_all(&self, f: impl Fn(&Memo, u64) -> u64) -> Vec<(u64, u64)> {
    let mut done = Vec::new();
    while let Some(key) = self.pop() {
        let value = self.get_or_compute(key, &f);
        done.push((key, value));
        if key > 1 {
            self.schedule(key / 2);
        }
    }
    done
}
```
