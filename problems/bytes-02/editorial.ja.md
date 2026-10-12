`split_at(n)` を使えばペイロードとストリームの残りを切り分けられますが、`n` がスライスより大きいと
パニックします。`split_at_checked` は代わりに `None` を返すので、`ok_or` で `LengthMismatch` にできます。
各ステップでは `rest` の範囲を前に進めるだけで、バイト列はコピーしません。

ヘッダにあるペイロード長は `u32`、スライスの長さは `usize` です。`declared as usize` は値を広げる変換です
（32 ビットと 64 ビットのターゲットでは何も失われません）。一方 `after_header.len() as u32` では、
4 GiB 以上の長さが切り捨てられてしまいます。また、`9 + declared` を `u32` で計算するのも避けてください。
長さが `u32::MAX` に近いとオーバーフローし、ジャッジのようなデバッグビルドではパニックします。

ペイロードはどれも `stream` の一部なので、パケットは `stream` を借用します。
`decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError>` の `'_` は `stream` のライフタイムです。
参照の引数が 1 つだけなので、省略規則によって戻り値が `stream` に結び付きます。同じ規則で
`split_header` が返すスライスは `bytes` に結び付くので、`after_header` と `rest` も `stream` の一部です。

### 別解

`get(..n)` も、範囲が末尾を越えると `None` を返します。これが成功した後なら、`after_header[n..]` は
パニックしません。

```rust
pub fn decode_all(stream: &[u8]) -> Result<Vec<Packet<'_>>, PacketError> {
    let mut packets = Vec::new();
    let mut rest = stream;
    while !rest.is_empty() {
        let (Header { kind, seq, declared }, after_header) = split_header(rest)?;
        let n = declared as usize;
        let Some(payload) = after_header.get(..n) else {
            return Err(PacketError::LengthMismatch { declared, actual: after_header.len() });
        };
        packets.push(Packet { kind, seq, payload });
        rest = &after_header[n..];
    }
    Ok(packets)
}
```
