1 変数 `x` の式を `Expr` の木で表します（列挙型はエディタに入っています）。いくつかの規則で式を簡単にする
`simplify` と、式の値を求める `eval` を実装してください。

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),                  // 定数
    Var,                       // 変数 x
    Add(Box<Expr>, Box<Expr>), // 左 + 右
    Mul(Box<Expr>, Box<Expr>), // 左 * 右
    Neg(Box<Expr>),            // -中身（符号反転）
}

pub fn simplify(e: Expr) -> Expr
pub fn eval(e: &Expr, x: i64) -> Option<i64>
```

以下では `Expr::` と `Box::new` を省略して書きます。たとえば `Add(Var, Num(1))` は
`Expr::Add(Box::new(Expr::Var), Box::new(Expr::Num(1)))` のことです。

`simplify` は下から上へ処理します。まずノードの子を簡単にし、それから（子を簡単にした後の）ノードを
次の表に従って書き換えます。`a` と `b` は数、`e` と `f` は任意の式を表します。

| ノード | 書き換え後 |
| --- | --- |
| `Add(Num(a), Num(b))` | `Num(a + b)` |
| `Add(e, Num(0))` または `Add(Num(0), e)` | `e` |
| `Add(e, Neg(f))` または `Add(Neg(f), e)`（`e == f` のとき） | `Num(0)` |
| `Mul(Num(a), Num(b))` | `Num(a * b)` |
| `Mul(e, Num(0))` または `Mul(Num(0), e)` | `Num(0)` |
| `Mul(e, Num(1))` または `Mul(Num(1), e)` | `e` |
| `Neg(Num(a))` | `Num(-a)` |
| `Neg(Neg(e))` | `e` |

- `a + b`、`a * b`、`-a` が `i64` の範囲を超える（オーバーフローする）ときは、その規則は
  適用しません。
- どの規則も当てはまらないノードは、子を簡単にしたうえでそのまま残します。`Num` と `Var` は
  変わりません。
- `e == f` は、`Expr` の `==` と同じように 2 つの部分木を厳密に比べます。項の並べ替えや
  組み替えは一切しません。`Mul(Var, Num(2))` と `Mul(Num(2), Var)` は別の式で、
  `Add(Add(Var, Num(1)), Num(2))` はそのまま残ります。
- 1 つのノードに複数の規則が当てはまるときは、どれも同じ結果になります。

`eval` は、`Var` を `x` としたときの `e` の値を返します。式の中の加算・乗算・符号反転のどれかが
`i64` の範囲を超えるときは `None` を返します。結果に影響しない部分（`0` と掛け合わせる因数など）で
超える場合も `None` です。

### 例

- `simplify(Add(Mul(Var, Num(1)), Num(0)))` は `Var` です。
- `simplify(Mul(Add(Num(2), Num(3)), Var))` は `Mul(Num(5), Var)` です。
- `simplify(Add(Mul(Var, Num(0)), Num(7)))` は `Num(7)` です（先に積が `Num(0)` になります）。
- `simplify(Neg(Num(i64::MIN)))` は `Neg(Num(i64::MIN))` です。`-i64::MIN` はオーバーフロー
  するからです。
- `simplify(Add(Var, Neg(Mul(Var, Num(1)))))` は `Num(0)` です。
- `eval(&Add(Mul(Var, Var), Num(1)), 3)` は `Some(10)`、`eval(&Neg(Var), i64::MIN)` は
  `None` です。
