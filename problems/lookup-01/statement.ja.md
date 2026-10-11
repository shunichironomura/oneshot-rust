`Registry<K, V>` は `HashMap<K, V>` を包んだ型です（エディタに入っています）。メソッドの本体は
すでに書かれていますが、4 つのシグネチャがキーとして `&K` しか受け付けないので、ハーネス
（`tests.rs` タブ）のコンパイルが通りません。ハーネスは、`HashMap` でできるのと同じように、`String` のキーを
`&str` で、`Vec<u8>` のキーを `&[u8]` で引くからです。`get`、`contains`、`remove`、`get_or` の
**シグネチャだけ**を書き換えて、ハーネスがコンパイルでき、テストに合格するようにしてください。
構造体、本体、`new`、`insert` は変更しないでください。`use` 宣言は追加してもかまいません。

```rust
impl<K: Hash + Eq, V> Registry<K, V> {
    pub fn new() -> Self;                              // 変更しない
    pub fn insert(&mut self, k: K, v: V);              // 変更しない
    pub fn get(&self, key: &K) -> Option<&V>;          // 直す
    pub fn contains(&self, key: &K) -> bool;           // 直す
    pub fn remove(&mut self, key: &K) -> Option<V>;    // 直す
    pub fn get_or(&self, key: &K, default: &V) -> &V;  // 直す
}
```

- `insert(k, v)` は `k` に `v` を格納します。`k` がすでにあれば、その値を置き換えます。
- `get(key)` は `key` に格納された値を返します。なければ `None` です。
- `contains(key)` は `key` がレジストリにあるかどうかを返します。
- `remove(key)` は `key` を取り除いて、その値を返します。なければ `None` です。
- `get_or(key, default)` は `key` に格納された値を返します。`key` がなければ `default` そのものを
  返します。
- ハーネスが使うのは `Registry<String, u32>`、`Registry<Vec<u8>, &str>`、`Registry<u32, char>`
  です。`HashMap` と同じく、キーの代わりに、キーを借用した形で検索できます。`String` のキーなら
  `&str` か `&String`、`Vec<u8>` のキーなら `&[u8]` か `&Vec<u8>`、`u32` のキーなら `&u32` です。
- キーは検索に使うだけです。`get` と `get_or` の返り値がキーを借用することはないので、返り値を
  使う前にキーを破棄してもかまいません。`get_or` の返り値は、レジストリか `default` を借用します。

### 例

```rust
let mut r: Registry<String, u32> = Registry::new();
r.insert(String::from("apple"), 1);
r.insert(String::from("pear"), 2);
r.get("apple")                // Some(&1)
r.get(&String::from("pear"))  // Some(&2)
r.contains("pear")            // true
r.remove("apple")             // Some(1)。その後の r.get("apple") は None
let zero = 0;
r.get_or("zzz", &zero)        // &0
r.get_or("pear", &zero)       // &2

let mut bytes: Registry<Vec<u8>, &str> = Registry::new();
bytes.insert(vec![1, 2], "one-two");
bytes.get(&[1u8, 2][..])      // Some(&"one-two")

let mut chars: Registry<u32, char> = Registry::new();
chars.insert(7, 'x');
chars.get(&7)                 // Some(&'x')
```
