ファイルやソケットから読んだバイト列は、まだテキストではありません。正しい UTF-8 だと
わかって初めて `&str` になります。厳密なデコーダと、不正な部分を置き換えるデコーダを
実装してください。`DecodeError` はエディタに入っています。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid { at: usize, len: usize },
    Incomplete { at: usize },
}

pub fn decode(bytes: &[u8]) -> Result<&str, DecodeError>;
pub fn decode_lossy(bytes: &[u8]) -> (String, usize);
```

UTF-8 は各 `char` を 1〜4 バイトで表し、バイト列は先頭から 1 文字ずつ読みます。
ある位置のバイトが `char` の UTF-8 表現になっていないとき、その位置の
*不正なシーケンス*は次のとおりです。

- そこから始まるバイトのうち、何らかの `char` の表現の先頭部分になっている最長の並び。
- その位置のバイトで始まる `char` の表現が 1 つもなければ、そのバイト 1 つ。

読み取りは不正なシーケンスの直後から続けます。これは `String::from_utf8_lossy` が
U+FFFD に置き換えるシーケンスとちょうど同じです。

| バイト列 | 不正なシーケンス |
| --- | --- |
| `F0 9F 98 41` | `F0 9F 98`。`😀`（`F0 9F 98 80`）の先頭部分です。続く `41` は `A` として読みます。 |
| `C0 AF` | `C0` と `AF`。先頭のバイトが `C0` や `AF` になる `char` はありません。 |
| `ED A0 80` | `ED`、`A0`、`80`。`ED A0 80` はサロゲート U+D800 になりますが、これは `char` ではありません。 |

1 つ目の種類の不正なシーケンスが入力の末尾まで続いているとき、それを*未完の文字*と
呼びます。後からバイトが来れば完成する可能性があるものです。たとえば末尾の `E2 82` は
未完の文字ですが（`€` は `E2 82 AC`）、末尾の `C0` は未完の文字ではありません。

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
```
