`const fn` の中で使えるのは Rust の一部だけです。`for` ループは使えません。`for` は
`IntoIterator::into_iter` と `Iterator::next` を呼びますが、これらはトレイトのメソッドで
`const` ではないからです（エラー E0015）。同じ理由で、`bytes.iter().map(...).sum()` のような
イテレータのチェーン、クロージャの呼び出し、`Ord::max`、`std::array::from_fn`、`?` 演算子も
使えません。それでも、`while` と `loop`、`if` と `match`、算術演算、`.len()`、配列やスライスの
添字アクセス、ほかの `const fn` の呼び出し（再帰も含む）が使えれば十分です。ループはすべて、
添字を自分で進める `while` ループにします。

`first_primes` は `[0; N]` から始めて（ジェネリックな `N` でも書けます）、その場で埋めていきます。
ローカルな配列の要素への代入は `const fn` の中でもできます。書き込む前に `count < N` を確かめて
ください。最初に `primes[0] = 2` としてしまうと、`N` が `0` のときに範囲外アクセスになります。

定数評価の中で起きたエラーは、実行時のパニックにはなりません。`const` アイテムを計算している
途中のオーバーフロー、範囲外アクセス、ゼロ除算は**コンパイルエラー**になります。ここで
やりがちなのは、`is_prime` の `d * d <= n` です。`4_294_967_291` のような `u32::MAX` に近い素数では
`d` が `65536`（奇数だけを試す場合は `65537`）に達し、`d * d` が `u32` でオーバーフローします。
`d <= n / d` なら、オーバーフローせずに同じ条件を調べられます。また、コンパイラは時間が
かかりすぎる定数評価を打ち切ります（`long_running_const_eval` リント）。そのため、`n` 未満の
すべての約数候補を試す実装や、剰余ではなく引き算を繰り返す `gcd` は、大きな入力でコンパイルに
失敗します。

`weighted_sum` では、`(i as u32 + 1) * bytes[i] as u32` のように、掛ける前に両方を `u32` に
変換します。`(i as u8 + 1) * bytes[i]` のように `u8` のまま掛けると、オーバーフローします。
`b"ab"` のようなバイト文字列リテラルの型は `&[u8; 2]` で、`const` アイテムの中でも `&[u8]` に
型強制されます。

### 別解

`const fn` では再帰も使えるので、`gcd` はユークリッドの互除法の定義どおりに書けます。

```rust
pub const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
```

`u32::isqrt` は `const fn` です。上限を先に一度だけ計算しておけば、オーバーフローも起きません。

```rust
pub const fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let limit = n.isqrt();
    let mut d = 2;
    while d <= limit {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
```

`const fn` ではスライスパターンも使えます。`weighted_sum` は、添字を使う代わりに `while let` で
スライスの先頭から 1 バイトずつ取り出しても書けます。

```rust
pub const fn weighted_sum(bytes: &[u8]) -> u32 {
    let mut rest = bytes;
    let mut weight = 1;
    let mut sum = 0;
    while let [first, tail @ ..] = rest {
        sum += weight * *first as u32;
        weight += 1;
        rest = tail;
    }
    sum
}
```
