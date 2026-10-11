3 つの関数は、それぞれ async Rust の別々の規則に違反しています。

**`greet`**：`async` ブロックはクロージャと同じように変数をキャプチャし、参照で足りるなら参照で
キャプチャします。フォーマットには `&name` しか必要ないので、返される Future はローカル変数
`name` を借用しますが、`name` は `greet` から戻るときに破棄されます（E0597）。`async move` にすれば、
`name` は Future の中にムーブされます。`async fn` ではこの問題は起きません。引数は必ず Future の中に
ムーブされるからです。ブロックの中で `name.clone()` と書いても解決しません。クローンが実行されるのは
Future がポーリングされたときなので、ブロックは結局 `name` を借用します。

**`sum_all`**：`.await` が使えるのは、`async fn`、`async` ブロック、非同期クロージャの本体の中に
直接書いたときだけです。クロージャ `|f| f.await` は、async 関数の中に書かれていても普通の
クロージャです（E0728）。`map` や `sum` はこのクロージャを同期的に呼ぶので、途中で中断する
場所がありません。非同期クロージャ（`async |f| f.await`）にしても解決しません。`map` が
Future を返すようになり、`sum` ではそれを足し合わせられないからです。`for` ループには
クロージャがないので、その本体は async 関数の一部です。また、`for` ループは Future を 1 つずつ
順番に `await` します。（`futs` を可変にして）`while let Some(f) = futs.pop()` で取り出す方法も
コンパイルは通りますが、逆順に `await` してしまいます。

**`depth`**：`async fn` の Future は、`.await` をまたいで生きているものをすべて保持する値で、
`await` 中の Future もそこに含まれます。ここではそれが `depth` 自身の Future なので、型が自分自身を
含み、サイズが無限大になってしまいます（E0733）。`Box::pin(depth(c))` で内側の Future をヒープに
移せば、外側の Future はポインタを持つだけになります。Rust 1.77 からは、再帰する `async fn` には
これだけで十分です。`Box::new` では足りません。`Box<F>` を `await` できるのは `F: Unpin` の
ときだけで、`async fn` の Future は `Unpin` ではないからです。`std::pin::pin!` はスタック上で
ピン留めするだけなので、Future のサイズは無限大のままです。

### 別解

`greet` では、先に文字列を作り、すでに完了している Future を返すこともできます。結果は同じですが、
フォーマットは最初のポーリング時ではなく、`greet` を呼んだときに行われます。

```rust
pub fn greet(name: String) -> impl Future<Output = String> {
    std::future::ready(format!("Hello, {name}!"))
}
```

Rust 1.77 より前は、再帰する非同期関数は、ボックス化した Future を明示的に返す必要がありました。
公開する `async fn` はそのままにして、そのような非公開のヘルパーを呼ぶ形にします。ボックス化した
`dyn Future` は既定で `'static` になりますが、この Future は `t` を借用するので `+ '_` が必要です。

```rust
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
