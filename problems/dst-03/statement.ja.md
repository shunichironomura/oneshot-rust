`Packet` は、ID の後にペイロードが続くパケットです。下のコードでは、`label` で作ったパケットは
`String` しか持てません。テストハーネス（`tests.rs` タブ）は新しい設計を前提にしています。
`label` は `Display` を実装したどんな値でも受け取り、作ったパケットはすべて同じ型になるので、
1 つの `Vec` に入れられます。

本体は新しい設計に合わせて書かれていますが、シグネチャが古い設計のままなので、
ハーネスがコンパイルできません。ハーネスがコンパイルでき、テストに通るように、
**シグネチャだけ**（`struct` の行と `fn` の行）を書き換えてください。本体とフィールドは
変更しないでください。

```rust
pub struct Packet<T> {
    pub id: u32,
    pub payload: T,
}

pub fn label(id: u32, payload: String) -> Box<Packet<String>>
pub fn describe(ps: &[Box<Packet<String>>]) -> Vec<String>
```

- `label(id, payload)` は、値を新しくボックス化したパケットに移します。ハーネスは整数、
  浮動小数点数、文字、`bool`、文字列リテラル、`String` を渡します。`label` で作ったパケットは、
  ペイロードの型によらずすべて同じ型です。
- `describe(ps)` は、各パケットを `#<id>: <payload>` の形に書式化します。ペイロードは
  `{}`（`Display`）で書式化します。ハーネスは `label` で作ったパケットの `Vec` か、`&ls[1..]` の
  ようにその一部を渡します。その `Vec` には、`Box::new(Packet { id: 3, payload: 'q' })` のように、
  ハーネスが自分でボックス化したパケットが入っていることもあります。

ハーネスは、パケットのフィールド `id` と `payload` も直接読みます。

### 例

```rust
let ls = vec![label(1, 5), label(2, "x")];
describe(&ls)  // ["#1: 5", "#2: x"]
```
