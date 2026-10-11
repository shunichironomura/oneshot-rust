トレイト `Store` は非同期のキー・バリューストアを表し、非同期関数 `total` は任意のストアから値を
集めて合計します。エディタのコードは単体ではコンパイルできますが、テストハーネス
（`tests.rs` タブ）と組み合わせるとコンパイルできません。ハーネスは `total` の Future を
あるスレッドで作り、マルチスレッドのエグゼキュータがするように、別のスレッドで完了まで
実行します。

**トレイト `Store` だけ**を書き換えて、ハーネスがコンパイルできるようにしてください。それ以外
（`MemStore`、`Fallback`、それぞれの `impl Store` ブロック、`total`）は一切変えないでください。
`use` 宣言は追加してかまいません。

```rust
pub trait Store {
    async fn get(&self, key: &str) -> Option<u32>;
}

pub struct MemStore { /* private */ }

impl MemStore {
    pub fn new(pairs: &[(&str, u32)]) -> Self;
}

pub struct Fallback<A, B> {
    pub first: A,
    pub second: B,
}

impl Store for MemStore { /* async fn get */ }
impl<A: Store, B: Store> Store for Fallback<A, B> { /* async fn get */ }

pub async fn total<S: Store>(s: &S, keys: &[&str]) -> u32;
```

- `MemStore` の `get(key)` は、`pairs` の中で `key` と組になっている値に解決されます。
  そのような組がなければ `None` に解決されます。`pairs` のキーはすべて異なります。
- `Fallback` の `get(key)` は `first` に問い合わせます。その結果が `None` のときに限り `second` に
  問い合わせ、`second` の答えに解決されます。
- `total(s, keys)` は `keys` の各キーについて、順に 1 つずつ `s` に問い合わせ、値の合計に
  解決されます。見つからないキーは `0` として数えます（`keys` が空なら合計は `0` です）。
  同じキーが何度も現れる場合は、そのたびに問い合わせます。合計は必ず `u32` に収まります。
- ハーネスは、自前のストアにも `async fn get` で `Store` を実装しています。この `get` は、
  問い合わせられたキーを記録し、最初にポーリングされたときにはまだ完了しません。ハーネスは
  このストアを単独でも、`Fallback` の中でも使います。

### 例

- `MemStore::new(&[("a", 1), ("b", 2)])` に対して、キー `["a", "b", "zz"]` で呼んだ `total` は
  `3` に解決されます。
- `first` が `MemStore::new(&[("a", 1)])`、`second` が `MemStore::new(&[("a", 100), ("b", 20)])` の
  `Fallback` に対して、キー `["a", "b"]` で呼んだ `total` は `21` に解決されます。`"a"` の値は
  `first` から、`"b"` の値は `second` から得られます。
