`let パターン = 式 else { ... };`（let-else）は「この値を分解する。できなければこの行は飛ばす」という処理に
ぴったりです。パターンにマッチしなかったときは `else` ブロックが実行されますが、このブロックは
**発散**しなければなりません（`continue`、`break`、`return`、パニックなど）。そのまま後ろに抜けることは
できません。パターンで束縛した変数は文の後で使われるので、抜けてしまうと値がないからです。
`if let` と違って束縛した変数は文の後もスコープに残るので、チェックを 1 つ増やしてもネストは深くならず、
1 行増えるだけです。

「ちょうど 2 つの単語」は、集めた単語に対するスライスパターンで表せます。`words[..]` は `[&str]` で、
`[count, item]` は長さ 2 のスライスにだけマッチします。`&str` は `Copy` なので、`count` と `item` は
ベクタからコピーされた `&str` になります。`&words[..]` や `words.as_slice()` に同じパターンを使うと、
束縛されるのは `&&str`（ローカルな `Vec` の中への参照）です。すると `orders.push((count, item))` によって
`orders` が `Vec<(u32, &&str)>` になって返り値の型と合わなくなり、`*item` と書く必要があります。

返す `&str` が借用しているのは、ローカルな `Vec` ではなく `text` です。ライフタイムの省略規則により、
返り値は唯一の参照引数から借用したものになります。`lines`、`split_once`、`split_whitespace` はどれも
渡された文字列のスライスを返すので、単語のベクタを各反復の終わりに破棄しても、商品名は生き続けます。
`String` は必要ありません（関数の中で作った `String` のスライスを返そうとするとコンパイルが通りません）。

`str::parse::<u32>` は先頭の `+` や 0 を受け付け、`-` や `u32::MAX` より大きい数は拒否します。
`0` は問題なく解釈されるので、0 は別にチェックする必要があります。

`total_quantity` は `parse_orders` を再利用します。`.iter().filter(|(_, name)| name == item)` と書くと
`name` は `&&str` になり、`&str` の `item` と比べるとコンパイルが通りません（`&str == str` は
実装されていません）。`*name == item` と書くか、模範解答や下の別解のように組を値で受け取ってください。

### 別解

イテレータを使った書き方です。クロージャの中では `continue` を使えないので、`else` ブロックでは
`filter_map` のクロージャから `return None` します。`next()` で 3 つ目まで取り出して 3 つ目が `None` で
あることを確かめれば、`Vec` を作らずに「ちょうど 2 つの単語」を確認できます（タプルの要素は左から順に
評価されます）。範囲パターン `count @ 1..` を使うと、同じ `let` で `0` も弾けます。

```rust
pub fn parse_orders(text: &str) -> Vec<(u32, &str)> {
    text.lines()
        .filter_map(|line| {
            let code = line.split_once('#').map_or(line, |(code, _)| code);
            let mut words = code.split_whitespace();
            let (Some(count), Some(item), None) = (words.next(), words.next(), words.next()) else {
                return None;
            };
            let Ok(count @ 1..) = count.parse::<u32>() else {
                return None;
            };
            Some((count, item))
        })
        .collect()
}

pub fn total_quantity(text: &str, item: &str) -> u32 {
    parse_orders(text)
        .into_iter()
        .filter(|&(_, name)| name == item)
        .map(|(count, _)| count)
        .sum()
}
```

let チェーン（Rust 2024）を使えば、チェックを `&&` でつないで 1 つの `if let` にまとめられます。

```rust
pub fn parse_orders(text: &str) -> Vec<(u32, &str)> {
    let mut orders = Vec::new();
    for line in text.lines() {
        let code = line.split_once('#').map_or(line, |(code, _)| code);
        let words: Vec<&str> = code.split_whitespace().collect();
        if let [count, item] = words[..]
            && let Ok(count) = count.parse::<u32>()
            && count > 0
        {
            orders.push((count, item));
        }
    }
    orders
}
```
