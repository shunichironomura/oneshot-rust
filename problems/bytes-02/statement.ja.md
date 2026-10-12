ある小さなネットワークプロトコルは、パケットを生のバイト列として、1 つのストリームにすき間なく並べて
送ります。ストリームをパケットに分ける `decode_all` を実装してください。次の型とヘッダの解析関数は
エディタに入っています。

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

struct Header {
    kind: Kind,
    seq: u16,
    declared: u32,
}

fn split_header(bytes: &[u8]) -> Result<(Header, &[u8]), PacketError> // 実装済み

pub fn decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError>
```

パケットは 9 バイトのヘッダと、それに続くペイロードからなります。

| バイト | フィールド | 形式 |
| --- | --- | --- |
| `0..2` | マジック | 2 バイトの `b"OS"` |
| `2` | 種類 | `1` は `Ping`、`2` は `Data`、`3` は `Ack` |
| `3..5` | `seq` | `u16`、リトルエンディアン |
| `5..9` | ペイロード長（`declared`） | `u32`、ビッグエンディアン |
| `9..` | `payload` | 続く `declared` バイト |

- `split_header(bytes)` は `bytes` の先頭にあるヘッダを読みます。`bytes` が 9 バイトより短ければ
  `TooShort`、先頭の 2 バイトが `b"OS"` でなければ `BadMagic`、種類のバイト `k` が `1`、`2`、`3` の
  どれでもなければ `UnknownKind(k)` を、この順にチェックして返します。それ以外の場合は、ヘッダと、
  その後ろのすべてのバイトを返します。
- `decode_all(stream)` は `stream` の中のパケットをデコードします。最初のパケットは `stream` の先頭から
  始まり、次のパケットは前のパケットのペイロードの直後から始まります。パケットの後ろにバイトが
  残っていなければデコードを終え、すべてのパケットを順に返します。`stream` が空なら `Ok(vec![])` です。
- 各パケットのヘッダは `split_header` と同じようにチェックします。ヘッダの後に残っているバイトが
  `declared` バイトより少なければ、エラーは `LengthMismatch { declared, actual }` で、`actual` は
  ヘッダの後に残っているバイト数です。
- 不正なパケットがあれば、最初の不正なパケットのエラーを返します。
- `decode_all` は、どんな入力に対してもパニックしてはいけません。

### 例

```rust
let stream = [
    b'O', b'S', 1, 1, 0, 0, 0, 0, 0,       // Ping、seq 1、空のペイロード
    b'O', b'S', 2, 2, 0, 0, 0, 0, 1, b'!', // Data、seq 2、ペイロード b"!"
];
decode_all(&stream)
// Ok(vec![
//     Packet { kind: Kind::Ping, seq: 1, payload: &[] },
//     Packet { kind: Kind::Data, seq: 2, payload: b"!" },
// ])

let cut = [
    b'O', b'S', 1, 1, 0, 0, 0, 0, 0,       // Ping、seq 1、空のペイロード
    b'O', b'S', 2, 2, 0, 0, 0, 0, 3, b'!', // Data、seq 2、3 バイトと宣言、残りは 1 バイト
];
decode_all(&cut)        // Err(LengthMismatch { declared: 3, actual: 1 })
decode_all(&cut[..12])  // Err(TooShort)
```
