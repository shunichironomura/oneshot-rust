次のコードはコンパイルが通り、手元で実行すると「それっぽく」動くこともあります。
しかし**未定義動作（UB）**を含んでおり、Miri で実行すると UB として検出されます。

4 つの小さな関数が、共用体（`union`）を通して値を読み替えています。

```rust
pub fn float_bits(x: f32) -> u32
pub fn byte_to_bool(b: u8) -> Option<bool>
pub fn widen(b: u8) -> u32
pub fn pack_header(kind: u8, len: u16) -> u32
```

- `float_bits` は `x` の IEEE 754 のビットパターン、つまり `x` のビットを `u32` として読んだ値を
  返します。
- `byte_to_bool` は、バイト `0` には `Some(false)`、`1` には `Some(true)`、それ以外のどのバイトにも
  `None` を返します。
- `widen` は `b` を同じ値の `u32` にして返します。
- `pack_header` はヘッダを 1 つの `u32` にまとめます。ビット 0〜7（下位 8 ビット）が `kind`、
  ビット 8〜15 が 0、ビット 16〜31 が `len` です。言い換えると、結果は `kind + len * 65536` です。

**どの**入力に対してもすべての関数が正しく動くように、UB を取り除いてください（`unsafe` や共用体を
使い続けてもかまいません。非公開の型は変更・削除してかまいません）。

### 例

- `float_bits(1.0)` は `0x3F80_0000`、`float_bits(-0.0)` は `0x8000_0000` です。
- `byte_to_bool(0)` は `Some(false)`、`byte_to_bool(1)` は `Some(true)` です。
- `widen(0xAB)` は `0xAB` です。
- `pack_header(0x12, 0x3456)` は `0x3456_0012` です。
