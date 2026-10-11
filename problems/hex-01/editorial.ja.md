計算したりスライスしたりする前に、`#` の後のすべての文字が 16 進数字かどうかを確かめることが
大切です。

`strip_prefix('#')` は、先頭の `#` を取り除いた残りを返し、`#` がなければ `None` を返します。
その `None` は `?` でそのまま返せます。`char::to_digit(16)` は 16 進数字の値を返し、
それ以外の `char` には（ASCII でない文字も含めて）`None` を返します。`Option` を要素とするイテレータを
`Option<Vec<_>>` に `collect` すると、1 つでも `None` があれば結果は `None` になります。
残るのは数字の値の列なので、スライスパターンで正しい 2 つの長さだけを受け付けます。
3 桁の形では、`dd` は `16 * d + d`、つまり `17 * d` です。

途中には次の落とし穴があります。

- `u8::from_str_radix` は（`str::parse` と同じく）先頭の `+` を受け付けるので、
  `u8::from_str_radix("+f", 16)` は `Ok(15)` です。`#+f+f+f` を 2 文字ずつ変換すると、
  `None` ではなく `[15, 15, 15]` になります。6 桁をまとめて `u32` として変換すると `#+fff00` を
  受け付けてしまいます。符号なし整数型は `-` を拒否しますが、符号付き整数型なら受け付けます。
  先に `is_ascii_hexdigit`（または `to_digit`）で全文字を確かめれば、この穴はふさがります。
- `s.len()` は**バイト数**です。`#1é234` は 7 バイトで 6 桁のカラーコードと同じ長さですが、
  `&s[1..3]` はパニックします。バイト位置 3 が `é` の途中だからです。`#€` は 4 バイトで
  3 桁のカラーコードと同じ長さですが、`&s[1..2]` もパニックします。バイト位置でそのまま
  スライスしてよいのは、文字列が ASCII だとわかってからです。`s.get(1..3)` なら、
  パニックせずに `None` を返します。`char` を `c as u8` でバイトに変換するのもいけません。
  下位 1 バイトだけが残るので、`'٣'`（U+0663）は `b'c'` になり、`#٣٣٣` が `#ccc` として
  読まれてしまいます。
- `{:x}` は先頭に 0 を付けないので、`[255, 8, 0]` が `#ff80` になってしまいます。
  `{:02x}` なら、どのチャンネルも 0 で埋めて 2 桁にします。短い形にできるかどうかは
  `c % 17 == 0` でわかり、そのときの数字は `c / 17` です。

### 別解

先に確かめてから、`u8::from_str_radix` で変換する方法です。すべてのバイトが ASCII の 16 進数字だと
わかれば、どのバイト位置も文字の境界になり、符号が紛れ込むこともありません。

```rust
pub fn parse_hex_color(s: &str) -> Option<[u8; 3]> {
    let hex = s.strip_prefix('#')?;
    if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let width = match hex.len() {
        6 => 2,
        3 => 1,
        _ => return None,
    };
    let mut rgb = [0; 3];
    for (i, channel) in rgb.iter_mut().enumerate() {
        let value = u8::from_str_radix(&hex[i * width..(i + 1) * width], 16).ok()?;
        *channel = if width == 1 { value * 17 } else { value };
    }
    Some(rgb)
}
```

`to_hex` は 1 桁ずつ組み立てることもできます。`char::from_digit` は英字を小文字で返します。
短い形では、各チャンネルの下の桁だけを書きます。

```rust
pub fn to_hex(rgb: [u8; 3]) -> String {
    let short = rgb.iter().all(|&c| c >> 4 == c & 0xf);
    let digit = |n: u8| char::from_digit(u32::from(n), 16).unwrap();
    let mut out = String::from("#");
    for c in rgb {
        if !short {
            out.push(digit(c >> 4));
        }
        out.push(digit(c & 0xf));
    }
    out
}
```
