ソケットから読んだバイト列は少しずつ届き、チャンクが文字の途中で終わることもあります。
届いたバイト列を順にデコードする `Utf8Stream` を実装してください。`decode` と `decode_lossy` は
エディタに実装済みで、使って構いません。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid { at: usize, len: usize },
    Incomplete { at: usize },
}

pub fn decode(bytes: &[u8]) -> Result<&str, DecodeError>; // 実装済み
pub fn decode_lossy(bytes: &[u8]) -> (String, usize);      // 実装済み

#[derive(Debug, Default)]
pub struct Utf8Stream {
    pending: Vec<u8>,
}

impl Utf8Stream {
    pub fn new() -> Self;
    pub fn push(&mut self, chunk: &[u8]) -> String;
    pub fn finish(self) -> String;
}
```

UTF-8 は各 `char` を 1〜4 バイトで表します。ある位置のバイトが `char` の表現になっていないとき、
その位置の*不正なシーケンス*は、そこから始まるバイトのうち何らかの `char` の表現の先頭部分になっている
最長の並びです。そのバイトで始まる表現が 1 つもなければ、そのバイト 1 つです。これは
`String::from_utf8_lossy` が U+FFFD に置き換えるシーケンスとちょうど同じです。1 つ目の種類の
不正なシーケンスが入力の末尾まで続いているとき、それを*未完の文字*と呼びます。後からバイトが来れば
完成する可能性があるものです。末尾の `E2 82` は未完の文字ですが（`€` は `E2 82 AC`）、末尾の `C0` や
`FF` は違います。

実装済みの関数：

- `decode(bytes)` は、`bytes` 全体が正しければそのテキストを `Ok` で返します。そうでなければ、
  最初の不正なシーケンスが位置 `at` から始まる未完の文字なら `Err(Incomplete { at })` を、
  それ以外の `len` バイトの不正なシーケンスなら `Err(Invalid { at, len })` を返します。
- `decode_lossy(bytes)` は、不正なシーケンス（末尾の未完の文字も含む）をそれぞれ 1 つの `'?'` に
  置き換えたテキストと、置き換えた数を返します。

`Utf8Stream`：

- `new` は、まだ何も受け取っていないストリームを作ります。
- `push(chunk)` は、それまでに受け取ったバイト列に `chunk` を加え、そこからデコードした
  テキストのうち、まだ返していない部分を返します。それまでに受け取ったバイト列を
  入力とみなしたとき、それが未完の文字で終わっていれば、そのバイトは保留します。
  保留したバイトはまだデコードせず、次のチャンクとつなげて読み直します。その前の部分は
  `decode_lossy` と同じようにデコードするので、不正なシーケンスは受け取った時点で
  `'?'` になります。
- `finish()` はストリームを終えます。保留しているバイトがあれば `"?"` を、なければ `""` を
  返します。
- したがって、バイト列をどのようにチャンクに分けても（空のチャンクもあり得ます）、
  すべての `push` と `finish` の結果をつなげると、バイト列全体に対する `decode_lossy` の
  テキストと同じになります。
- `Utf8Stream` の非公開フィールドは変更して構いません。

### 例

```rust
let mut s = Utf8Stream::new();
s.push(b"caf\xC3")        // "caf"（C3 は 'é' = C3 A9 の先頭部分）
s.push(b"\xA9 \xE2")      // "é "
s.push(b"\x82\xAC!\xFF")  // "€!?"（FF は何をつなげても完成しない）
s.finish()                // ""

let mut s = Utf8Stream::new();
s.push(&[b'x', 0xF0, 0x9F])  // "x"
s.finish()                   // "?"
```
