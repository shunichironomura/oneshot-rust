温度は 2 つのニュータイプ `Celsius`（摂氏）と `Fahrenheit`（華氏）で表します（どちらもエディタに入っています）。
2 つの間の変換と、測定値を扱う 3 つの関数を実装してください。

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

impl From<Fahrenheit> for Celsius { /* 実装する */ }
impl From<Celsius> for Fahrenheit { /* 実装する */ }

pub fn is_freezing(t: impl Into<Celsius>) -> bool
pub fn average<T: Into<Celsius>>(readings: Vec<T>) -> Option<Celsius>
pub fn warmest(readings: &[Fahrenheit]) -> Option<Celsius>
```

- `Celsius::from(Fahrenheit(f))` は `Celsius((f - 32.0) * 5.0 / 9.0)`、
  `Fahrenheit::from(Celsius(c))` は `Fahrenheit(c * 9.0 / 5.0 + 32.0)` です。
  テストでは `.into()` による変換も、両方向で行います。
- `is_freezing` は、温度を摂氏にした値が `0.0` 以下なら `true` を返します。
  `Celsius` と `Fahrenheit` の両方の値で呼ばれます。
- `average` は、すべての測定値を摂氏に変換し、その合計を個数で割った値を返します。
  `readings` が空なら `None` を返します。`Vec<Celsius>` と `Vec<Fahrenheit>` の両方で呼ばれます。
- `warmest` は、最も高い測定値を摂氏に変換して返します。`readings` が空なら `None` を返します。
- NaN の値はありません。また、テストで使う値はどれも誤差なく変換できるので、結果は `==` で比べます。

### 例

```rust
Celsius::from(Fahrenheit(212.0))                 // Celsius(100.0)
let f: Fahrenheit = Celsius(-40.0).into();       // Fahrenheit(-40.0)

is_freezing(Fahrenheit(32.0))                    // true
is_freezing(Celsius(0.5))                        // false

average(vec![Fahrenheit(32.0), Fahrenheit(212.0)])  // Some(Celsius(50.0))
average(vec![Celsius(10.0), Celsius(20.0)])         // Some(Celsius(15.0))
warmest(&[Fahrenheit(50.0), Fahrenheit(212.0), Fahrenheit(32.0)])  // Some(Celsius(100.0))
```
