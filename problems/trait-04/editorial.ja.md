`Describe` は自分のクレートのトレイトなので、どんな型にも実装できます。`str`、`Vec<T>`、`Option<T>`、
タプル、参照といった標準ライブラリの型も例外ではありません。孤児ルールが禁じているのは、
他のクレートのトレイトを他のクレートの型に実装すること（`Vec<T>` に `Display` を実装するなど）だけです。

ジェネリックな `impl` は「境界を満たすすべての `T` について、この型は `Describe` を実装する」と読みます。
コンテナの本体は要素の `describe` を呼ぶので、コンテナの `impl` にはそれぞれ、`Describe` 境界の付いた
型引数が必要です。

```rust
impl<T: Describe> Describe for [T]
impl<T: Describe> Describe for Vec<T>
impl<T: Describe> Describe for Option<T>
impl<A: Describe, B: Describe> Describe for (A, B)
```

入れ子の型では、コンパイラがこれらの `impl` を連鎖させます。`Vec<Option<i32>>` が `Describe` を
実装するのは `Option<i32>` が実装するからで、それは `i32` が実装するからです。

- `Vec` の本体は `self.as_slice().describe()` を呼ぶので、`[T]: Describe` が必要です。ハーネスは
  スライスも直接説明しますが、そうでなくてもスライスの `impl` はジェネリックにしなければなりません。
  `impl Describe for [i32]` のままでは、ジェネリックにした `Vec` の `impl` のコンパイルが通りません。
- ペアには型引数が 2 つ必要です。`(T, T)` では `(true, vec!["a", "b"])` に合いません。要素が `bool` と
  `Vec<&str>` だからです。
- 本体が要素の `describe` を呼べるのは境界があるからです。`impl<T> Describe for Option<T>` では
  `T` について何もわからないので、`value.describe()` がエラー（E0599）になります。
- 型引数は `impl` の直後で宣言しなければなりません。`impl Describe for Vec<T>` とだけ書くと
  「cannot find type `T` in this scope」になります。

`&T` の `impl` は最初からジェネリックです。型引数にはすべて暗黙の `Sized` 境界が付いていますが、
`?Sized` がそれを外しているので、`&str`、`&[i32]`、`&dyn Describe` にも当てはまります。`str`、
`[i32]`、`dyn Describe` は動的サイズ型です。`vec!["a", "b"]` は `Vec<&str>` なので、説明するときは
`T = str` とした `&T` の `impl` を使います。`Describe` のようにトレイトオブジェクトにできるトレイトでは、
`dyn Describe` に対する `Describe` の実装をコンパイラ自身が用意します。そのため `Vec<&dyn Describe>` に
型の違う値を入れられます。

ほかの `impl` には `?Sized` は合いません。スライスと `Vec<T>` は `T` の値を隙間なく並べて格納し、
`Option<T>` は `T` の値をそのまま中に持つので、`T` の大きさはコンパイル時にわかっていなければ
なりません。そのため `impl<T: Describe + ?Sized> Describe for Vec<T>` はエラー（E0277）になります
（タプルでは最後の要素だけがサイズ不定になれるので、`B: ?Sized` は通りますが `A: ?Sized` は
通りません。どちらも不要です）。

### 別解

境界は `where` 節に書くこともできます。`Vec` の `impl` では、`T: Describe` の代わりに、本体が実際に
使う `[T]: Describe` をそのまま要求できます。

```rust
impl<T> Describe for Vec<T>
where
    [T]: Describe,
{
    fn describe(&self) -> String {
        self.as_slice().describe()
    }
}

impl<A, B> Describe for (A, B)
where
    A: Describe,
    B: Describe,
{
    fn describe(&self) -> String {
        format!("({}, {})", self.0.describe(), self.1.describe())
    }
}
```
