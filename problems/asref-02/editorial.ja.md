本体は `into_iter()` を 2 回、`as_ref()` を 1 回呼ぶので、シグネチャには段ごとに境界が 1 つずつ
必要です。引数は `IntoIterator`、その要素もまた `IntoIterator`、さらにその要素は `AsRef<str>` です。
使うトレイトは `Iterator` ではなく `IntoIterator` です。`Vec` や配列はイテレータではありませんが、
イテレータに変換でき、どのイテレータも `IntoIterator` を実装しています。

模範解答では関連型境界（Rust 1.79 で安定化）を使って
`I: IntoIterator<Item: IntoIterator<Item: AsRef<str>>>` と書いています。`&[Vec<String>; 1]` を回すと
`&Vec<String>` が得られ、その要素は `&String` です。`&String` も包括実装
`impl<T: ?Sized + AsRef<U>, U: ?Sized> AsRef<U> for &T` によって `AsRef<str>` を実装しているので、
参照にも何も追加せずに対応できます。

`s.as_ref()` があいまいにならないのも境界のおかげです。テンプレートの `s: String` では `as_ref` の
候補が複数あり（`String` は `AsRef<str>`、`AsRef<[u8]>`、`AsRef<OsStr>`、`AsRef<Path>` を実装しています）、
本体まで「type annotations needed」でエラーになります。ジェネリックな関数の中では、内側の要素について
わかっているのは `AsRef<str>` であることだけなので、`as_ref` は `&str` を返します。

### 別解

関連型境界を使わない場合は、`where` 節に段ごとの境界を書きます。内側の要素の型は
`<I::Item as IntoIterator>::Item` と書く必要があります。`I::Item::Item` はあいまいな関連型として
エラーになります。

```rust
pub fn total_chars<I>(groups: I) -> usize
where
    I: IntoIterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}
```

段ごとに型引数を用意する書き方もあります。

```rust
pub fn total_chars<I, G, S>(groups: I) -> usize
where
    I: IntoIterator<Item = G>,
    G: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}
```

引数位置の `impl Trait` を使うと短く書けます。

```rust
pub fn total_chars(groups: impl IntoIterator<Item = impl IntoIterator<Item = impl AsRef<str>>>) -> usize {
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}
```
