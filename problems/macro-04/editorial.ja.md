`matches_any!` のマッチャー `$($p:pat)|+` は、`|` で区切られた 1 つ以上のパターンという意味です。
Rust 2021 より前は `pat` フラグメントが最上位の `|` にマッチしなかったので、`matches!` などのマクロは
こう書かれていました。2021 エディションからは、`pat` は or パターン全体にマッチします。`1 | 4..=6` で
1 つの `pat` です。そのため、`pat` フラグメントの直後の `|` はどちらの意味か決められず、rustc は定義そのものを
拒否します（「`$p:pat` is followed by `|`, which is not allowed for `pat` fragments」）。`pat` の後に
置けるのは `=>`、`,`、`=`、`if`、`in` だけです。パターン全体を 1 つの `$p:pat` で受け取れば直ります。
今の `matches!` もそう定義されています。

### 別解

`pat_param` は `pat` の以前の意味（最上位に `|` を含まないパターン）を保っています。後ろに `|` を
置けるので、フラグメント指定子を変えるだけで元の繰り返しが使えます。

```rust
macro_rules! matches_any {
    ($value:expr, $($p:pat_param)|+) => {
        match $value {
            $($p)|+ => true,
            _ => false,
        }
    };
}
pub(crate) use matches_any;
```
