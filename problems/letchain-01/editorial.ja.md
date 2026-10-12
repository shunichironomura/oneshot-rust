let チェーン（Rust 2024）を使うと、1 つの `if` の中で `let` パターンと普通の条件を
`&&` でつなぐことができます。束縛した変数は、それより後ろの条件と本体で使えます。
「`'` で始まり、残りが `'` で終わり、その間に `'` がない」を、`if let` を入れ子にせず
1 つの平らな条件として書けます。チェーンのどこかが成り立たなければ条件全体が偽になり、
`classify` は次の規則に進みます。これがまさに「最初に当てはまる規則」です。`'it's:x'` は
クォートされたトークンではないので `Pair` になります。チェーンに使えるのは `&&` だけで、
`let` と `||` を組み合わせることはできません。

落とし穴は規則 1 です。`tok.starts_with('\'') && tok.ends_with('\'')` は、`'` 1 文字だけの
トークンでも真になります。開きと閉じのクォートが同じ文字になってしまうのです。このとき
`&tok[1..tok.len() - 1]` は `&tok[1..0]` で、終わりが始まりより前にある範囲はパニックします
（「byte range starts at 1 but ends at 0」）。`tok.trim_matches('\'')` はパニックしませんが、
両端の `'` を**すべて**取り除くので、`'` も `'''` も `Quoted("")` になってしまいます。
先に開きのクォートを取り除き、閉じのクォートは残りの部分だけで探しましょう。`strip_prefix` と
`strip_suffix` はパニックせずに `None` を返し、let チェーンなら 2 つ目の `let` で 1 つ目の束縛を
使えます。同じ理由で、`&tok[..1]` と比べるより `strip_prefix('\'')` のほうが安全です。
`&tok[..1]` は空文字列や、`é` のような複数バイトの文字で始まるトークンでパニックします。

`split_once(':')` は最初の `:` で分け、`:` がなければ `None` を返すので、規則 2 にぴったりです。
同じチェーンで `!before.is_empty() && !after.is_empty()` を確かめれば、`":b"`、`"a:"`、`":"` は
`Word` に回ります。

### 別解

長さで `'` 1 文字だけのトークンを除いておけば、スライスしても構いません。2 バイト以上で `'` で始まり
`'` で終わるトークンなら、2 つのクォートは別の文字です。`'` は 1 バイトなので、`1` と `tok.len() - 1` は
文字の境界です。「間に `'` がない」は、クォートの数を数えて確かめています。

```rust
pub fn classify(tok: &str) -> Token {
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
