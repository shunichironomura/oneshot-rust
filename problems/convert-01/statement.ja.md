曜日を表す列挙型 `Weekday` と、変換の失敗を表す `DayError` が用意されています（どちらもエディタに入っています）。
2 つのメソッドと、`u8` と文字列からの変換を実装してください。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday { Mon = 1, Tue, Wed, Thu, Fri, Sat, Sun }

#[derive(Debug, PartialEq, Eq)]
pub enum DayError {
    OutOfRange(u8),
    Unknown(String),
}

impl Weekday {
    pub fn number(self) -> u8;
    pub fn next(self) -> Weekday;
}

impl TryFrom<u8> for Weekday { /* type Error = DayError; */ }
impl std::str::FromStr for Weekday { /* type Err = DayError; */ }
```

- `number` は曜日の番号を返します。`Mon` が 1、`Tue` が 2、…、`Sun` が 7 です。
- `next` は翌日の曜日を返します。曜日は一周するので、`Weekday::Sun.next()` は `Weekday::Mon` です。
- `Weekday::try_from(n)` は 1 〜 7 の数を `Mon` 〜 `Sun` に変換します（`number` の逆）。
  それ以外の `n` は `Err(DayError::OutOfRange(n))` になります。
- `from_str`（`s.parse::<Weekday>()` から使われます）が受け付けるのは、英語の 3 文字の略称
  `Mon`、`Tue`、`Wed`、`Thu`、`Fri`、`Sat`、`Sun` だけです。ASCII の大文字・小文字は区別しません
  （`"mon"`、`"MON"`、`"mOn"` はどれも `Mon` になります）。前後の空白は取り除かず、`"Monday"` のような
  略さない名前も受け付けません。それ以外の文字列は `Err(DayError::Unknown(..))` になり、中身は
  渡された文字列そのもの（大文字・小文字や空白も元のまま）です。

### 例

- `Weekday::Thu.number()` は `4`、`Weekday::Fri.next()` は `Weekday::Sat` です。
- `Weekday::try_from(7)` は `Ok(Weekday::Sun)` です。`let r: Result<Weekday, _> = 0u8.try_into();` とすると、
  `r` は `Err(DayError::OutOfRange(0))` です。
- `"fri".parse::<Weekday>()` は `Ok(Weekday::Fri)`、`"Funday".parse::<Weekday>()` は
  `Err(DayError::Unknown("Funday".to_string()))` です。
