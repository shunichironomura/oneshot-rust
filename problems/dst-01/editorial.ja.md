`str`、スライス `[T]`、トレイトオブジェクト `dyn Trait` は**動的サイズ型**（DST）で、値の大きさは
実行時にしかわかりません。そのため、いつも `&str`、`Box<[T]>`、`&dyn Debug` のように
ポインタ越しに使います。このポインタは「ファットポインタ」で、アドレスの隣に長さ（`str` と `[T]`）か
vtable へのポインタ（`dyn Trait`）を持っています。

型引数にはすべて、暗黙の `Sized` 境界が付いています。`fn byte_size<T>(x: &T)` は、実際には
`fn byte_size<T: Sized>(x: &T)` という意味です。そのため、テンプレートを素直にジェネリックにしただけでは、
`T = str` が必要な `byte_size("héllo")` の呼び出しで「the size for values of type `str` cannot be known at
compilation time」（E0277）になります。`T: ?Sized` と書くと、この既定の境界が外れます（「`T` は `Sized` で
あってもなくてもよい」）。ここでそれが許されるのは、本体が `T` をポインタ越しにしか使わないからです。
`T` が `Sized` でなくても、`&T` 自体の大きさは決まっています。

`std::mem::size_of_val` 自身が `fn size_of_val<T: ?Sized>(val: &T) -> usize` と宣言されています。
ファットポインタの付加情報から大きさを計算するので、`&dyn Debug` ならトレイトオブジェクトの裏にある
値の大きさを返します。

ほかに思いつきやすい直し方が 2 つありますが、どちらもうまくいきません。引数の型を
`&dyn Debug` にすると、`"héllo"` を渡せません。`str` を指す `&dyn Debug` は長さと vtable の両方を
持たなければなりませんが、ファットポインタにはどちらか一方の分の場所しかないので、
トレイトオブジェクトにできるのは `Sized` な型だけです。また、本体が使わない境界を付けると、
ハーネスの引数の一部を受け取れなくなります。`T: Debug + ?Sized` と書くと `Rgb` を受け取れません。

### 別解

引数位置の `impl Trait` にも `?Sized` を付けられます。

```rust
pub fn byte_size(x: &impl ?Sized) -> usize {
    std::mem::size_of_val(x)
}
```
