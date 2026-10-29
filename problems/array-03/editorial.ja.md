エラーは 2 つあり、コンパイラは両方を一度に報告します。どちらも、`u8`、`usize`、`char` の境目で
起きています。Rust はこの境目を自動では越えません。

- **`buckets[letter - b'a']`**：`letter` は範囲 `b'a'..=b'z'` から来るので `u8` で、差も `u8` です。
  配列、スライス、`Vec` の添字は `usize` だけです（エラーは「cannot be indexed by `u8`」）。
  小さい整数型が自動で広げられることもありません。`usize::from(letter - b'a')` で
  情報を失わずに広げます。`(letter - b'a') as usize` でも構いません。
- **`letters.push(letter)`**：`u8` は、ASCII のコードを持っていても `char` ではありません。
  `char::from(letter)` か `letter as char` で変換します。この変換ができるのは `u8` からだけです。
  バイトの値はどれも有効な `char`（U+0000 〜 U+00FF）ですが、`u32` の値はそうとは限らないので、
  `97u32 as char` はエラーになり、`char::from_u32` は `Option` を返します。

### 別解

`char` の範囲もイテレータなので、`letters_used` は算術演算も添字も使わずに、各英字とバケツを組にできます。

```rust
pub fn letters_used(buckets: &[Vec<String>; 26]) -> String {
    ('a'..='z')
        .zip(buckets)
        .filter(|(_, bucket)| !bucket.is_empty())
        .map(|(letter, _)| letter)
        .collect()
}
```

`enumerate` なら添字は `usize` で得られるので、注意が要るのは `char` に戻すところだけです。
足す前に添字を `u8` に変換します。`(b'a' + i) as char` は `u8` に `usize` を足すことになり、
`('a' as usize + i) as char` は、`char` にキャストできるのが `u8` だけなのでエラーになります。

```rust
pub fn letters_used(buckets: &[Vec<String>; 26]) -> String {
    let mut letters = String::new();
    for (i, bucket) in buckets.iter().enumerate() {
        if !bucket.is_empty() {
            letters.push((b'a' + i as u8) as char);
        }
    }
    letters
}
```
