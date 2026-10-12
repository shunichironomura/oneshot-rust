A command-line tool sorts the tokens it reads into kinds. Implement `classify`, which finds the kind
of one token. The `Token` enum is already in the editor.

```rust
#[derive(Debug, PartialEq)]
pub enum Token {
    Quoted(String),
    Pair(String, String),
    Word(String),
}

pub fn classify(tok: &str) -> Token
```

`classify(tok)` uses the first of these rules that applies. `tok` can be any string, and it is used
as it is: it is not trimmed.

1. If `tok` consists of an opening single quote `'`, then a `text` that contains no `'`, then a
   closing `'`, the token is `Quoted(text)`. The `text` is kept as it is. It may be empty (`''` is
   `Quoted("")`) and may contain any other character, `:` included.
2. If `tok` contains a `:`, split it at its **first** `:`. If the part before it (`before`) and the
   part after it (`after`) are both non-empty, the token is `Pair(before, after)`. The `:` itself
   belongs to neither part.
3. Otherwise, the token is `Word(tok)`. This includes the empty string.

### Examples

| Call | Result |
| --- | --- |
| `classify("'a:b'")` | `Quoted("a:b")` |
| `classify("'it's'")` | `Word("'it's'")` (a third `'`) |
| `classify("a:b:c")` | `Pair("a", "b:c")` |
| `classify("hello")` | `Word("hello")` |

`Pair("a", "b:c")` is short for `Token::Pair("a".to_string(), "b:c".to_string())`.
