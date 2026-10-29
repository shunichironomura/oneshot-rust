`send` は値を**ムーブで**受け取るので、`tx.send(item)` の後の `item` はもう使えず、エラーの分岐で `item` を
`push` しようとするとコンパイルエラー（ムーブ済みの値の使用）になります。ただし値はなくなったわけではありません。
受信側がいないと `send` は `Err(SendError(item))` を返し、`.0` で値を取り戻せます。`T` にはトレイト境界が
ないので、クローンには頼れません。受信側が破棄された後は以降の `send` もすべて失敗し、それらの要素も返す必要が
あります。最初のエラーで単に `break` すると、残りの要素が失われます。

### 別解

イテレータのチェーン 1 つで書く方法です。`Result::err` で失敗だけを残し、`SendError(item)` という
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
