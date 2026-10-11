曜日を表す列挙型 `Weekday` と、変換の失敗を表す `DayError` が用意されています（どちらもエディタに入っています）。
3 つのメソッド、`u8` と文字列からの変換、`parse_schedule` を実装してください。

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
    pub fn is_weekend(self) -> bool;
}

impl TryFrom<u8> for Weekday { /* type Error = DayError; */ }
impl std::str::FromStr for Weekday { /* type Err = DayError; */ }

pub fn parse_schedule(s: &str) -> Result<Vec<Weekday>, DayError>
```

- `number` は曜日の番号を返します。`Mon` が 1、`Tue` が 2、…、`Sun` が 7 です。
- `next` は翌日の曜日を返します。曜日は一周するので、`Weekday::Sun.next()` は `Weekday::Mon` です。
- `is_weekend` は `Sat` と `Sun` で `true`、それ以外の曜日で `false` を返します。
- `Weekday::try_from(n)` は 1 〜 7 の数を `Mon` 〜 `Sun` に変換します（`number` の逆）。
  それ以外の `n` は `Err(DayError::OutOfRange(n))` になります。
- `from_str`（`s.parse::<Weekday>()` から使われます）が受け付けるのは、英語の 3 文字の略称
  `Mon`、`Tue`、`Wed`、`Thu`、`Fri`、`Sat`、`Sun` だけです。ASCII の大文字・小文字は区別しません
  （`"mon"`、`"MON"`、`"mOn"` はどれも `Mon` になります）。前後の空白は取り除かず、`"Monday"` のような
  略さない名前も受け付けません。それ以外の文字列は `Err(DayError::Unknown(..))` になり、中身は
  渡された文字列そのもの（大文字・小文字や空白も元のまま）です。
- `parse_schedule` はカンマ区切りの曜日の並びを読み取ります。
  - `s` が空か、空白文字だけなら、結果は空のベクタの `Ok` です。
  - そうでなければ、`s` をすべての `,` で区切ります。区切った各部分は、前後の空白を
    （`str::trim` と同じように）取り除いてから `from_str` で解釈します。
    結果はすべての曜日を順番どおりに並べたもので、重複もそのまま残します。
  - 解釈できない部分があれば、そのうち**最初の**部分のエラー、つまり前後の空白を取り除いた部分を持つ
    `DayError::Unknown` を返します。空の部分（`"mon,,fri"` や `"mon,"` など）もエラーで、
    `Unknown("")` になります。

### 例

- `Weekday::Thu.number()` は `4`、`Weekday::Fri.next()` は `Weekday::Sat`、
  `Weekday::Sat.is_weekend()` は `true` です。
- `Weekday::try_from(7)` は `Ok(Weekday::Sun)` です。`let r: Result<Weekday, _> = 0u8.try_into();` とすると、
  `r` は `Err(DayError::OutOfRange(0))` です。
- `"fri".parse::<Weekday>()` は `Ok(Weekday::Fri)`、`"Funday".parse::<Weekday>()` は
  `Err(DayError::Unknown("Funday".to_string()))` です。
- `parse_schedule("mon, FRI")` は `Ok(vec![Weekday::Mon, Weekday::Fri])`、
  `parse_schedule("mon,xyz")` は `Err(DayError::Unknown("xyz".to_string()))` です。
