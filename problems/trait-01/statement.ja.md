トレイト `Shape` と 2 つの図形 `Rect`、`Circle` はエディタに宣言済みです。中身が書かれていない部分を実装してください。

```rust
pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
    fn describe(&self) -> String { /* 実装する */ }
}

pub struct Rect { pub width: f64, pub height: f64 }
pub struct Circle { pub radius: f64 }

pub fn total_area(shapes: &[&dyn Shape]) -> f64
```

- `Rect` の `area` は `width * height`、`Circle` の `area` は πr² です（`std::f64::consts::PI` を使います）。
- `name` は `Rect` なら `"rect"`、`Circle` なら `"circle"` です。
- `describe` は `"<name> with area <area>"` を返します。面積は小数点以下 2 桁に丸めます。`Shape` を実装した
  ほかの型でも使えるようにしてください。
- `total_area` はすべての図形の面積の合計を返します（空のスライスなら `0.0`）。

### 例

- `Rect { width: 2.0, height: 3.0 }.describe()` は `"rect with area 6.00"` です。
- `total_area(&[&Rect { width: 2.0, height: 3.0 }, &Circle { radius: 1.0 }])` は `6.0 + PI` です。
