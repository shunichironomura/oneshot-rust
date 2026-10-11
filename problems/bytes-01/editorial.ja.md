`u16::from_le_bytes` と `u32::from_be_bytes` が値で受け取るのは、スライスではなく**配列**
（`[u8; 2]`、`[u8; 4]`）です。そのため `u16::from_le_bytes(bytes[3..5])` はコンパイルが通りません。
模範解答では `split_first_chunk::<9>()` で配列を取り出しています。9 バイト未満なら `None`
を返し（チェック 1）、そうでなければ `Some((&[u8; 9], &[u8]))`、つまり配列にしたヘッダと
スライスの残りを返します。`*header` に配列パターンを使うと 9 バイトすべてに名前を付けられ、
`[s0, s1]` と `[l0, l1, l2, l3]` で 2 つの数値用の配列を作れます。バイト順に注意してください。
`seq` はリトルエンディアン、ペイロード長はビッグエンディアンです。`encode` では
`to_le_bytes` と `to_be_bytes` で書き戻します。

添字で読む場合は、先に長さをチェックしてください。短いスライスでは `bytes[2]` や
`bytes[5..9]` がパニックします。また、マジックを先にチェックすると（たとえば
`starts_with` で）、バイトが違う短いスライスに対して `TooShort` ではなく `BadMagic`
を返してしまいます。`bytes[3..5].try_into()` で `[u8; 2]` は作れますが、そのエラー型は
`TryFromSliceError` で、`?` では `PacketError` に変換できません。長さをチェックした後なら
変換は失敗しないので、`unwrap()` で構いません。

ヘッダにあるペイロード長は `u32`、スライスの長さは `usize` なので、そのままでは比較できません。
`declared as usize` は値を広げる変換です（32 ビットと 64 ビットのターゲットでは何も失われません）。
一方 `payload.len() as u32` では、4 GiB 以上の長さが切り捨てられてしまいます。また、
`9 + declared` を `u32` で計算するのも避けてください。長さが `u32::MAX` に近いとオーバーフローし、
ジャッジのようなデバッグビルドではパニックします。

`decode_all` で `split_at(n)` を使うと、`n` がスライスより大きいときにパニックします。
`split_at_checked` は代わりに `None` を返すので、`ok_or` で `LengthMismatch` にできます。
各ステップでは `rest` の範囲を前に進めるだけで、バイト列はコピーしません。

ペイロードは入力の一部なので、`Packet<'a>` は入力を借用します。
`impl<'a> TryFrom<&'a [u8]> for Packet<'a>` が 2 つのライフタイムを結び付けているので、
`try_from` の引数は `bytes: &'a [u8]` になります。ここを `bytes: &[u8]` と書くと
「lifetime may not live long enough」というエラーになります。ヘルパーの
`split_header(bytes: &[u8]) -> Result<(Header, &[u8]), PacketError>` には注釈が要りません。
参照の引数が 1 つだけなので、省略規則によって返すスライスが `bytes` に結び付きます。
最後に、`Kind` の判別値はバイト列での値と同じなので、`self.kind as u8` で種類のバイトになります。

### 別解

以下のコードは、模範解答の定数 `MAGIC` と `HEADER_LEN` を使っています。

長さをチェックした後で、スライスに添字を使って読む方法です。

```rust
impl<'a> TryFrom<&'a [u8]> for Packet<'a> {
    type Error = PacketError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.len() < HEADER_LEN {
            return Err(PacketError::TooShort);
        }
        if !bytes.starts_with(b"OS") {
            return Err(PacketError::BadMagic);
        }
        let kind = Kind::try_from(bytes[2])?;
        // 長さは上でチェックしたので、これらの変換は失敗しない
        let seq = u16::from_le_bytes(bytes[3..5].try_into().unwrap());
        let declared = u32::from_be_bytes(bytes[5..9].try_into().unwrap());
        let payload = &bytes[HEADER_LEN..];
        if payload.len() != declared as usize {
            return Err(PacketError::LengthMismatch { declared, actual: payload.len() });
        }
        Ok(Packet { kind, seq, payload })
    }
}
```

`Kind::try_from` は、数値を繰り返し書く代わりに、判別値でバリアントを探すこともできます。

```rust
fn try_from(b: u8) -> Result<Self, Self::Error> {
    [Kind::Ping, Kind::Data, Kind::Ack]
        .into_iter()
        .find(|&kind| kind as u8 == b)
        .ok_or(PacketError::UnknownKind(b))
}
```

`encode` は `concat` で各部分をつなげても書けます。型注釈によって、指す配列の長さに関係なく、
どの要素も `&[u8]` になります。

```rust
pub fn encode(&self) -> Vec<u8> {
    let len = self.payload.len() as u32;
    let parts: [&[u8]; 5] = [
        &MAGIC,
        &[self.kind as u8],
        &self.seq.to_le_bytes(),
        &len.to_be_bytes(),
        self.payload,
    ];
    parts.concat()
}
```

`decode_all` では `try_from` を再利用できます。ペイロード長だけを読んでパケットの終わりを求め、
ちょうどその部分に対して `try_from` にすべてのチェックをさせます。ペイロードが途中で切れていれば、
その部分は残り全体になり、`try_from` が同じ `LengthMismatch` を返します。

```rust
pub fn decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError> {
    let mut packets = Vec::new();
    let mut rest = stream;
    while !rest.is_empty() {
        // このパケットの終わりを求め、ちょうどその部分を `try_from` でチェックする
        let end = match rest.first_chunk::<HEADER_LEN>() {
            Some(header) => {
                let declared = u32::from_be_bytes([header[5], header[6], header[7], header[8]]);
                rest.len().min((declared as usize).saturating_add(HEADER_LEN))
            }
            None => rest.len(), // 短すぎる：`try_from` が TooShort を返す
        };
        let (packet, next) = rest.split_at(end);
        packets.push(Packet::try_from(packet)?);
        rest = next;
    }
    Ok(packets)
}
```
