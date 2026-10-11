`&` を書いていないパターンが参照にマッチすると、コンパイラはその参照を読み飛ばし、**デフォルトの束縛モード**を
`move` から `ref` に切り替えます。そこから下の束縛はすべて借用になります。これが「マッチエルゴノミクス」です。
`&Option<String>` に `Some(s)` をマッチさせると `s` は `&String` になり、`for Player { name, score } in players`
では 2 つの束縛がどちらも参照になります。コードで `*score` と書いているのはこのためです。

Rust 2024 では、デフォルトの束縛モードが `move` でなくなった場所に `ref`・`mut`・`&` を書くことが禁止されました。
壊れている関数は、それぞれこのどれかに引っかかっています。

- `ref`（`name_len`、`leader`）：束縛はもともと借用になるので冗長で、今はエラーです。消すだけで直ります。
- `mut`（`bonus_total`）：2021 では `mut` がこっそりモードを `move` に戻していたので、`(name, mut s)` の
  `s` はコピーされた `u32`、`name` は `&String` のままでした。`mut` を消すだけだと `s` は `&u32` になり、
  `s += 5` はコンパイルできません。`*s += 5` は共有参照を通した書き込みなので、これも許されません。
  外側の `&` を自分で書きましょう。`&(ref name, mut s)` は参照パターンで参照そのものにマッチするので、
  モードは `move` のままで、`ref` も `mut` もまた書けるようになります。`let mut s = *s;` で値をコピーして
  取り出す方法でも構いません。
- `&`（`rounds_over`）：`filter` は各要素を**参照で**渡すので、クロージャが受け取るのは `&(usize, &u32)` です。
  パターン `(_, &s)` はこの外側の参照を読み飛ばしており、読み飛ばした参照より下の `&` は今はエラーです。
  `|&(_, &s)|` と書けば 2 つの参照を両方とも明示できます。逆に `&` を消すと `s` は `&&u32` になり、
  `s > par` はコンパイルできません（`**s > par` なら通ります）。

`initials` は最初から正しいコードです。`|&(c, _)|` は `&` から始まるので、パターンは**完全に明示的**
（読み飛ばす参照がない）で、`c` は `char` になります。ルールは「クロージャの引数に `&` を書かない」ではなく、
「パターンが読み飛ばした参照より下に `ref`・`mut`・`&` を書かない」です。同様に、`.map(|(round, _)| round)`
が受け取るのは値としての `(usize, &u32)` なので、ここでは何も読み飛ばしていません。

`cargo fix --edition` もコンパイラの提案と同じ直し方をします。模範解答も同じです。冗長な `ref` は消し、
`mut` や `&` のために `move` モードが必要な場所では、パターンを完全に明示的な形にします。

### 別解

短いパターンのままにして、本体でコピーや参照外しをする方法です。

```rust
pub fn bonus_total(scores: &[(String, u32)]) -> u32 {
    let mut total = 0;
    for (name, s) in scores {
        let mut s = *s;
        if name.len() > 3 {
            s += 5;
        }
        total += s;
    }
    total
}

pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize> {
    scores
        .iter()
        .enumerate()
        .filter(|(_, s)| **s > par)
        .map(|(round, _)| round)
        .collect()
}
```

`ref` の場合も、パターンを完全に明示的にすれば直ります。束縛の型が変わることに注意してください。
`&Player { ref name, score }` では `score` はコピーされた `u32` になるので、`*` を外す必要があります。

```rust
pub fn name_len(nick: &Option<String>) -> usize {
    match nick {
        &Some(ref s) => s.len(),
        &None => 0,
    }
}

pub fn leader(players: &[Player]) -> Option<&str> {
    let mut best: Option<(&str, u32)> = None;
    for &Player { ref name, score } in players {
        let better = match best {
            Some((_, top)) => score > top,
            None => true,
        };
        if better {
            best = Some((name, score));
        }
    }
    best.map(|(name, _)| name)
}
```

イテレータアダプタを使う方法です。`filter_map` は各要素を値で受け取るので、`(round, &s)` は参照を
読み飛ばしておらず、そこでは `&` を書けます。

```rust
pub fn bonus_total(scores: &[(String, u32)]) -> u32 {
    scores
        .iter()
        .map(|(name, s)| if name.len() > 3 { s + 5 } else { *s })
        .sum()
}

pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize> {
    scores
        .iter()
        .enumerate()
        .filter_map(|(round, &s)| (s > par).then_some(round))
        .collect()
}
```
