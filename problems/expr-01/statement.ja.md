1 変数 `x` の式を `Expr` の木で表します（列挙型はエディタに入っています）。いくつかの規則で式を簡単にする
`simplify` を実装してください。

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
```

以下では `Expr::` と `Box::new` を省略して書きます。たとえば `Add(Var, Num(1))` は
`Expr::Add(Box::new(Expr::Var), Box::new(Expr::Num(1)))` のことです。

`simplify` は下から上へ処理します。まずノードの子を簡単にし、それから（子を簡単にした後の）ノードを
次の表に従って書き換えます。`e` と `f` は任意の式を表します。

| ノード | 書き換え後 |
| --- | --- |
| `Add(e, Num(0))` または `Add(Num(0), e)` | `e` |
| `Add(e, Neg(f))` または `Add(Neg(f), e)`（`e == f` のとき） | `Num(0)` |
| `Mul(e, Num(0))` または `Mul(Num(0), e)` | `Num(0)` |
| `Mul(e, Num(1))` または `Mul(Num(1), e)` | `e` |
| `Neg(Neg(e))` | `e` |

- どの規則も当てはまらないノードは、子を簡単にしたうえでそのまま残します。`Num` と `Var` は
  変わりません。数どうしの足し算や掛け算もしません。`Add(Num(2), Num(3))` はそのまま残ります。
- `e == f` は、`Expr` の `==` と同じように 2 つの部分木を厳密に比べます。項の並べ替えや
  組み替えは一切しません。`Mul(Var, Num(2))` と `Mul(Num(2), Var)` は別の式で、
  `Add(Add(Var, Num(1)), Num(2))` はそのまま残ります。
- 1 つのノードに複数の行が当てはまるときは、表の上にある行を使います。

### 例

- `simplify(Add(Mul(Var, Num(1)), Num(0)))` は `Var` です。
- `simplify(Neg(Neg(Add(Var, Num(0)))))` は `Var` です。
- `simplify(Add(Mul(Var, Num(0)), Num(7)))` は `Num(7)` です（先に積が `Num(0)` になります）。
- `simplify(Add(Var, Neg(Mul(Var, Num(1)))))` は `Num(0)` です。
