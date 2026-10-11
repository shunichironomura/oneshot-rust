`str`、スライス `[T]`、トレイトオブジェクト `dyn Trait` は**動的サイズ型**（DST）で、値の大きさは
実行時にしかわかりません。そのため、いつも `&str`、`Box<[T]>`、`Rc<dyn Fn(i32) -> i32>` のように
ポインタ越しに使います。このポインタは「ファットポインタ」で、アドレスの隣に長さ（`str` と `[T]`）か
vtable へのポインタ（`dyn Trait`）を持っています。

型引数にはすべて、暗黙の `Sized` 境界が付いています。`fn byte_size<T>(x: &T)` は、実際には
`fn byte_size<T: Sized>(x: &T)` という意味です。そのため、テンプレートを素直にジェネリックにしただけでは、
`T = str` が必要な `byte_size("héllo")` の呼び出しで「the size for values of type `str` cannot be known at
compilation time」（E0277）になります。`T: ?Sized` と書くと、この既定の境界が外れます（「`T` は `Sized` で
あってもなくてもよい」）。ここでそれが許されるのは、本体が `T` をポインタ越しにしか使わないからです。
`T` が `Sized` でなくても、`&T` や `Box<T>` 自体の大きさは決まっています。

- `byte_size`：`std::mem::size_of_val` 自身が `fn size_of_val<T: ?Sized>(val: &T) -> usize` と
  宣言されています。ファットポインタの付加情報から大きさを計算するので、`&dyn Debug` なら
  トレイトオブジェクトの裏にある値の大きさを返します。
- `show_pair`：2 つの引数の型が違うので、型引数が 2 つ必要です。`T` が 1 つだけだと、
  `show_pair("x", &[1, 2][..])` が型の不一致になります。`?Sized` は書いた型引数にしか効かないので、
  両方に必要です。`{a:?}` は `impl<T: ?Sized + Debug> Debug for &T` を通して `a: &A` を書式化するので、
  両方に `Debug` も必要です。
- `share`：標準ライブラリは `T: ?Sized` について `From<Box<T>> for Rc<T>` を実装しているので、
  `Rc::from(b)` は `Box<str>` を `Rc<str>` に、`Box<dyn Fn(i32) -> i32>` を `Rc<dyn Fn(i32) -> i32>` に
  変換できます。値は、参照カウントも入る新しい領域に移されます。

ほかに思いつきやすい直し方が 2 つありますが、どちらもうまくいきません。引数の型を
`&dyn Debug` にすると、`"x"` を渡せません。`str` を指す `&dyn Debug` は長さと vtable の両方を
持たなければなりませんが、ファットポインタにはどちらか一方の分の場所しかないので、
トレイトオブジェクトにできるのは `Sized` な型だけです。また、本体が使わない境界を付けると、
ハーネスの引数の一部を受け取れなくなります。`byte_size` や `share` に `T: Debug + ?Sized` と
書くと `Rgb` やボックス化したクロージャを、`share` に `T: ?Sized + 'static` と書くと
ローカル変数を借用している `Box<&str>` を受け取れません。

### 別解

引数位置の `impl Trait` にも `?Sized` を付けられます。`show_pair` では `+` があるので括弧が必要です。
`share` は返り値の型で型引数を使うので、名前の付いた型引数のままにする必要があります。

```rust
pub fn byte_size(x: &impl ?Sized) -> usize

pub fn show_pair(a: &(impl Debug + ?Sized), b: &(impl Debug + ?Sized)) -> String
```

`show_pair` は、参照そのものを値として受け取ることもできます。型は `&str` や `&[i32]` などになり、
これらは `Sized` です。参照の `Debug` は指している値を表示するので、出力も変わりません。

```rust
pub fn show_pair(a: impl Debug, b: impl Debug) -> String
```

この方法はほかの 2 つの関数には使えません。`byte_size` の本体は `x` を参照を受け取る `size_of_val` に
渡していますし、`share` の返り値の型には `Rc<str>` のように、サイズ不定の型そのものを書く必要があるからです。
