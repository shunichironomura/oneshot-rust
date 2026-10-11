次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

長方形を扱うコードが、いくつかのモジュールに分かれています。`shapes` には長方形の型 `Rect` があり、
その非公開の子モジュール `shapes::checks` には関数 `is_square` があります。非公開のモジュール `units` には、
長さをセンチメートル単位で持つタプル構造体 `Cm` があります。あなたのコードはモジュール `crate::user` です。
テストハーネスは同じクレートの別のモジュールで、`crate::user` の外にあります。ハーネスが使うパスは、
ちょうど次のとおりです。

```rust
crate::user::shapes::Rect::new(width: u32, height: u32) -> Rect
crate::user::shapes::Rect::area(&self) -> u32
crate::user::shapes::is_square(rect: &Rect) -> bool
crate::user::square(side: u32) -> Rect
crate::user::total_area(rects: &[Rect]) -> u32
```

- 長さはセンチメートル単位の整数で、`0` も使えます。`Rect::new(width, height)` はその大きさの長方形を作り、
  `area` は `width * height` を返します。
- `is_square` は、幅と高さが等しければ `true` を返します。`0` × `0` の長方形も正方形です。
- `square(side)` は `side` × `side` の長方形を返します。
- `total_area` は、スライスに入っている長方形の面積の合計を返します（空のスライスなら `0`）。
- 面積やその合計は、いつも `u32` に収まります。
- 変更してよいのは、可視性、パス、`use` 宣言だけです。モジュール、型、関数はどれも、移動、名前の変更、
  削除をせずに今の場所に置いたままにしてください。

### 例

- `Rect::new(2, 3).area()` は `6`、`square(4).area()` は `16` です。
- `total_area(&[Rect::new(1, 2), Rect::new(3, 3)])` は `11` です。
- `is_square(&square(2))` は `true`、`is_square(&Rect::new(1, 2))` は `false` です。
