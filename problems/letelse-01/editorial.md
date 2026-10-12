`let PATTERN = EXPR else { ... };` (let-else) fits "take this value apart, or skip this line". When
the pattern does not match, the `else` block runs, and it must **diverge**: `continue`, `break`,
`return` or a panic. It cannot just fall through, because the variables bound by the pattern are
used after the statement and would have no value. Unlike with `if let`, those variables stay in
scope after the statement, so each check adds one line instead of one level of nesting.

"Exactly two words" is a slice pattern on the collected words. `words[..]` is a `[&str]`, and
`[count, item]` matches only a slice of length 2. Since `&str` is `Copy`, `count` and `item` are
`&str`s copied out of the vector. With `&words[..]` or `words.as_slice()`, the same pattern binds
`&&str`s (references into the local `Vec`). `orders.push((count, item))` then makes `orders` a
`Vec<(u32, &&str)>`, which does not match the return type, and you need `*item`.

The returned `&str`s borrow `text`, not the local `Vec`. By lifetime elision, the output borrows
from the only reference parameter, and `lines` and `split_whitespace` both return
slices of the string they are given. So the vector of words can be dropped at the end of each
iteration while the items live on. No `String` is needed (and returning slices of a `String` created
inside the function would not compile).

`str::parse::<u32>` accepts a leading `+` and leading zeros, and rejects `-` and numbers above
`u32::MAX`. It parses `0` without complaint, so zero needs its own check.

### Alternatives

An iterator version. `continue` cannot be used inside a closure, so the `else` blocks
`return None` from the `filter_map` closure instead. Taking three words with `next()` and requiring
the third to be `None` checks "exactly two words" without a `Vec` (the elements of a tuple are
evaluated from left to right). The range pattern `count @ 1..` rejects `0` in the same `let`.

```rust
pub fn parse_orders(text: &str) -> Vec<(u32, &str)> {
    text.lines()
        .filter_map(|line| {
            let mut words = line.split_whitespace();
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
```

With let chains (Rust 2024), the checks can be joined with `&&` in a single `if let`.

```rust
pub fn parse_orders(text: &str) -> Vec<(u32, &str)> {
    let mut orders = Vec::new();
    for line in text.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
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
