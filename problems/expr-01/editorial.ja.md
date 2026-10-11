安定版の Rust には box パターンがないので、パターンで `Box` の中身を直接調べることはできません。
フィールドの型は `Box<Expr>` なので、`Expr::Add(Expr::Num(a), _)` は型エラーになります。
模範解答では、ノードを分解して先に子を簡単にし（下から上への順序のためにもともと必要です）、
その結果である**所有権を持った** 2 つの値のタプルに対して `match` します。`*l` は箱 `l` から `Expr` を
ムーブして取り出します。`*` で中身をムーブして取り出せるのは `Box` だけで、取り出した後の箱は使えません。
`simplify(*inner)` の後に `Expr::Neg(inner)` と書くと「use of moved value」になります。そのため新しいノードは
`Box::new` で作ります。同じ理由で、`match e` で `e` から箱をムーブした後は、`e` をそのまま返すこともできません。

間違えやすい点です。

- or パターンでは、どの選択肢でも同じ名前を同じ型で束縛しなければなりません。
  `(e, Expr::Num(0)) | (Expr::Num(0), e)` は通りますが、`(e, Expr::Num(0)) | (Expr::Num(0), _)` は
  コンパイルエラーです。
- `(e, Expr::Neg(f))` の `f` は `Box<Expr>` です。`e == f` は `Expr` と `Box<Expr>` の比較になり、
  コンパイルが通りません。`e == *f` なら通ります。ガードは束縛した値を借用するだけなので、比較で
  何かがムーブされることはありません。or パターンのある選択肢でガードが失敗すると、次の選択肢が試されます。
- `a + b`、`a * b`、`-a` は、ジャッジでは（普通のデバッグビルドと同じく）オーバーフローするとパニックします。
  `checked_add`、`checked_mul`、`checked_neg` を使いましょう。`if let` ガード（Rust 1.95 で安定化）を使えば、
  オーバーフローしたときはそのアームにマッチしないだけです。後のアームにもそのような 2 つの数に
  マッチするものはないので、最後のアームでノードがそのまま組み立て直されます。
- `Expr::Neg(e) => *e`：内側の `e` は箱なので、中の `Expr` を取り出して返す必要があります。
- 規則は子を簡単にした後に適用します。先に規則を調べてから子を簡単にすると、
  `Add(Mul(Var, Num(0)), Num(7))` を取りこぼします。積が `Num(0)` になるのは簡単にする途中だからです。

`eval` では、オペランドの計算が失敗した時点で `?` が `None` を返し、演算そのものがオーバーフローしたときは
`checked_*` が `None` を返します。因数が `0` のときに先に `Some(0)` を返すのは誤りです。もう一方の因数が
オーバーフローするかもしれないからです。

### 別解

`if let` ガードを使わずに、アームの中でオーバーフローを処理して、ノードを手で組み立て直す方法です。

```rust
pub fn simplify(e: Expr) -> Expr {
    match e {
        Expr::Add(l, r) => match (simplify(*l), simplify(*r)) {
            (Expr::Num(a), Expr::Num(b)) => match a.checked_add(b) {
                Some(sum) => Expr::Num(sum),
                None => Expr::Add(Box::new(Expr::Num(a)), Box::new(Expr::Num(b))),
            },
            (e, Expr::Num(0)) | (Expr::Num(0), e) => e,
            (e, Expr::Neg(f)) | (Expr::Neg(f), e) if e == *f => Expr::Num(0),
            (l, r) => Expr::Add(Box::new(l), Box::new(r)),
        },
        Expr::Mul(l, r) => match (simplify(*l), simplify(*r)) {
            (Expr::Num(a), Expr::Num(b)) => match a.checked_mul(b) {
                Some(product) => Expr::Num(product),
                None => Expr::Mul(Box::new(Expr::Num(a)), Box::new(Expr::Num(b))),
            },
            (_, Expr::Num(0)) | (Expr::Num(0), _) => Expr::Num(0),
            (e, Expr::Num(1)) | (Expr::Num(1), e) => e,
            (l, r) => Expr::Mul(Box::new(l), Box::new(r)),
        },
        Expr::Neg(inner) => match simplify(*inner) {
            Expr::Num(a) => match a.checked_neg() {
                Some(negated) => Expr::Num(negated),
                None => Expr::Neg(Box::new(Expr::Num(a))),
            },
            Expr::Neg(e) => *e,
            e => Expr::Neg(Box::new(e)),
        },
        leaf => leaf,
    }
}
```

箱を再利用することもできます。`*l = simplify(*l)` は子を箱からムーブして取り出し、簡単にした子を
同じ箱に戻します。その後の `match` では子を参照越しに調べるだけにして、結果に応じて子を 1 つ取り出すか
（`*l`）、2 つの箱をそのまま使います（`Expr::Add(l, r)`）。こうすると、そのまま残るノードでは新しい
メモリ確保が要りません。`Add` のアームは次のとおりです（`Mul` と `Neg` も同じ考え方で書けます）。

```rust
Expr::Add(mut l, mut r) => {
    *l = simplify(*l);
    *r = simplify(*r);
    match (&*l, &*r) {
        (Expr::Num(a), Expr::Num(b)) if let Some(sum) = a.checked_add(*b) => {
            Expr::Num(sum)
        }
        (_, Expr::Num(0)) => *l,
        (Expr::Num(0), _) => *r,
        (e, Expr::Neg(f)) | (Expr::Neg(f), e) if *e == **f => Expr::Num(0),
        _ => Expr::Add(l, r),
    }
}
```
