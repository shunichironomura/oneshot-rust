`std::sync::mpsc` のチャネルを使う小さな関数を 3 つ実装してください。

```rust
use std::sync::mpsc::{Receiver, Sender};

pub fn send_all<T>(tx: &Sender<T>, items: Vec<T>) -> Vec<T>
pub fn drain_ready<T>(rx: &Receiver<T>) -> (Vec<T>, bool)
pub fn fan_in(groups: Vec<Vec<u32>>) -> Vec<u32>
```

- `send_all` は `items` を先頭から順に `tx` で送ります。受信側がもういないために届けられなかった要素を、
  元の順序のまま返します。すべて送れたときは空の `Vec` を返します。
- `drain_ready` は、チャネルにすでに届いているメッセージを**ブロックせずに**すべて受け取り、届いた順に
  返します。`bool` は、受け取り終えた時点でチャネルが切断されている（そのチャネルの `Sender` が、
  クローンも含めてすべて破棄されている）なら `true`、そうでなければ `false` です。
- `fan_in` はチャネルを 1 つ作り、グループごとにスレッドを 1 つ生成します。各スレッドは、チャネルの
  `Sender` を自分用にクローンしたものを使って、自分のグループの数を送ります。`fan_in` はすべての数を
  受け取り、すべてのスレッドを `join` して、数を昇順に並べて返します。重複した数もそのまま残します。
  `groups` が空のことも、グループが空のこともあります。

シグネチャは変えないでください。

### 例

```rust
let (tx, rx) = channel();
drop(rx);
send_all(&tx, vec![1, 2])          // vec![1, 2]：受信側がいない

let (tx, rx) = channel();
tx.send(1).unwrap();
tx.send(2).unwrap();
drain_ready(&rx)                   // (vec![1, 2], false)
drop(tx);
drain_ready(&rx)                   // (vec![], true)

fan_in(vec![vec![3, 1], vec![2]])  // vec![1, 2, 3]
```
