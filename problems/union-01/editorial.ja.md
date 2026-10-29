共用体のフィールドを読むと、共用体が持っているバイト列をそのフィールドの型として読み替えます。
これが健全なのは、そのバイト列がその型の**有効な値**になっている場合だけです。整数はどんなビットパターンでも
有効ですが、未初期化のバイトは許されません。未初期化のメモリを整数として読むのは UB です。
「何かわからない数」になるのではありません。

`Header` は `#[repr(C)]` なので、`len` はオフセット 2 にそろえられ、オフセット 1 のバイトは
**パディング**になります。先に `bits` を 0 にしても役に立ちません。`packed.header = Header { .. }` は
`Header` の値を丸ごと書き込みますが、値はパディングを持たないので、書き込むとそのバイトは再び
未初期化になります。その後 `bits` を読むと、そのバイトを読んでしまいます。代わりにシフトで数を組み立てます。

模範解答には `unsafe` は要りません。シフトや `u32::from` はターゲットのバイト順にも依存しませんが、
以下に示す共用体を使う `pack_header` は、ターゲットがリトルエンディアンであることに頼っています。
共用体を使う実装でも、`len.to_le()` を格納し、結果を `u32::from_le` で変換すれば、両方のバイト順に
対応できます。

### 別解

読み出しが初期化済みのバイトだけを見るようにすれば、共用体は残せます。`Header` のフィールドを
1 つずつ書き込みます。1 つのフィールドへの書き込みはそのフィールドのバイトにしか触れないので、
0 にしたパディングのバイトは 0 のままです（この `pack_header` が正しいのは、ジャッジの x86_64 のような
リトルエンディアンのターゲットだけです）。

```rust
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
