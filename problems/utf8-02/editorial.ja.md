`<[u8]>::utf8_chunks` はバイト列を、正しい部分の `&str`（`valid()`）と、その後に続く 1 つの不正な
シーケンス（`invalid()`。空になり得るのは最後のチャンクだけ）に分けます。ただしこれだけでは、
未完の文字（`E2 82`）と、何をつなげても完成しない不正なシーケンス（`FF`）を区別できません。
どちらも最後のチャンクの `invalid()` になるからです。末尾の不正なシーケンスをすべて保留すると、
`FF` の `'?'` が遅れてしまいます。`decode` を使えば区別できますが、使ってよいのは末尾にある
シーケンスだけです。`x` が続く `E2 82` は不正なシーケンスですが、`decode(&[0xE2, 0x82])` 単独では
`Incomplete` になります。

ループは `utf8_chunks()` を通じて `self.pending` を借用しているので、デコードし終えたバイト数を
`used` に数えるだけにして、ループの後で `drain(..used)` で取り除きます。`finish` の時点で残っているのは、
未完の文字か、何もないかのどちらかで、`decode_lossy` がそれぞれ `"?"` と `""` にします。

### 別解

`push` は `decode` だけでも書けます。`Incomplete` がちょうどバイトを保留する場合です。
`rest` は `self.pending` を借用しているので、使ったバイト数を `drain` の前に計算しておきます。

```rust
pub fn push(&mut self, chunk: &[u8]) -> String {
    self.pending.extend_from_slice(chunk);
    let mut text = String::new();
    let mut rest = &self.pending[..];
    loop {
        match decode(rest) {
            Ok(valid) => {
                text.push_str(valid);
                rest = &[];
                break;
            }
            Err(DecodeError::Invalid { at, len }) => {
                text.push_str(decode(&rest[..at]).unwrap());
                text.push('?');
                rest = &rest[at + len..];
            }
            Err(DecodeError::Incomplete { at }) => {
                text.push_str(decode(&rest[..at]).unwrap());
                rest = &rest[at..];
                break;
            }
        }
    }
    let used = self.pending.len() - rest.len();
    self.pending.drain(..used);
    text
}
```
