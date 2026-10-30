信号機を表す列挙型 `Light` が用意されています（エディタに入っています）。3 つのメソッドを実装してください。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
    Flashing(u32),
}

impl Light {
    pub fn next(self) -> Light;
    pub fn duration_secs(self) -> u32;
    pub fn time_until_green(self) -> u32;
}
```

信号機は普段、`Green` → `Yellow` → `Red` → `Green` の順に変わります。交通量の少ない夜間などは、黄色の点滅に
切り替えられることがあります。この切り替えは外部の操作で行われるので、`Red`・`Yellow`・`Green` の
どれも `next` で `Flashing` には変わりません。点滅は決まった回数で終わり、そのあと通常の動作に戻ります。
点滅からいきなり赤にはせず、一度 `Yellow` を挟んでから `Red` に変わります。

`Flashing(n)` は黄色の点滅 1 回を表し、`n` は今回の点滅の後に残っている点滅の回数です。`n` には任意の値が
与えられます。たとえば `Flashing(2)` からは、今回を含めて 3 回点滅します。

- `next` は次の状態を返します。`Flashing(n)` は `Flashing(n - 1)` に、`Flashing(0)` は `Yellow` になります。
- `duration_secs` はその状態が続く秒数を返します。`Red` は 30、`Yellow` は 5、`Green` は 25、`Flashing` はどれも 1 です。
- `time_until_green` は、今の状態が始まってから `Green` になるまでの秒数を返します。すでに `Green` なら `0` です。

### 例

- `Light::Green.next()` は `Light::Yellow` です。
- `Light::Flashing(2).next()` は `Light::Flashing(1)`、`Light::Flashing(0).next()` は `Light::Yellow` です。
- `Light::Yellow.time_until_green()` は `35` です（黄 5 秒、赤 30 秒）。
- `Light::Flashing(0).time_until_green()` は `36` です（点滅 1 秒、黄 5 秒、赤 30 秒）。
