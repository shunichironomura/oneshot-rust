`rx.iter()`（や `for n in rx`）は、`Sender` が**すべて**破棄されたときにだけ終わります。スレッドに
ムーブしたクローンはスレッドが終わると破棄されますが、元の `tx` は `fan_in` 自身の中で生きています。`drop(tx)` が
ないと、ループは決して来ないメッセージを待ち続け、Miri はデッドロックを報告します。スレッドを 1 つも生成しない
空の `groups` でも同じです。スレッドが終わる順番は決まっていないので、結果はソートする必要があります。

### 別解

`thread::scope` を使うと、スコープの終わりですべてのスレッドが `join` されます。その時点ですべての数は
チャネルに入っている（容量に上限のないチャネルなので可能です）ので、`tx` を破棄すれば待たずに集められます。

```rust
pub fn fan_in(groups: Vec<Vec<u32>>) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();
    thread::scope(|s| {
        for group in groups {
            let tx = tx.clone();
            s.spawn(move || {
                for n in group {
                    tx.send(n).unwrap();
                }
            });
        }
    });
    drop(tx);
    let mut all: Vec<u32> = rx.iter().collect();
    all.sort();
    all
}
```

先に数の個数を数えておき、`rx.iter().take(total)` でちょうどその個数だけ受け取る方法もあります。これなら
送信側の破棄を待ちません。この問題では動きますが、総数を前もって知っている必要があります。受信ループを
終わらせる一般的な方法は、`tx` を破棄することです。
