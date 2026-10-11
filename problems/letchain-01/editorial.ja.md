let チェーン（Rust 2024）を使うと、1 つの `if` の中で `let` パターンと普通の条件を
`&&` でつなぐことができます。束縛した変数は、それより後ろの条件と本体で使えます。
「`'` で始まり、残りが `'` で終わり、その間に `'` がない」を、`if let` を入れ子にせず
1 つの平らな条件として書けます。チェーンのどこかが成り立たなければ条件全体が偽になり、
`classify` は次の規則に進みます。これがまさに「最初に当てはまる規則」です。`'it's:x'` は
クォートされたトークンではないので `Pair` になります。チェーンに使えるのは `&&` だけで、
`let` と `||` を組み合わせることはできません。

落とし穴は規則 2 です。`tok.starts_with('\'') && tok.ends_with('\'')` は、`'` 1 文字だけの
トークンでも真になります。開きと閉じのクォートが同じ文字になってしまうのです。このとき
`&tok[1..tok.len() - 1]` は `&tok[1..0]` で、終わりが始まりより前にある範囲はパニックします
（「byte range starts at 1 but ends at 0」）。`tok.trim_matches('\'')` はパニックしませんが、
両端の `'` を**すべて**取り除くので、`'` も `'''` も `Quoted("")` になってしまいます。
先に開きのクォートを取り除き、閉じのクォートは残りの部分だけで探しましょう。`strip_prefix` と
`strip_suffix` はパニックせずに `None` を返し、let チェーンなら 2 つ目の `let` で 1 つ目の束縛を
使えます。同じ理由で、`&tok[..1]` と比べるより `strip_prefix('\'')` のほうが安全です。
`&tok[..1]` は空文字列や、`é` のような複数バイトの文字で始まるトークンでパニックします。

`0..10` のような排他的範囲パターン（Rust 1.80 で安定化）を使うと、規則をそのまま書けます。
コンパイラは整数の範囲がすべての値を網羅しているかを確かめるので、`0..10`、`10..1000`、`1000..` なら
`_` のアームは要りません。最後のアームを `1000..u64::MAX` と書くとコンパイルが通りません。
排他的範囲には `u64::MAX` 自体が含まれないからです。

`split_once(':')` は最初の `:` で分け、`:` がなければ `None` を返すので、規則 3 にぴったりです。
同じチェーンで `!before.is_empty() && !after.is_empty()` を確かめれば、`":b"`、`"a:"`、`":"` は
`Word` に回ります。

`u8` の `a + b` は、オーバーフローするとデバッグビルド（と Miri）ではパニックし、リリースビルドでは
黙って値が一周します（ラップアラウンド）。`checked_add` なら代わりに `None` を返します。
スライスパターン `[a, b, ..]` は要素が 2 つ以上のスライスにだけマッチするので、
範囲外の添字になることもありません。

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
            if let Some(rest) = tok.strip_prefix('\'')
                && let Some(text) = rest.strip_suffix('\'')
                && !text.contains('\'') =>
        {
            Token::Quoted(text.to_string())
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

長さで `'` 1 文字だけのトークンを除いておけば、スライスしても構いません。2 バイト以上で `'` で始まり
`'` で終わるトークンなら、2 つのクォートは別の文字です。`'` は 1 バイトなので、`1` と `tok.len() - 1` は
文字の境界です。「間に `'` がない」は、クォートの数を数えて確かめています。

```rust
pub fn classify(tok: &str) -> Token {
    if let Ok(n) = tok.parse::<u64>() {
        return match n {
            0..10 => Token::Tiny(n),
            10..1000 => Token::Small(n),
            1000.. => Token::Large(n),
        };
    }
    if tok.len() >= 2
        && tok.starts_with('\'')
        && tok.ends_with('\'')
        && tok.matches('\'').count() == 2
    {
        return Token::Quoted(tok[1..tok.len() - 1].to_string());
    }
    if let Some((before, after)) = tok.split_once(':')
        && !before.is_empty()
        && !after.is_empty()
    {
        return Token::Pair(before.to_string(), after.to_string());
    }
    Token::Word(tok.to_string())
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
