Collect the words into a `Vec<&str>` and `match` on `words.as_slice()`. A **slice pattern** checks
the number of words and the literal words at the same time: `["move", x, y]` matches exactly three
words starting with `"move"` and binds the other two. Anything with extra or missing words falls
through to `_ => None` without any length checks.

- `rest @ ..` binds the remaining words as a sub-slice. The **guard** `if !rest.is_empty()` rejects a
  lone `say`, and `rest.join(" ")` rebuilds the message.
- `"quit" | "exit"` is an or-pattern inside the slice pattern.
- `Ok(n @ 1..=60)` matches only a parsed number in range and binds it to `n`.
- Inside a function returning `Option`, `x.parse().ok()?` turns a parse error into `None` at once.

### Alternatives

The `say` arm can require at least one word in the pattern itself, without a guard.

```rust
["say", first, rest @ ..] => {
    let mut text = first.to_string();
    for w in rest {
        text.push(' ');
        text.push_str(w);
    }
    Some(Command::Say(text))
}
```

The directions can also be written as separate arms, such as
`["go", "north"] => Some(Command::Move { x: 0, y: 1 })`. That is more arms, but each one reads as a
single rule.
