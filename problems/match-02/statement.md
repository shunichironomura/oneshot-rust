Parse one line typed by the player of a text adventure into a `Command`. Return `None` for anything
that is not a valid command.

```rust
#[derive(Debug, PartialEq)]
pub enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Wait(u32),
    Quit,
}

pub fn parse_command(line: &str) -> Option<Command>
```

The line is split into words on whitespace. Words are case-sensitive.

| Words | Command |
| --- | --- |
| `move X Y` (`X` and `Y` are `i32`) | `Move { x: X, y: Y }` |
| `go north` / `go south` / `go east` / `go west` | `Move` by one step: `(0, 1)` / `(0, -1)` / `(1, 0)` / `(-1, 0)` |
| `say` followed by **one or more** words | `Say` with those words joined by single spaces |
| `wait` | `Wait(1)` |
| `wait N` (`N` is from 1 to 60) | `Wait(N)` |
| `quit` or `exit` | `Quit` |

Anything else, including extra or missing words, is `None`.

### Examples

| Input | Output |
| --- | --- |
| `"move 3 -2"` | `Some(Move { x: 3, y: -2 })` |
| `"go west"` | `Some(Move { x: -1, y: 0 })` |
| `"say  hello   world "` | `Some(Say("hello world"))` |
| `"wait"` | `Some(Wait(1))` |
| `"wait 61"` | `None` |
| `"exit"` | `Some(Quit)` |
| `"dance"` | `None` |
