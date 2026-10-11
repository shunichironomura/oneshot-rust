ある小さなネットワークプロトコルは、パケットを生のバイト列として送ります。
バイト列と次の型（エディタに入っています）の間の変換と、
バイト列のストリームをパケットに分ける関数を実装してください。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Ping = 1,
    Data = 2,
    Ack = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    TooShort,
    BadMagic,
    UnknownKind(u8),
    LengthMismatch { declared: u32, actual: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Packet<'a> {
    pub kind: Kind,
    pub seq: u16,
    pub payload: &'a [u8],
}

impl TryFrom<u8> for Kind { /* type Error = PacketError; */ }
impl<'a> TryFrom<&'a [u8]> for Packet<'a> { /* type Error = PacketError; */ }

impl Packet<'_> {
    pub fn encode(&self) -> Vec<u8>;
}

pub fn decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError>
```

パケットは 9 バイトのヘッダと、それに続くペイロードからなります。

| バイト | フィールド | 形式 |
| --- | --- | --- |
| `0..2` | マジック | 2 バイトの `b"OS"`（`0x4F`、`0x53`） |
| `2` | 種類 | `1` は `Ping`、`2` は `Data`、`3` は `Ack` |
| `3..5` | `seq` | `u16`、リトルエンディアン |
| `5..9` | ペイロード長 | `u32`、**ビッグエンディアン** |
| `9..` | `payload` | ペイロードのバイト列 |

- `Kind::try_from(b)` は `1`、`2`、`3` をそれぞれ `Ping`、`Data`、`Ack` に変換します。
  それ以外の `b` では `Err(PacketError::UnknownKind(b))` を返します。
- `Packet::try_from(bytes)` は、ちょうど 1 つのパケットが入ったスライスをデコードします。
  次のチェックをこの順に行い、最初に失敗したチェックのエラーを返します。
  1. `bytes` が 9 バイトより短い：`TooShort`。
  2. 先頭の 2 バイトが `b"OS"` でない（大文字と小文字は区別します）：`BadMagic`。
  3. 種類のバイト `k` が `1`、`2`、`3` のどれでもない：`UnknownKind(k)`。
  4. ヘッダより後ろのバイト数が、ヘッダにあるペイロード長と等しくない：
     `LengthMismatch { declared, actual }`。`declared` はヘッダにあるペイロード長、
     `actual` はヘッダより後ろのバイト数です。

  すべてのチェックを通過すれば、パケットを返します。その `payload` は、
  `bytes` のうちヘッダより後ろの部分です。
- `encode` はその逆で、ヘッダとそれに続くペイロードを返します。つまり、
  `Packet::try_from(&p.encode()[..])` は `Ok(p)` を返します。
  ペイロードの長さは `u32::MAX` バイト以下だと仮定して構いません。
- `decode_all(stream)` は、すき間なく並んだパケットをデコードします。最初のパケットは
  `stream` の先頭から始まり、次のパケットは前のパケットのペイロードの直後から始まります。
  パケットの後ろにバイトが残っていなければデコードを終え、すべてのパケットを順に返します。
  `stream` が空なら `Ok(vec![])` です。
  - 各パケットは `Packet::try_from` と同じようにチェックしますが、チェック 4 だけは違います。
    ペイロードはヘッダの後の `declared` バイトで、その後ろのバイトは次のパケットのものです。
    ヘッダの後に残っているバイトが `declared` バイトより少なければ、エラーは
    `LengthMismatch { declared, actual }` で、`actual` はヘッダの後に残っているバイト数です。
  - 特に、パケットが始まるはずの位置から後ろに 9 バイト未満しか残っていなければ、
    エラーは `TooShort` です。
  - 不正なパケットがあれば、最初の不正なパケットのエラーを返します。
- どの関数も、どんな入力に対してもパニックしてはいけません。

### 例

```rust
let bytes = [b'O', b'S', 2, 0x34, 0x12, 0, 0, 0, 2, 0xAA, 0xBB];
Packet::try_from(&bytes[..])
// Ok(Packet { kind: Kind::Data, seq: 0x1234, payload: &[0xAA, 0xBB] })
Packet::try_from(&bytes[..10])  // Err(LengthMismatch { declared: 2, actual: 1 })
Packet::try_from(&bytes[..5])   // Err(TooShort)
Kind::try_from(4)               // Err(UnknownKind(4))

Packet { kind: Kind::Ack, seq: 7, payload: b"hi" }.encode()
// vec![b'O', b'S', 3, 7, 0, 0, 0, 0, 2, b'h', b'i']

let stream = [
    b'O', b'S', 1, 1, 0, 0, 0, 0, 0,       // Ping、seq 1、空のペイロード
    b'O', b'S', 2, 2, 0, 0, 0, 0, 1, b'!', // Data、seq 2、ペイロード b"!"
];
decode_all(&stream)
// Ok(vec![
//     Packet { kind: Kind::Ping, seq: 1, payload: &[] },
//     Packet { kind: Kind::Data, seq: 2, payload: b"!" },
// ])
```
