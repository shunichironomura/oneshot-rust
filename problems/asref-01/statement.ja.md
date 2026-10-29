次の関数の本体はすでに書かれていますが、シグネチャが 1 つの具体的な要素の型に決め打ちされて
いるので、ハーネス（`tests.rs` タブ）がコンパイルできません。**シグネチャだけ**を書き換えて、
ハーネスがコンパイルでき、テストに合格するようにしてください。本体は変更しないでください。

```rust
pub fn join_upper(words: &[String], sep: &str) -> String
```

- `join_upper` はすべての単語を大文字にし、隣り合う単語の間に `sep` を挟んでつなげます。
  ハーネスは `&str` の配列への参照、`Vec<String>` への参照、`Vec<String>` のスライスを渡します。
  空のリストを渡すときは、`join_upper::<String>(&[], ",")` のように要素の型を明示します。

### 例

```rust
join_upper(&["ab", "c"], "-")              // "AB-C"
join_upper(&vec![String::from("é")], ",")  // "É"
join_upper::<String>(&[], ",")             // ""
```
