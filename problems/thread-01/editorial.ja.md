`thread::spawn` に渡すクロージャは `'static` でなければなりません。生成したスレッドは関数より長生きするかもしれないので、
ローカルなものを借用できないからです。各 `chunk` は `values` を借用していますが、`values` が生きている保証があるのは
`parallel_sum` から戻るまでです。そのため、戻る前にすべてのスレッドを `join` していても、コンパイラはこのコードを拒否します。

この問題をそのまま解決するのが `thread::scope` です。スコープ `s` で生成したスレッドは `scope` から戻る前に
すべて `join` されるので、クロージャから `values` を借用できます。`scope` に渡したクロージャの返り値が、
そのまま `scope` の返り値になります。

### 別解

各塊を `Vec` にコピーして所有させれば、スレッドは何も借用しなくなり、`thread::spawn` のままで動きます。
ただし入力をコピーする分のコストがかかります。

```rust
use std::thread;

pub fn parallel_sum(values: &[u64], workers: usize) -> u64 {
    let chunk_size = values.len().div_ceil(workers).max(1);
    let handles: Vec<_> = values
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            thread::spawn(move || chunk.iter().sum::<u64>())
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}
```

`values` は借用したスライスなので、`Arc` で包むだけでは解決しません。先に `Arc<[u64]>` へコピーする必要があります。
