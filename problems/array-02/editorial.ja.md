エラーは 3 つあり、コンパイラはすべてを一度に報告します。

- **`[Vec::new(); 26]`**：繰り返し式 `[value; N]` は `value` を 1 回だけ評価し、その結果を各要素にコピーするので、
  要素の型は `Copy` でなければなりません。`Vec` はヒープ上のバッファを所有していて、ビット単位でコピーした
  26 個はすべて同じバッファを指してしまいます。`Vec<String>` が `Copy` でないのはそのためです。
  例外は定数です。`value` が `const` ブロックか定数の名前なら、コピーされずに `N` 回評価されます。
  `Vec::new` は `const fn` なので、`Vec::new()` をインラインの `const` ブロックで包めます。
  `[const { Vec::new() }; 26]` と書けば、別々の空の `Vec` が 26 個できます。
- **`first.to_ascii_lowercase() - 'a'`**：`char` にはそもそも算術演算がないので、先に整数に変換します。
  ここでの `first` は ASCII の英字なので `as u8` で情報は失われません。`b'a'` はバイト `97` です。
  `(first.to_ascii_lowercase() as u8 - b'a') as usize` と書きます。
- **`'a' + i`**：同じ問題の逆向きです。しかも `char` に戻す道は狭く、`char` にキャストできるのは `u8` だけです。
  `('a' as usize + i) as char` はエラーになり、`(b'a' + i) as char` は `u8` に `usize` を足しています。
  添字のほうを変換して `(b'a' + i as u8) as char` と書きます。

算術演算の周りのコードはそのまま残してください。`to_ascii_lowercase` がないと、
`b'A' - b'a'` がアンダーフローし、デバッグビルドではパニックします。`is_ascii_alphabetic` を
`is_alphabetic` にすると、ASCII 以外の英字もチェックを通り、`as u8` はコードポイントの下位 1 バイトしか
残しません。`ñ`（U+00F1）では添字が 25 をはるかに超え、`š`（U+0161）にいたってはバケツ `a` に
入ってしまいます。

### 別解

`std::array::from_fn` は、添字ごとに 1 回クロージャを呼んで配列を作ります（引数は添字で、
ここでは使いません）。クロージャでは何でも計算でき、`Vec::with_capacity(8)` のような定数にできない値も
作れるので、もっとも汎用的な方法です。

```rust
let mut buckets: [Vec<String>; 26] = std::array::from_fn(|_| Vec::new());
```

ほかに 2 つの方法も使えます。名前付きの定数も `const` ブロックと同じように繰り返せます。
`const EMPTY: Vec<String> = Vec::new();` としてから `[EMPTY; 26]` と書きます。
`Default::default()` でも構いません。配列は要素の型が `Default` なら `Default` を実装しますが、
長さ 32 までに限られるので、バケツが 40 個ならコンパイルが通らなくなります。

`char` の範囲はイテレータなので、`letters_used` は算術演算なしで各英字とバケツを組にできます。

```rust
pub fn letters_used(buckets: &[Vec<String>; 26]) -> String {
    ('a'..='z')
        .zip(buckets)
        .filter(|(_, bucket)| !bucket.is_empty())
        .map(|(letter, _)| letter)
        .collect()
}
```

添字のほうは、`first.to_ascii_lowercase() as usize - 'a' as usize` でも構いません。
`char` はどの整数型にもキャストできます。制限があるのは `char` *への*キャストだけで、`u8` に限られます。
