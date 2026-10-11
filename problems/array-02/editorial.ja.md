エラーは 3 つあり、コンパイラはすべてを一度に報告します。3 つとも、`char`、`u8`、`usize` の境目で
起きています。Rust はこの境目を自動では越えません。

- **`first.to_ascii_lowercase() - 'a'`**：`char` にはそもそも算術演算がなく、`char` どうしの引き算も
  できません。先に整数に変換します。ここでの `first` は ASCII の英字なので `as u8` で情報は失われません。
  `b'a'` はバイト `97` です。`(first.to_ascii_lowercase() as u8 - b'a') as usize` と書きます。
- **`buckets[letter - b'a']`**：`letter` は範囲 `b'a'..=b'z'` から来るので `u8` で、差も `u8` です。
  配列、スライス、`Vec` の添字は `usize` だけです（「the type `[Vec<String>]` cannot be indexed by
  `u8`」）。小さい整数型が自動で広げられることもありません。`usize::from(letter - b'a')` で
  情報を失わずに広げます。`(letter - b'a') as usize` でも構いません。
- **`letters.push(letter)`**：`u8` は、ASCII のコードを持っていても `char` ではありません。
  `char::from(letter)` か `letter as char` で変換します。この変換ができるのは `u8` からだけです。
  バイトの値はどれも有効な `char`（U+0000 〜 U+00FF）ですが、`u32` の値はそうとは限らないので、
  `97u32 as char` はエラーになり、`char::from_u32` は `Option` を返します。

`Default::default()` はこのままで問題ありません。配列は要素の型が `Default` なら（長さ 32 までは）
`Default` を実装しているので、別々の空の `Vec` が 26 個できます。

算術演算の周りのコードはそのまま残してください。`to_ascii_lowercase` がないと、
`b'A' - b'a'` がアンダーフローし、デバッグビルドではパニックします。`is_ascii_alphabetic` を
`is_alphabetic` にすると、ASCII 以外の英字もチェックを通り、`as u8` はコードポイントの下位 1 バイトしか
残しません。`ñ`（U+00F1）では添字が 25 をはるかに超え、`š`（U+0161）にいたってはバケツ `a` に
入ってしまいます。

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

`bucket_by_initial` の添字は、`first.to_ascii_lowercase() as usize - 'a' as usize` でも構いません。
`char` はどの整数型にもキャストできます。制限があるのは `char` *への*キャストだけで、`u8` に限られます。
