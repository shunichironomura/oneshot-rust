ペイロードの型が違うパケットどうしが同じ型になるには、ペイロードの型を消すしかありません。
それが `Packet<dyn Display>`、つまり最後のフィールドがトレイトオブジェクトである**自作の動的サイズ型**（DST）です。
`dyn Display` そのものと同じく、ポインタ越しにしか使えません。`Box<Packet<dyn Display>>` のような
ポインタはファットポインタで、アドレスの隣にペイロードの vtable へのポインタを持っています。
型引数には暗黙の `Sized` 境界が付いているので、`Packet<dyn Display>` と書くためには、まず構造体に
`T: ?Sized` が必要です。サイズ不定にできるのは最後のフィールドだけで、`payload` はちょうど最後の
フィールドです。

`Packet<dyn Display>` を直接作ることはできません。代わりに、サイズの決まったパケットから
**サイズ不定化の型強制**（unsizing coercion）で作ります。ポインタ越しなら、`T` が `Display` を
実装していれば、`Packet<T>` は `Packet<dyn Display>` に型強制できます。型引数が最後のフィールドに
しか現れないからです。本体を変えずに済むのはこのためです。

- `label` が作るのは `Box<Packet<T>>` ですが、返り値の型 `Box<Packet<dyn Display>>` に型強制されます。
  `-> Box<Packet<T>>` でも `label` 自体はコンパイルできますが、`label(1, 5)` と `label(2, "x")` の
  型が違ってしまうので、`vec![label(1, 5), label(2, "x")]` がコンパイルできません。
- シグネチャの中の `Box<Packet<dyn Display>>` は `Box<Packet<dyn Display + 'static>>` という意味なので、
  `T: Display` だけでは「the parameter type `T` may not live long enough」（E0310）になります。
  `T: Display + 'static` とすれば解決します。`5`、`"x"`、`String` のようなペイロードは、
  短命なものを何も借用していません。
- `describe` は、`Vec` が持っているボックスのスライス `&[Box<Packet<dyn Display>>]` を受け取ります。
  スライスの要素は `Sized` でなければならないので、`&[Packet<dyn Display>]` という型は使えません。
  `{}` は、ペイロードごとの vtable を通して `&p.payload` を書式化します。

### 別解

`label` は、`'static` を要求する代わりに、ペイロードのライフタイムを引き継ぐこともできます。
そのようなパケットを受け取るには、`describe` にもライフタイムが必要です。

```rust
pub fn label<'a, T: Display + 'a>(id: u32, payload: T) -> Box<Packet<dyn Display + 'a>> {
    Box::new(Packet { id, payload })
}

pub fn describe(ps: &[Box<Packet<dyn Display + '_>>]) -> Vec<String> {
    ps.iter().map(|p| format!("#{}: {}", p.id, &p.payload)).collect()
}
```

`describe` はジェネリックにもできます。ハーネスは `T = dyn Display` として `describe` を呼ぶので、
`T: ?Sized` が必要です。

```rust
pub fn describe<T: Display + ?Sized>(ps: &[Box<Packet<T>>]) -> Vec<String> {
    ps.iter().map(|p| format!("#{}: {}", p.id, &p.payload)).collect()
}
```
