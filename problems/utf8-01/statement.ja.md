ファイルやソケットから読んだバイト列は、まだテキストではありません。正しい UTF-8 だと
わかって初めて `&str` になります。厳密なデコーダ、不正な部分を置き換えるデコーダ、
少しずつ届くバイト列のためのデコーダを実装してください。`DecodeError` と `Utf8Stream` は
エディタに入っています。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid { at: usize, len: usize },
    Incomplete { at: usize },
}

pub fn decode(bytes: &[u8]) -> Result<&str, DecodeError>;
pub fn decode_lossy(bytes: &[u8]) -> (String, usize);

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

UTF-8 は各 `char` を 1〜4 バイトで表し、バイト列は先頭から 1 文字ずつ読みます。
ある位置のバイトが `char` の UTF-8 表現になっていないとき、その位置の
*不正なシーケンス*は次のとおりです。

- そこから始まるバイトのうち、何らかの `char` の表現の先頭部分になっている最長の並び。
- その位置のバイトで始まる `char` の表現が 1 つもなければ、そのバイト 1 つ。

読み取りは不正なシーケンスの直後から続けます。これは `String::from_utf8_lossy` が
U+FFFD に置き換えるシーケンスとちょうど同じで、長さはどれも 1〜3 バイトです。

| バイト列 | 不正なシーケンス |
| --- | --- |
| `F0 9F 98 41` | `F0 9F 98`。`😀`（`F0 9F 98 80`）の先頭部分です。続く `41` は `A` として読みます。 |
| `C0 AF` | `C0` と `AF`。先頭のバイトが `C0` になる `char` はありません。 |
| `ED A0 80` | `ED`、`A0`、`80`。`ED A0 80` はサロゲート U+D800 になりますが、これは `char` ではありません。 |

1 つ目の種類の不正なシーケンスが入力の末尾まで続いているとき、それを*未完の文字*と
呼びます。後からバイトが来れば完成する可能性があるものです。たとえば末尾の `E2 82` は
未完の文字ですが（`€` は `E2 82 AC`）、末尾の `C0` は何をつなげても完成しないので
未完の文字ではありません。

`decode(bytes)` はバイト列を厳密に検査します。

- `bytes` 全体が正しい UTF-8 なら、そのテキストを `Ok` で返します（空の入力も正しい
  UTF-8 です）。テキストは `bytes` を借用し、コピーはしません。
- そうでなければ、最初の不正なシーケンスが始まる位置を `at` とします（`bytes[..at]` が
  正しい UTF-8 である最長の先頭部分です）。そのシーケンスが未完の文字なら
  `Err(DecodeError::Incomplete { at })` を、そうでなければ
  `Err(DecodeError::Invalid { at, len })` を返します。`len` はそのシーケンスのバイト数です。

`decode_lossy(bytes)` は失敗しません。

- 不正なシーケンス（末尾の未完の文字も含む）をそれぞれ 1 つの `'?'` に置き換えた
  テキストと、置き換えた数を返します。
- それ以外はそのまま残します。`bytes` の中に正しくエンコードされた `'?'` や `'\u{FFFD}'`
  （U+FFFD。UTF-8 では `EF BF BD`）があっても、それらは置き換えではなく、数にも入れません。
  言い換えると、テキストは `String::from_utf8_lossy` の結果のうち、不正なシーケンスの
  代わりに入れられた U+FFFD だけを `'?'` にしたものです。

`Utf8Stream` は、ソケットからの読み込みのように少しずつ届くバイト列をデコードします。
1 回分のチャンクは文字の途中で終わることがあります。

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
decode("hé".as_bytes())                  // Ok("hé")
decode(b"hi\xFFthere")                   // Err(Invalid { at: 2, len: 1 })
decode(&[0xF0, 0x9F, 0x98, b'A'])        // Err(Invalid { at: 0, len: 3 })
decode(&[b'a', 0xE2, 0x82])              // Err(Incomplete { at: 1 })

decode_lossy(b"ab\xFFcd")                // ("ab?cd", 1)
decode_lossy(&[0xC0, 0xAF])              // ("??", 2)
decode_lossy(&[0xF0, 0x9F, 0x98, b'A'])  // ("?A", 1)
decode_lossy(b"\xEF\xBF\xBD?")           // ("\u{FFFD}?", 0)。どちらも正しくエンコードされている
decode_lossy(&[b'a', 0xE2, 0x82])        // ("a?", 1)

let mut s = Utf8Stream::new();
s.push(b"caf\xC3")        // "caf"（C3 は 'é' = C3 A9 の先頭部分）
s.push(b"\xA9 \xE2")      // "é "
s.push(b"\x82\xAC!\xFF")  // "€!?"（FF は何をつなげても完成しない）
s.finish()                // ""

let mut s = Utf8Stream::new();
s.push(&[b'x', 0xF0, 0x9F])  // "x"
s.finish()                   // "?"
```
