The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

A `Profile` has a username and, optionally, a nickname and an email address.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
}

impl Profile {
    pub fn display_name(&self) -> &str;
    pub fn email_domain(&self) -> Option<&str>;
    pub fn take_nickname(&mut self) -> Option<String>;
}
```

- `display_name` returns the nickname if there is one, and the username otherwise. An empty nickname
  `Some("")` still counts as a nickname, so the result is then `""`.
- `email_domain` returns the part of the email after its **first** `@`. That part may be empty:
  `"abc@"` gives `Some("")`. It returns `None` if there is no email or the email contains no `@`.
- `take_nickname` removes the nickname and returns it. Afterwards the profile has no nickname
  (`nickname` is `None`). If there was no nickname, it returns `None`.
- The methods change nothing else: `username` and `email` stay as they are.

Do not change the struct or the signatures.

### Examples

- If the email is `"a@b.c"`, `email_domain()` returns `Some("b.c")`.
- For the username `"alice"` with the nickname `"ace"`, `display_name()` is `"ace"`. After that,
  `take_nickname()` returns `Some("ace")`, and `display_name()` falls back to `"alice"`.
