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

ペイロードは入力の一部なので、`Packet<'a>` は入力を借用します。
`impl<'a> TryFrom<&'a [u8]> for Packet<'a>` が 2 つのライフタイムを結び付けているので、
`try_from` の引数は `bytes: &'a [u8]` になります。ここを `bytes: &[u8]` と書くと
「lifetime may not live long enough」というエラーになります。
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
