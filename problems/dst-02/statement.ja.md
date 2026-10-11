`Packet` は、ID の後にペイロードが続くパケットです。下のコードでは、`make` で作ったパケットは
バイト列を別の `Vec<u8>` に持ち、`label` で作ったパケットは `String` しか持てません。
テストハーネス（`tests.rs` タブ）は新しい設計を前提にしています。`make` はバイト配列をパケット自体の
中に移し、ID とバイト列が 1 つの領域に収まるようにします。`label` は `Display` を実装した
どんな値でも受け取ります。

本体はすべて新しい設計に合わせて書かれていますが、シグネチャが古い設計のままなので、
ハーネスがコンパイルできません。ハーネスがコンパイルでき、テストに通るように、
**シグネチャだけ**（`struct` の行と `fn` の行）を書き換えてください。本体とフィールドは
変更しないでください。

```rust
pub struct Packet<T> {
    pub id: u32,
    pub payload: T,
}

pub fn make(id: u32, bytes: Vec<u8>) -> Box<Packet<Vec<u8>>>
pub fn checksum(p: &Packet<Vec<u8>>) -> u32
pub fn longest(ps: &[Box<Packet<Vec<u8>>>]) -> Option<u32>
pub fn label(id: u32, payload: String) -> Box<Packet<String>>
pub fn describe(ps: &[Box<Packet<String>>]) -> Vec<String>
```

- `make(id, bytes)` は、任意の長さ（`[]` も含む）のバイト配列を、新しくボックス化したパケットに
  移します。`make` で作ったパケットは、ペイロードの長さによらずすべて同じ型です。ハーネスは
  それらを 1 つの `Vec` にまとめて入れ、ボックス越しにバイトを書き換えます。
- `checksum(p)` は、ID にペイロードのバイトの合計を足した値を返します。ハーネスは `make` で作った
  パケットのほか、`&Packet { id: 7, payload: [5u8, 6] }` のように、バイト配列をペイロードにして
  自分で作ったパケットも渡します。
- `longest(ps)` は、ペイロードがいちばん長いパケットの ID を返します。同じ長さのパケットが
  複数あれば、そのうち最初のものを選びます。`ps` が空なら `None` を返します。ハーネスは
  `make` で作ったパケットの `Vec` か、`&ps[1..]` のようにその一部を渡します。
- `label(id, payload)` は、値を新しくボックス化したパケットに移します。ハーネスは整数、
  浮動小数点数、文字、`bool`、文字列リテラル、`String` を渡します。`label` で作ったパケットは、
  ペイロードの型によらずすべて同じ型です。
- `describe(ps)` は、各パケットを `#<id>: <payload>` の形に書式化します。ペイロードは
  `{}`（`Display`）で書式化します。ハーネスは `label` で作ったパケットの `Vec` か、その一部を
  渡します。その `Vec` には、`Box::new(Packet { id: 3, payload: 'q' })` のように、ハーネスが
  自分でボックス化したパケットが入っていることもあります。

ハーネスは、パケットのフィールド `id` と `payload` も直接読みます。

### 例

```rust
let a = make(1, [1, 2, 3]);
checksum(&a)                                    // 7（1 + 1 + 2 + 3）
checksum(&Packet { id: 7, payload: [5u8, 6] })  // 18

let ps = vec![make(1, [1, 2, 3]), make(2, [9; 5]), make(3, [])];
longest(&ps)                                    // Some(2)

let ls = vec![label(1, 5), label(2, "x")];
describe(&ls)                                   // ["#1: 5", "#2: x"]
```
