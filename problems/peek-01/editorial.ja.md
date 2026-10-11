トークナイザは、次の文字を**消費せずに**見る必要があります。`42+y` の数字の並びは `+` で終わり、
その `+` が次のトークンになるからです。これをしてくれるのが `chars().peekable()` です。`peek` は次の要素を
見るだけで、`next_if(pred)` は `pred` が `true` を返したときだけ次の要素を取り出します。そのため
`while let Some((_, d)) = chars.next_if(...)` は数字の並びを読み、数字でない最初の文字の手前で止まります。
`->` も同じで、`-` の後に `>` が続いていれば `next_if` でそれを取り出します。

`peekable` の前に `enumerate` を挟むと、各文字に位置が付くので、自分で数えなくても `pos` が正しく
なります。`char_indices` が返すのは**バイト**単位の位置なので、U+3000 のような ASCII 以外の文字の後では
文字単位の位置とずれます。

数は、数字を `String` に集めて `parse::<u64>()` で変換しています。この文字列は空ではなく ASCII の数字だけ
なので、起こりうるエラーは値が大きすぎることだけです。`value * 10 + digit` を直接計算すると、
デバッグビルド（と Miri）ではオーバーフローでパニックします。代わりに `checked_mul` と `checked_add` を
使います。数の桁数に上限はないので、`u128` のような大きい型にしても解決しません。

エラーを見つけたらすぐに返せば（`?` や `return Err(...)`）、自然に最初のエラーが返ります。

詰まりやすい点：

- `chars.by_ref().take_while(...)` はぴったりの道具に見えますが、条件を満たさなかった最初の文字を
  消費して捨ててしまいます。`42+y` では `+` が消えます。
- `peek` は `&mut self` を取り、イテレータの中への参照を返します。
  `while let Some(d) = chars.peek() { chars.next(); name.push(*d); }` では `next` を呼ぶ時点で
  `d` がまだ使われているので、借用チェッカーに拒否されます（E0499）。`Some(&d)` というパターンで
  値をコピーして取り出してください（`next` を別に呼ばずに済む `next_if` を使う手もあります）。
- `next_if_eq` は参照を取ります。`next_if_eq('>')` ではなく `next_if_eq(&'>')` です。
- `is_alphanumeric`、`is_alphabetic`、`is_numeric` は ASCII 以外の英字や数字も受け付け、
  `is_ascii_whitespace` は U+3000 を空白とみなしません。トークンには `is_ascii_*` のメソッドを、
  空白には `is_whitespace` を使います。

### 別解

`peek` と自前の位置カウンタを使い、`checked_mul` と `checked_add` で値を計算する方法です。
`next` のたびに `pos += 1` が必要で、忘れやすいところです。`enumerate` を使えばこれが要りません。

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
                    value = value
                        .checked_mul(10)
                        .and_then(|v| v.checked_add(u64::from(digit)))
                        .ok_or(LexError::Overflow { pos: start })?;
                }
                Token::Num(value)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut name = String::from(c);
                while let Some(&d) = chars.peek() {
                    if !(d.is_ascii_alphanumeric() || d == '_') {
                        break;
                    }
                    name.push(d);
                    chars.next();
                    pos += 1;
                }
                Token::Ident(name)
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
                let value = digits.parse().map_err(|_| LexError::Overflow { pos: start })?;
                Token::Num(value)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                Token::Ident(chars[start..i].iter().collect())
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
