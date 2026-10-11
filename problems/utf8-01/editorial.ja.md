`std::str::from_utf8` はバイト列を検査し、正しければそのバイト列を指す `&str` を返すので、
コピーは起きません。ライフタイムの省略規則により、返す `&str` は `bytes` を借用します。
`from_utf8(bytes)` が返すのはちょうどそれです。`String::from_utf8(bytes.to_vec())` だと
新しい `String` を作ることになり、ローカルな `String` を指す `&str` は返せません。

失敗したときは、`Utf8Error` がどこで何が起きたかを教えてくれます。`valid_up_to()` は
正しい UTF-8 である最長の先頭部分の長さです。`error_len()` は、`len` バイトの不正なシーケンスなら
`Some(len)`、文字の途中で入力が終わったなら `None` です。これがそのまま `Invalid` と
`Incomplete` に対応します。

`decode_lossy` には `String::from_utf8_lossy` を使いたくなりますが、返り値は `Cow<str>` です。
また `from_utf8_lossy(bytes).replace('\u{FFFD}', "?")` では、入力の中で正しくエンコード
されていた U+FFFD まで書き換えてしまいます。結果の中の U+FFFD を数えても同じ問題が
起きます。`<[u8]>::utf8_chunks` を使うと、`from_utf8_lossy` が扱うのと同じ単位に分けられます。
各 `Utf8Chunk` は、正しい部分の `&str`（`valid()`）と、その後に続く 1 つの不正なシーケンス
（`invalid()`。空になり得るのは最後のチャンクだけ）です。デコーダを自分で書くのは
思ったより難しいものです。不正なシーケンスの長さは冗長なエンコード、サロゲート、
U+10FFFF を超える値によって変わります。不正なバイトを 1 つずつ置き換えると、
`F0 9F 98 41` は `"?A"` ではなく `"???A"` になってしまいます。

`Utf8Stream::push` では、`utf8_chunks` だけでは未完の文字（`E2 82`）と、何をつなげても完成
しない不正なシーケンス（`FF`）を区別できません。どちらも最後のチャンクの `invalid()` に
なるからです。末尾の不正なシーケンスをすべて保留すると、`FF` の `'?'` が遅れてしまいます。
`decode` を使えば区別できますが、使ってよいのは末尾にあるシーケンスだけです。`x` が続く
`E2 82` は不正なシーケンスですが、`decode(&[0xE2, 0x82])` 単独では `Incomplete` になります。
ループは `utf8_chunks()` を通じて `self.pending` を借用しているので、デコードし終えた
バイトはループの後で `drain(..used)` で取り除きます。`finish` の時点で残っているのは、
未完の文字か、何もないかのどちらかです。

### 別解

`from_utf8` をループで呼ぶ `decode_lossy` です。未完の文字（`error_len()` が `None`）は
入力の末尾まで続くので、残りをすべて読み飛ばします。

```rust
pub fn decode_lossy(bytes: &[u8]) -> (String, usize) {
    let mut text = String::with_capacity(bytes.len());
    let mut replaced = 0;
    let mut rest = bytes;
    loop {
        match std::str::from_utf8(rest) {
            Ok(valid) => {
                text.push_str(valid);
                return (text, replaced);
            }
            Err(e) => {
                let (valid, after) = rest.split_at(e.valid_up_to());
                // This prefix is valid UTF-8, so checking it again cannot fail.
                text.push_str(std::str::from_utf8(valid).unwrap());
                text.push('?');
                replaced += 1;
                rest = &after[e.error_len().unwrap_or(after.len())..];
            }
        }
    }
}
```

`push` も同じように `decode` で書けます。`Incomplete` がちょうどバイトを保留する場合です。
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
