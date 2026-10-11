A command-line tool sorts the tokens it reads into kinds. Implement `classify`, which finds the kind
of one token, and `sum_first_two`, a small helper that adds two bytes without overflowing. The
`Token` enum is already in the editor.

```rust
#[derive(Debug, PartialEq)]
pub enum Token {
    Tiny(u64),
    Small(u64),
    Large(u64),
    Quoted(String),
    Pair(String, String),
    Word(String),
}

pub fn classify(tok: &str) -> Token
pub fn sum_first_two(nums: &[u8]) -> Option<u8>
```

`classify(tok)` uses the first of these rules that applies. `tok` can be any string, and it is used
as it is: it is not trimmed.

1. If `tok.parse::<u64>()` succeeds with a value `n`, the token is a number: `Tiny(n)` if `n < 10`,
   `Small(n)` if `10 <= n < 1000`, and `Large(n)` if `n >= 1000`. This accepts a leading `+` and
   leading zeros (`"+7"` and `"007"` are both 7), but not `"-7"`, `" 7"`, `"7.0"`, or a number
   larger than `u64::MAX`.
2. If `tok` consists of an opening single quote `'`, then a `text` that contains no `'`, then a
   closing `'`, the token is `Quoted(text)`. The `text` is kept as it is. It may be empty (`''` is
   `Quoted("")`) and may contain any other character, `:` included.
3. If `tok` contains a `:`, split it at its **first** `:`. If the part before it and the part after
   it are both non-empty, the token is `Pair(before, after)`. The `:` itself belongs to neither
   part.
4. Otherwise, the token is `Word(tok)`. This includes the empty string.

`sum_first_two(nums)` returns `Some(a + b)`, where `a` and `b` are the first two elements of `nums`.
It returns `None` if `nums` has fewer than two elements, or if `a + b` is larger than `u8::MAX`
(255). The remaining elements are ignored.

### Examples

| Call | Result |
| --- | --- |
| `classify("7")` | `Tiny(7)` |
| `classify("+250")` | `Small(250)` |
| `classify("'a:b'")` | `Quoted("a:b")` |
| `classify("'it's'")` | `Word("'it's'")` (a third `'`) |
| `classify("a:b:c")` | `Pair("a", "b:c")` |
| `sum_first_two(&[200, 55, 9])` | `Some(255)` |
| `sum_first_two(&[200, 56])` | `None` (`256` does not fit in a `u8`) |

`Pair("a", "b:c")` is short for `Token::Pair("a".to_string(), "b:c".to_string())`.
