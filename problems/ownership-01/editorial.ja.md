`for name in names` は `names` をループに**ムーブ**するので、ループの後の `names.len()` はムーブ済みの値を使うことになります。
`&names` を回せば借用で済み、`name` は `&String` になります。`format!` はそれでも同じように使えます。

### 別解

イテレータで挨拶を組み立てる書き方です。`names.iter()` の借用は `collect` の時点で終わっているので、
その後に `names.len()` を呼べます。

```rust
pub fn greet_all(names: Vec<String>) -> (Vec<String>, usize) {
    let greetings = names.iter().map(|name| format!("Hello, {name}!")).collect();
    (greetings, names.len())
}
```

ループの前に `let count = names.len();` と数を取っておけば、ループで `names` を消費しても構いません。
