エディタにある 2 つの関数の本体はすでに書かれていますが、シグネチャが間違っているので、
コンパイルが通りません。**シグネチャだけ**（`pub fn` の行と `where` 節）を書き換えて、
ハーネス（`tests.rs` タブ）がコンパイルでき、テストに合格するようにしてください。
本体は変更しないでください。

- `par_map(items, f)` は、`items` の要素ごとに `f` をちょうど 1 回、それぞれ別のスコープ付き
  スレッドで呼び出し、結果を `items` と同じ順に並べて返します。`items` が空なら空の `Vec` を返します。
- `spawn_job(input, f)` は、`f(input)` を実行する新しいスレッドを起動し、そのスレッドを待たずに
  `JoinHandle` をすぐに返します。このハンドルを `join` すると `f(input)` の結果が得られます。

ハーネスは、さまざまなクロージャと入力を渡して 2 つの関数を呼び出します。
そのすべての呼び出しがコンパイルできなければなりません。

### 例

```rust
let calls = AtomicUsize::new(0);
let prefix = String::from("item-");
let names = vec![String::from("a"), String::from("bb"), String::from("ccc")];
par_map(names, |name| {
    calls.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}{name}")
})
// ["item-a", "item-bb", "item-ccc"]。calls は 3 になる

let name = String::from("Ferris");
spawn_job(3, move |n: usize| name + &"!".repeat(n)).join().unwrap()
// "Ferris!!!"
```
