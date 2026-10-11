共用体のフィールドを読むと、共用体が持っているバイト列をそのフィールドの型として読み替えます。
これが健全なのは、そのバイト列がその型の**有効な値**になっている場合だけです。整数や浮動小数点数は
どんなビットパターンでも有効ですが、未初期化のバイトは許されません。`bool` は `0` と `1` しか
許されません。4 つの関数を順に見ていきます。

- `float_bits` はもともと健全です。4 バイトすべてが `f32` として書き込まれており、どんなビットパターンも
  `u32` として有効だからです。これは `f32::to_bits` と同じ処理で、慣用的にはそちらを使います。
  `x as u32` に置き換えてはいけません。`as` は**数値**を変換するので（`1.0` は `1` になる）、
  ビットは保たれません。
- `byte_to_bool` は `b` を確かめる**前に** `flag` を読んでいます。`b = 2` のとき、この読み出しで
  `2` を持つ `bool` ができてしまいます。不正な値を作った時点で UB であり、後でその値を使うかどうかは
  関係ありません。この読み出しが許されるのは `b` が 0 か 1 のときだけなので、コンパイラが
  「`b <= 1` は常に成り立つ」と結論してチェックを消してしまうことさえあります。バイトで `match` すれば、
  `unsafe` はまったく要りません。
- `widen` は 1 バイトを書き込んで 4 バイトを読んでいます。残りの 3 バイトは一度も書き込まれておらず、
  未初期化のメモリを整数として読むのは UB です。「何かわからない数」になるのではありません。
  数値の変換は `u32::from(b)`（または、ここでは何も失わない `b as u32`）で書けます。
- `pack_header` がいちばんわかりにくいところです。`Header` は `#[repr(C)]` なので、`len` は
  オフセット 2 にそろえられ、1 バイト目は**パディング**になります。先に `bits` を 0 にしても
  役に立ちません。`packed.header = Header { .. }` は `Header` の値を丸ごと書き込みますが、値は
  パディングを持たないので、書き込むと 1 バイト目は再び未初期化になります。その後 `bits` を読むと、
  そのバイトを読んでしまいます。代わりにシフトで数を組み立てます。

模範解答では `unsafe` を使っていません。シフトや `u32::from` はターゲットのバイト順にも依存しませんが、
共用体を使う `widen` や `pack_header` は、ターゲットがリトルエンディアンであることに頼っています。

### 別解

どの読み出しも有効で初期化済みのバイトだけを見るようにすれば、共用体は残せます。`bool` はチェックの
後で読み、`small` を書く前に `big` を 0 にし、`Header` のフィールドは 1 つずつ書き込みます。
1 つのフィールドへの書き込みはそのフィールドのバイトにしか触れないので、0 にしたパディングのバイトは
0 のままです。`float_bits` は元のままです（この `widen` と `pack_header` が正しいのは、ジャッジの
x86_64 のようなリトルエンディアンのターゲットだけです）。

```rust
pub fn byte_to_bool(b: u8) -> Option<bool> {
    if b <= 1 {
        // SAFETY: 0 and 1 are valid `bool`s.
        Some(unsafe { ByteBool { byte: b }.flag })
    } else {
        None
    }
}

pub fn widen(b: u8) -> u32 {
    let mut w = Widen { big: 0 };
    w.small = b;
    // SAFETY: all four bytes are initialized: `b` and three zeros.
    unsafe { w.big }
}

pub fn pack_header(kind: u8, len: u16) -> u32 {
    let mut packed = HeaderBits { bits: 0 };
    packed.header.kind = kind;
    packed.header.len = len;
    // SAFETY: the padding byte has not been written since it was zeroed.
    unsafe { packed.bits }
}
```

パディングをなくして `HeaderBits` を残す方法もあります。`kind` と `len` の間に明示的なフィールド
`pad: u8` を加え、`Header { kind, pad: 0, len }` を作ります。

`pack_header` は、共用体を使わずにバイト列から組み立てることもできます。

```rust
pub fn pack_header(kind: u8, len: u16) -> u32 {
    let [lo, hi] = len.to_le_bytes();
    u32::from_le_bytes([kind, 0, lo, hi])
}
```
