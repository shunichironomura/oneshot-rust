`async fn` の Future は、`.await` をまたいで生きているものをすべて保持する値で、`await` 中の Future も
そこに含まれます。ここではそれが `depth` 自身の Future なので、型が自分自身を含み、サイズが無限大に
なってしまいます（E0733）。`Box::pin(depth(c))` で内側の Future をヒープに移せば、外側の Future は
ポインタを持つだけになります。Rust 1.77 からは、再帰する `async fn` にはこれだけで十分です。

- `Box::new` では足りません。`Box<F>` を `await` できるのは `F: Unpin` のときだけで、`async fn` の
  Future は `Unpin` ではないからです。
- `std::pin::pin!` はスタック上でピン留めするだけなので、Future のサイズは無限大のままです。

### 別解

Rust 1.77 より前は、再帰する async 関数は、ボックス化した Future を明示的に返す必要がありました。
公開する `async fn` はそのままにして、そのような非公開のヘルパーを呼ぶ形にします。ボックス化した
`dyn Future` は既定で `'static` になりますが、この Future は `t` を借用するので `+ '_` が必要です。

```rust
use std::pin::Pin;

pub async fn depth(t: &Tree) -> usize {
    depth_boxed(t).await
}

fn depth_boxed(t: &Tree) -> Pin<Box<dyn Future<Output = usize> + '_>> {
    Box::pin(async move {
        let mut d = 0;
        for c in &t.children {
            d = d.max(depth_boxed(c).await);
        }
        d + 1
    })
}
```
