次の 5 つの関数の本体はすでに書かれていますが、どのシグネチャも 1 つの具体的な型に決め打ちされて
いるので、ハーネス（`tests.rs` タブ）がコンパイルできません。**シグネチャだけ**を書き換えて、
ハーネスがコンパイルでき、テストに合格するようにしてください。本体は変更しないでください。

```rust
pub fn char_len(s: &str) -> usize
pub fn join_upper(words: &[String], sep: &str) -> String
pub fn total_chars(groups: Vec<Vec<String>>) -> usize
pub fn square_in_place(n: &mut u32)
pub fn widen(values: Vec<i32>) -> Vec<i64>
```

- `char_len` は文字列に含まれる文字（`char`）の数を返します。ハーネスは `&str`、`String`、
  `&String` の値を渡します。
- `join_upper` はすべての単語を大文字にし、隣り合う単語の間に `sep` を挟んでつなげます。
  ハーネスは `&str` の配列への参照、`Vec<String>` への参照、`Vec<String>` のスライスを渡します。
  空のリストを渡すときは、`join_upper::<String>(&[], ",")` のように要素の型を明示します。
- `total_chars` は、文字列（`&str` または `String`）のグループに含まれる文字（`char`）の
  総数を返します。ハーネスは `Vec` の `Vec`、配列の配列、`Vec` の配列、それらへの参照、そして
  要素が `&str` のイテレータを要素とするイテレータ（`text.lines().map(|l| l.split(' '))` など）を
  渡します。
- `square_in_place` は `u32` の値をその 2 乗で置き換えます。ハーネスは `&mut Box<u32>` と、
  ハーネス自身の型 `Score`（`AsMut<u32>` を実装しています）への可変参照を渡します。
- `widen` は整数を、順番を保ったまま `i64` に変換します。ハーネスは `u8`、`u16`、`i32`、
  `u32`、`i64` の値の `Vec`、配列、イテレータを渡します。

### 例

```rust
char_len("Café")                           // 4
char_len(String::from("ab"))               // 2
join_upper(&["ab", "c"], "-")              // "AB-C"
join_upper(&vec![String::from("é")], ",")  // "É"
total_chars([["a", "b"]; 3])               // 6
total_chars(&[vec![String::from("é")]])    // 1

let mut b = Box::new(3u32);
square_in_place(&mut b);                   // *b は 9 になる
widen(vec![1u8, 2])                        // vec![1, 2]（Vec<i64>）
```
