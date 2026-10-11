`send` は値を**ムーブで**受け取るので、`tx.send(item)` の後の `item` はもう使えず、エラーの分岐で `item` を
`push` しようとするとコンパイルエラー（ムーブ済みの値の使用）になります。ただし値はなくなったわけではありません。
受信側がいないと `send` は `Err(SendError(item))` を返し、`.0` で値を取り戻せます。`T` にはトレイト境界が
ないので、クローンには頼れません（テストでは `Clone` も `Copy` も実装していない型を使っています）。
受信側が破棄された後は以降の `send` もすべて失敗し、それらの要素も返す必要があります。最初のエラーで
単に `break` すると、残りの要素が失われます。

`try_recv` はブロックせず、失敗には 2 種類あります。`TryRecvError::Empty`（まだ何もないが、送信側はまだ送るかも
しれない）と `TryRecvError::Disconnected`（もう何もなく、送信側もすべていない）です。最後の送信側が破棄される前に
送られたメッセージは先に受け取れるので、`Disconnected` になるのはキューが空になってからです。
`recv()` や `rx.iter()` を使うと、送信側が生きていて空のチャネルでは永遠にブロックします。

`fan_in` の `rx.iter()`（や `for n in rx`）は、`Sender` が**すべて**破棄されたときにだけ終わります。スレッドに
ムーブしたクローンはスレッドが終わると破棄されますが、元の `tx` は `fan_in` 自身の中で生きています。`drop(tx)` が
ないと、ループは決して来ないメッセージを待ち続け、Miri はデッドロックを報告します。スレッドを 1 つも生成しない
空の `groups` でも同じです。スレッドが終わる順番は決まっていないので、結果はソートする必要があります。

### 別解

`send_all` をイテレータのチェーン 1 つで書く方法です。`Result::err` で失敗だけを残し、`SendError(item)` という
パターンで返ってきた値を取り出します。

```rust
use std::sync::mpsc::SendError;

pub fn send_all<T>(tx: &Sender<T>, items: Vec<T>) -> Vec<T> {
    items
        .into_iter()
        .filter_map(|item| tx.send(item).err())
        .map(|SendError(item)| item)
        .collect()
}
```

`drain_ready` は、フラグをループから返すこともできます。値付きの `break` はループを終わらせ、その値を `let` に
渡します。`TryRecvError` は `PartialEq` を実装しているので、`==` で比較できます。

```rust
pub fn drain_ready<T>(rx: &Receiver<T>) -> (Vec<T>, bool) {
    let mut ready = Vec::new();
    let disconnected = loop {
        match rx.try_recv() {
            Ok(value) => ready.push(value),
            Err(e) => break e == TryRecvError::Disconnected,
        }
    };
    (ready, disconnected)
}
```

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
