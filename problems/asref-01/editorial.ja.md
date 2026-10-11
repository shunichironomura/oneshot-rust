すべての呼び出しに合う具体的な型はなく、参照外し型強制（deref coercion）も役に立ちません。
引数が `&str` なら `&String` を `&str` に変換してくれますが、値で渡す `String` はそもそも参照では
ありません。また、型強制はスライスや配列、`Vec` の中の要素を変えることはありません。
`&Vec<String>` は `&[String]` にはなっても、`&[&str]` にはなりません。そこで、どの関数にも型引数を
導入し、本体が使っているトレイトで境界を付けます。`as_ref()` は `AsRef<str>`、`as_mut()` は
`AsMut<u32>`、`into_iter()` は `IntoIterator`、`Into::into` は `Into<i64>` のメソッドです。

- `char_len` は `S` を**値で**受け取ります。`&str` と `String` は `AsRef<str>` を実装しており、
  `&String` も包括実装 `impl<T: ?Sized + AsRef<U>, U: ?Sized> AsRef<U> for &T` によって実装しています。
  代わりに `s: &S` とすると、`String` の引数が合わなくなり、`"Café"` では `S = str` が必要になります。
  `str` はサイズが不定ですが、型引数は `?Sized` を付けない限り `Sized` です。
- `join_upper` はスライスのまま、要素の型をジェネリックにして `&[S]` とします。`&["ab", "c"]` は
  `&[&str]` に、`&Vec<String>` は `&[String]` に型強制され、`S` は要素の型から推論されます。
  `words: &[impl AsRef<str>]` では**うまくいきません**。引数の `impl Trait` の型には名前がないので、
  `join_upper::<String>` で指定できる型引数がなくなるからです（「function takes 0 generic
  arguments」）。`&[]` だけでは要素の型がわからないので、ハーネスにはターボフィッシュが必要です。
- `total_chars` には、要素の要素への境界が必要です。模範解答では関連型境界（Rust 1.79 で安定化）を
  使って `I: IntoIterator<Item: IntoIterator<Item: AsRef<str>>>` と書いています。`IntoIterator` なら
  `Vec` だけでなく、配列、参照、イテレータも受け取れます。`&[Vec<String>; 1]` を回すと
  `&Vec<String>` が得られ、その要素は `&String` で、これもやはり `AsRef<str>` です。
- `square_in_place` は `T: AsMut<u32>` として `&mut T` を受け取ります。`Box<u32>` は `AsMut<u32>` を
  実装しており、`Score` も実装しています。
- `widen` は、要素が `Into<i64>` である任意の `IntoIterator` を受け取ります。`u8`、`u16`、`i32`、
  `u32` の値はどれも `i64` に収まります（`u64` は大きな値が収まらないので、`Into<i64>` を実装して
  いません）。

`s.as_ref()` があいまいにならないのも境界のおかげです。テンプレートの `s: &str` では `as_ref` の
候補が複数あり（`str` は `AsRef<str>`、`AsRef<[u8]>`、`AsRef<OsStr>`、`AsRef<Path>` を実装しています）、
本体まで「type annotations needed」でエラーになります。ジェネリックな関数の中では `S` について
わかっているのは `S: AsRef<str>` だけなので、`as_ref` は `&str` を返します。

### 別解

引数位置の `impl Trait` を使うと、`join_upper` 以外は短く書けます。`join_upper` は呼び出し側が
型引数を指定するので、名前のある型引数が必要です。

```rust
pub fn char_len(s: impl AsRef<str>) -> usize
pub fn total_chars(groups: impl IntoIterator<Item = impl IntoIterator<Item = impl AsRef<str>>>) -> usize
pub fn square_in_place(n: &mut impl AsMut<u32>)
pub fn widen(values: impl IntoIterator<Item = impl Into<i64>>) -> Vec<i64>
```

関連型境界を使わない場合は、`where` 節に段ごとの境界を書きます。内側の要素の型は
`<I::Item as IntoIterator>::Item` と書く必要があります。`I::Item::Item` はあいまいな関連型として
エラーになります。

```rust
pub fn total_chars<I>(groups: I) -> usize
where
    I: IntoIterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
```

段ごとに型引数を用意する書き方もあります。

```rust
pub fn total_chars<I, G, S>(groups: I) -> usize
where
    I: IntoIterator<Item = G>,
    G: IntoIterator<Item = S>,
    S: AsRef<str>,
```

`square_in_place` は `T` を値で受け取ることもできます。`&mut b` を渡すと `T` は `&mut Box<u32>` に
なり、これは `&mut T` 向けの実装によって `AsMut<u32>` を実装しています。`as_mut` は `n` を可変で
借用するので、引数は `mut n` にする必要があります。

```rust
pub fn square_in_place<T: AsMut<u32>>(mut n: T)
```
