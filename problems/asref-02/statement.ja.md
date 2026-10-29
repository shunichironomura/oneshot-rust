次の関数は、文字列のグループに含まれる文字を数えます。本体はすでに書かれていますが、シグネチャが
`Vec<String>` の `Vec` しか受け取らないので、ハーネス（`tests.rs` タブ）がコンパイルできません。
**シグネチャだけ**（と、必要なら `where` 節）を書き換えて、ハーネスがコンパイルでき、テストに
合格するようにしてください。本体は変更しないでください。

```rust
pub fn total_chars(groups: Vec<Vec<String>>) -> usize
```

- `total_chars` は、文字列（`&str` または `String`）のグループに含まれる文字（`char`）の
  総数を返します。ハーネスは `Vec` の `Vec`、配列の配列、`Vec` の配列、それらへの参照、そして
  要素が `&str` のイテレータを要素とするイテレータ（`text.lines().map(|l| l.split(' '))` など）を
  渡します。

### 例

```rust
total_chars(vec![vec!["ab", "c"], vec![]])          // 3
total_chars([["a", "b"]; 3])                        // 6
total_chars(&[vec![String::from("é")]])             // 1
total_chars("a b\nc".lines().map(|l| l.split(' ')))  // 3
```
