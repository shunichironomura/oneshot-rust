トークナイザは、次の文字を**消費せずに**見る必要があります。`42+3` の数字の並びは `+` で終わり、
その `+` が次のトークンになるからです。これをしてくれるのが `chars().peekable()` です。`peek` は次の要素を
見るだけで、`next_if(pred)` は `pred` が `true` を返したときだけ次の要素を取り出します。そのため
`while let Some((_, d)) = chars.next_if(...)` は数字の並びを読み、数字でない最初の文字の手前で止まります。
`->` も同じで、`-` の後に `>` が続いていれば `next_if` でそれを取り出します。

`peekable` の前に `enumerate` を挟むと、各文字に位置が付くので、自分で数えなくても `pos` が正しく
なります。`char_indices` が返すのは**バイト**単位の位置なので、U+3000 のような ASCII 以外の文字の後では
文字単位の位置とずれます。

数は、数字を `String` に集めて `parse::<u64>()` で変換しています。この文字列は空ではなく、ASCII の数字だけで、
値は `u64` に収まるので、変換は失敗しません。

エラーを見つけたらすぐに返せば（`return Err(...)`）、自然に最初のエラーが返ります。

詰まりやすい点：

- `chars.by_ref().take_while(...)` はぴったりの道具に見えますが、条件を満たさなかった最初の文字を
  消費して捨ててしまいます。`42+3` では `+` が消えます。
- `peek` は `&mut self` を取り、イテレータの中への参照を返します。
  `while let Some(d) = chars.peek() { chars.next(); digits.push(*d); }` では `next` を呼ぶ時点で
  `d` がまだ使われているので、借用チェッカーに拒否されます（E0499）。`Some(&d)` というパターンで
  値をコピーして取り出してください（`next` を別に呼ばずに済む `next_if` を使う手もあります）。
- `next_if_eq` は参照を取ります。`next_if_eq('>')` ではなく `next_if_eq(&'>')` です。
- `is_numeric` は `٣` のような ASCII 以外の数字も受け付け、`is_ascii_whitespace` は U+3000 を空白と
  みなしません。数には `is_ascii_digit` を、空白には `is_whitespace` を使います。

### 別解

`peek` と自前の位置カウンタを使い、値を 1 桁ずつ計算する方法です。`next` のたびに `pos += 1` が必要で、
忘れやすいところです。`enumerate` を使えばこれが要りません。

```rust
pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    let mut chars = src.chars().peekable();
    let mut pos = 0; // position of the next character `chars` yields
    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        let start = pos;
        pos += 1;
        if c.is_whitespace() {
            continue;
        }
        let token = match c {
            '0'..='9' => {
                let mut value = u64::from(c) - u64::from('0');
                // `&d` copies the peeked char, so `chars` is free to move on.
                while let Some(&d) = chars.peek() {
                    let Some(digit) = d.to_digit(10) else { break };
                    chars.next();
                    pos += 1;
                    value = value * 10 + u64::from(digit);
                }
                Token::Num(value)
            }
            '-' => {
                if chars.next_if_eq(&'>').is_some() {
                    pos += 1;
                    Token::Arrow
                } else {
                    Token::Op('-')
                }
            }
            '+' | '*' | '/' | '(' | ')' | '=' => Token::Op(c),
            _ => return Err(LexError::Unexpected { ch: c, pos: start }),
        };
        tokens.push(token);
    }
    Ok(tokens)
}
```

文字を `Vec<char>` に集めて添字で扱う方法もあります。添字がそのまま文字単位の位置になり、
先読みは `chars.get(i)` だけで済みます。

```rust
pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    let chars: Vec<char> = src.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let start = i;
        let c = chars[i];
        i += 1;
        let token = match c {
            _ if c.is_whitespace() => continue,
            '0'..='9' => {
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let digits: String = chars[start..i].iter().collect();
                Token::Num(digits.parse().expect("a run of digits that fits in a u64"))
            }
            '-' if chars.get(i) == Some(&'>') => {
                i += 1;
                Token::Arrow
            }
            '+' | '-' | '*' | '/' | '(' | ')' | '=' => Token::Op(c),
            _ => return Err(LexError::Unexpected { ch: c, pos: start }),
        };
        tokens.push(token);
    }
    Ok(tokens)
}
```
