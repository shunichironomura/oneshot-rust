`Packet<[u8]>` は**自作の動的サイズ型**（DST）、つまり最後のフィールドのサイズが不定の構造体です。
`[u8]` そのものと同じく、ポインタ越しにしか使えません。`Box<Packet<[u8]>>` や `&Packet<[u8]>` の
ようなポインタはファットポインタで、アドレスの隣にペイロードの長さを持っています。
`Box<Packet<[u8]>>` なら、ID とバイト列は 1 つの領域に収まります。型引数には暗黙の `Sized` 境界が
付いているので、`Packet<[u8]>` や `Packet<dyn Display>` と書くためには、まず構造体に `T: ?Sized` が
必要です。サイズ不定にできるのは最後のフィールドだけで、`payload` はちょうど最後のフィールドです。

`Packet<[u8]>` を直接作ることはできません。構造体式にはすべてのフィールドの値が必要ですが、
`[u8]` 型の値はそれだけでは受け渡しできないからです。代わりに、サイズの決まったパケットから
**サイズ不定化の型強制**（unsizing coercion）で作ります。ポインタ越しなら、`Packet<[u8; N]>` は
`Packet<[u8]>` に型強制できます。`[u8; N]` が `[u8]` に型強制でき、型引数が最後のフィールドにしか
現れないからです。同じように、`T` が `Display` を実装していれば、`Packet<T>` は
`Packet<dyn Display>` に型強制できます。本体を変えずに済むのはこのためです。

- `make` は、const ジェネリクスの `N` を使って `[u8; N]` を受け取るので、どの長さの配列も
  受け取れます。本体が作るのは `Box<Packet<[u8; N]>>` ですが、返り値の型 `Box<Packet<[u8]>>` に
  型強制され、長さ `N` は実行時のデータになります。`-> Box<Packet<[u8; N]>>` でも `make` 自体は
  コンパイルできますが、`make(1, [1, 2, 3])` と `make(2, [9; 5])` の型が違ってしまうので、
  `vec![make(1, [1, 2, 3]), make(2, [9; 5])]` がコンパイルできません。
- `checksum` は `&Packet<[u8]>` を受け取ります。`&Box<Packet<[u8]>>` は参照外しの型強制
  （deref coercion）で、`&Packet { id: 7, payload: [5u8, 6] }` は呼び出し時のサイズ不定化の
  型強制で、この型になります。`&Packet<[u8; N]>` を受け取る const ジェネリックな `checksum` では、
  後者しか受け取れません。
- `label` も同じように、`Box<Packet<T>>` を `Box<Packet<dyn Display>>` に型強制します。
  シグネチャの中の `Box<Packet<dyn Display>>` は `Box<Packet<dyn Display + 'static>>` という意味なので、
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
pub fn label<'a, T: Display + 'a>(id: u32, payload: T) -> Box<Packet<dyn Display + 'a>>

pub fn describe(ps: &[Box<Packet<dyn Display + '_>>]) -> Vec<String>
```

`describe` はジェネリックにもできます。ハーネスは `T = dyn Display` として `describe` を呼ぶので、
`T: ?Sized` が必要です。

```rust
pub fn describe<T: Display + ?Sized>(ps: &[Box<Packet<T>>]) -> Vec<String>
```
