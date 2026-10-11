let チェーン（Rust 2024）を使うと、1 つの `if` の中で `let` パターンと普通の条件を
`&&` でつなぐことができます。束縛した変数は、それより後ろの条件と本体で使えます。
「`#` で始まり、続く 6 バイトがすべて 16 進数字で、数として読める」を、`if let` を入れ子にせず
1 つの平らな条件として書けます。チェーンに使えるのは `&&` だけで、`let` と `||` を組み合わせることは
できません。

落とし穴は規則 2 です。`str::parse` と同じく `u32::from_str_radix` も先頭の `+` を受け付けます。
`"#+12345"` は `#` の後ろがちょうど 6 文字で、`u32::from_str_radix("+12345", 16)` は `Ok(0x12345)` を返します。
「16 進数として読める 6 文字」は「6 桁の 16 進数字」と同じではありません。先に各バイトを `is_ascii_hexdigit` で
確かめてください。確かめた後なら `from_str_radix` は失敗しないので、`let Ok(value) = ...` としてチェーンに
入れれば `unwrap` も要りません。`#` の確認には、`&tok[..1]` で切り出すのではなく `strip_prefix('#')` を
使いましょう。バイトの範囲が `é` のような複数バイトの文字の途中で終わると、パニックするからです。

`0..10` のような排他的範囲パターン（Rust 1.80 で安定化）を使うと、規則をそのまま書けます。
コンパイラは整数の範囲がすべての値を網羅しているかを確かめるので、`0..10`、`10..1000`、`1000..` なら
`_` のアームは要りません。最後のアームを `1000..u64::MAX` と書くとコンパイルが通りません。
排他的範囲には `u64::MAX` 自体が含まれないからです。

`split_once(':')` は最初の `:` で分け、`:` がなければ `None` を返すので、規則 3 にぴったりです。
同じチェーンで `!before.is_empty() && !after.is_empty()` を確かめれば、`":b"`、`"a:"`、`":"` は
`Word` に回ります。

`u8` の `a + b` は、オーバーフローするとデバッグビルド（と Miri）ではパニックし、リリースビルドでは
何も言わずに折り返します。`checked_add` なら代わりに `None` を返します。スライスパターン `[a, b, ..]` は
要素が 2 つ以上のスライスにだけマッチするので、範囲外のインデックスになることもありません。

### 別解

`parse` の結果に対する 1 つの `match` で、束縛付きの範囲パターンと `if let` ガード（Rust 1.95 で安定化）を
使う書き方です。ガード自体を let チェーンにでき、ガードで束縛した変数はそのアームで使えます。
アームは上から順に試されるので、`n @ ..1000` に来るのは 10 以上の数だけです。

```rust
pub fn classify(tok: &str) -> Token {
    match tok.parse::<u64>() {
        Ok(n @ ..10) => Token::Tiny(n),
        Ok(n @ ..1000) => Token::Small(n),
        Ok(n) => Token::Large(n),
        Err(_)
            if let Some(digits) = tok.strip_prefix('#')
                && digits.len() == 6
                && digits.bytes().all(|b| b.is_ascii_hexdigit())
                && let Ok(value) = u32::from_str_radix(digits, 16) =>
        {
            Token::Hex(value)
        }
        Err(_)
            if let Some((before, after)) = tok.split_once(':')
                && !before.is_empty()
                && !after.is_empty() =>
        {
            Token::Pair(before.to_string(), after.to_string())
        }
        Err(_) => Token::Word(tok.to_string()),
    }
}
```

`sum_first_two` は、let-else で 2 つの要素を取り出し、オーバーフローしない `u16` で足してから
`u8` に戻すこともできます。

```rust
pub fn sum_first_two(nums: &[u8]) -> Option<u8> {
    let [a, b, ..] = *nums else {
        return None;
    };
    u8::try_from(u16::from(a) + u16::from(b)).ok()
}
```

`Option` に対する `?` を使うこともできます。

```rust
pub fn sum_first_two(nums: &[u8]) -> Option<u8> {
    nums.first()?.checked_add(*nums.get(1)?)
}
```
