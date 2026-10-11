クロージャの 3 つのトレイトは段階的な関係になっています。どのクロージャも `FnOnce` を実装します（1 回は呼べます）。
キャプチャした値を外へムーブしないクロージャは `FnMut` も実装し、さらにキャプチャした値を変更もしないものは
`Fn` も実装します。境界の強さには一長一短があります。弱い境界ほど呼び出し側は多くのクロージャを渡せますが、
本体で `f` にできることは少なくなります。そこで、関数ごとに本体が必要とする最小限のものだけを要求します。

- `apply_n` は `f` を何度も呼ぶので、`FnOnce` では足りません。ハーネスはキャプチャを変更する
  `|| count += 2` を渡すので、`Fn` では厳しすぎます。残るのは `FnMut` です。`FnMut` を呼ぶには `f` を
  可変借用する必要があるので、`mut f` と書かないと本体が「cannot borrow `f` as mutable」（E0596）で失敗します。
- `consume` は `f` を 1 回しか呼ばないので `FnOnce` で足ります。ハーネスもまさにそれを必要としています。
  `move || name` は所有している `String` を手放すので 1 回しか実行できず、`FnMut` ではありません
  （それより強い境界では E0507 になります）。結果はある呼び出しでは `String`、別の呼び出しでは `Vec` なので、
  型引数 `T` を使います。
- `map_pair` は `f` を 2 回呼ぶので `FnOnce` は使えません（「use of moved value: `f`」）。ハーネスの番号付けの
  クロージャは `next` を更新するので `Fn` も使えません。入力と出力の型は異なる（`&str` から `String`、
  `i32` から `bool`）ので、型引数が `A` と `B` の 2 つ必要です。
- `call_shared` は最初から正しいシグネチャでした。本体は `g = &f` を通して `f` を呼びますが、共有参照 `&F` を
  呼べるのは `F: Fn` のときだけです。`FnMut` にすると、本体が「cannot borrow `*g` as mutable, as it is behind
  a `&` reference」で失敗します。ハーネスは `|| k + 1` のように変数をキャプチャするクロージャを渡すので、
  ただの `fn() -> u32` でもうまくいきません。
- `each_digit` は `f` を 2 回使います。上の桁のための再帰呼び出しに `f` を渡し、その後で最後の桁について
  `f` を呼びます。値で受け取ると `f` は再帰呼び出しにムーブされ、その後の呼び出しが「borrow of moved value:
  `f`」（E0382）で失敗します。ハーネスはクロージャへの `&mut` 参照を渡すので、それを受け取ります。
  `f: &mut impl FnMut(u64)` です。`&mut F` は `F` そのものと同じように呼べ（`mut f` は要りません）、
  `&mut` の引数が期待される場所に `f` を渡すと、ムーブではなく（`&mut *f` としての）再借用になるので、
  その後も `f` を使えます。クロージャは `Vec` に追加したり和を更新したりするので、境界はここでも `FnMut`
  です。コンパイラは `each_digit(n / 10, &mut f)` を提案しますが、これは本体の変更ですし、ジェネリックな
  関数ではそもそもうまくいきません。再帰の段ごとに新しい型 `&mut &mut ... F` で `each_digit` を呼ぶことに
  なり、rustc は「reached the recursion limit while instantiating」で諦めてしまいます。

### 別解

引数位置の `impl Trait` は型引数の略記です。名前を付けた型引数に `where` 節で境界を書いても、ここでは同じ意味になります。

```rust
pub fn apply_n<F>(mut f: F, n: usize)
where
    F: FnMut(),
{
    for _ in 0..n {
        f();
    }
}

pub fn consume<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
{
    f()
}

pub fn map_pair<A, B, F>(mut f: F, pair: (A, A)) -> (B, B)
where
    F: FnMut(A) -> B,
{
    (f(pair.0), f(pair.1))
}

pub fn call_shared<F>(f: F) -> u32
where
    F: Fn() -> u32,
{
    let g = &f;
    g() + g()
}

pub fn each_digit<F>(n: u64, f: &mut F)
where
    F: FnMut(u64),
{
    if n >= 10 {
        each_digit(n / 10, f);
    }
    f(n % 10);
}
```

`each_digit` はトレイトオブジェクトを受け取ることもできます。その場合、どのクロージャに対しても
コンパイルされる関数は 1 つだけになり、`f` の呼び出しはどれも vtable を経由します。

```rust
pub fn each_digit(n: u64, f: &mut dyn FnMut(u64)) {
    if n >= 10 {
        each_digit(n / 10, f);
    }
    f(n % 10);
}
```
