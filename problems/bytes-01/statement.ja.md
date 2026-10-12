ある小さなネットワークプロトコルは、パケットを生のバイト列として送ります。
バイト列と次の型（エディタに入っています）の間の変換を実装してください。

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

impl TryFrom<u8> for Kind { /* 実装済み */ }
impl<'a> TryFrom<&'a [u8]> for Packet<'a> { /* type Error = PacketError; */ }

impl Packet<'_> {
    pub fn encode(&self) -> Vec<u8>;
}
```

パケットは 9 バイトのヘッダと、それに続くペイロードからなります。

| バイト | フィールド | 形式 |
| --- | --- | --- |
| `0..2` | マジック | 2 バイトの `b"OS"`（`0x4F`、`0x53`） |
| `2` | 種類 | `1` は `Ping`、`2` は `Data`、`3` は `Ack` |
| `3..5` | `seq` | `u16`、リトルエンディアン |
| `5..9` | ペイロード長 | `u32`、**ビッグエンディアン** |
| `9..` | `payload` | ペイロードのバイト列 |

- `Kind::try_from(b)` は実装済みです。`1`、`2`、`3` をそれぞれ `Ping`、`Data`、`Ack` に変換し、
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
- どの関数も、どんな入力に対してもパニックしてはいけません。

### 例

```rust
let bytes = [b'O', b'S', 2, 0x34, 0x12, 0, 0, 0, 2, 0xAA, 0xBB];
Packet::try_from(&bytes[..])
// Ok(Packet { kind: Kind::Data, seq: 0x1234, payload: &[0xAA, 0xBB] })
Packet::try_from(&bytes[..10])  // Err(LengthMismatch { declared: 2, actual: 1 })
Packet::try_from(&bytes[..5])   // Err(TooShort)

Packet { kind: Kind::Ack, seq: 7, payload: b"hi" }.encode()
// vec![b'O', b'S', 3, 7, 0, 0, 0, 0, 2, b'h', b'i']
```
