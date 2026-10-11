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
impl<T: Describe + ?Sized> Describe for &T
```

入れ子の型では、コンパイラがこれらの `impl` を連鎖させます。`Vec<Option<i32>>` が `Describe` を
実装するのは `Option<i32>` が実装するからで、それは `i32` が実装するからです。

- `Vec` の本体は `self.as_slice().describe()` を呼ぶので、`[T]: Describe` が必要です。スライスの
  `impl` もジェネリックにしなければならず、`impl Describe for [i32]` のままでは、ジェネリックにした
  `Vec` の `impl` のコンパイルが通りません。
- 組には型引数が 2 つ必要です。`(T, T)` では `(true, vec!["a", "b"])` に合いません。要素が `bool` と
  `Vec<&str>` だからです。
- 本当の罠は、すでにジェネリックに見える `&T` の `impl` です。型引数にはすべて暗黙の `Sized` 境界が
  付いているので、`impl<T: Describe> Describe for &T` は `&i32` や `&String` には当てはまりますが、
  `&str`、`&[i32]`、`&dyn Describe` には当てはまりません。`str`、`[i32]`、`dyn Describe` は
  動的サイズ型だからです。`vec!["a", "b"]` は `Vec<&str>` なので、説明するには `&str: Describe` が
  必要で、これは `T = str` としたときの `&T` の `impl` です。`T: ?Sized` と書くと暗黙の境界が外れます。
  本体は `T` を参照越しにしか使わないので、これが許されます。`**self` は型 `T` の場所で、
  `.describe()` がそれをもう一度借用します。
- `Describe` のようにトレイトオブジェクトにできるトレイトでは、`dyn Describe` に対する `Describe` の
  実装をコンパイラ自身が用意します。`?Sized` があれば `&dyn Describe` も `Describe` になるので、
  `Vec<&dyn Describe>` に型の違う値を入れられます。

ほかの `impl` には `?Sized` は合いません。スライス、`Vec<T>`、`Option<T>` は `T` の値を直接
格納するので、`T` の大きさはコンパイル時にわかっていなければならず、
`impl<T: Describe + ?Sized> Describe for Vec<T>` はエラー（E0277）になります（タプルでは最後の要素だけが
サイズ不定になれるので、`B: ?Sized` は通りますが `A: ?Sized` は通りません。どちらも不要です）。

代わりに `impl Describe for &str` を追加するのはルール違反ですし、それでは `&str` しか扱えず、
`&[i32]` や `&dyn Describe` には効きません。`&T` に `?Sized` を付けると `&str` もそれに含まれるので、
`&str` 用の 2 つ目の `impl` は実装の衝突（E0119）としてエラーになります。コヒーレンスにより、
1 つの型に対するトレイトの `impl` は 1 つまでだからです（テンプレートの `?Sized` のない `&T` の
`impl` と並べた場合は通ります。`str` は決して `Sized` にならないので、2 つの `impl` は重ならないからです）。

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

impl<T> Describe for &T
where
    T: Describe + ?Sized,
{
    fn describe(&self) -> String {
        (**self).describe()
    }
}
```
