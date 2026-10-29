`std::sync::mpsc` のチャネルで送信すると、受信側がもういないときは失敗します。いくつかの要素を送り、
届けられなかった要素を返す関数 `send_all` を実装してください。

```rust
use std::sync::mpsc::Sender;

pub fn send_all<T>(tx: &Sender<T>, items: Vec<T>) -> Vec<T>
```

- `send_all` は `items` を先頭から順に `tx` で送ります。
- 受信側がもういないために届けられなかった要素を、元の順序のまま返します。すべて送れたときは空の `Vec` を返します。
- `T` はどんな型でもよく、`Clone` も `Copy` も実装していない型のこともあります。

シグネチャは変えないでください。

### 例

```rust
let (tx, rx) = channel();
drop(rx);
send_all(&tx, vec![1, 2])          // vec![1, 2]：受信側がいない
```
