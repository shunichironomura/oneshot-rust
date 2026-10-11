数当てゲームの予想は、1 から 100 までの数でなければなりません。予想を使う場所ごとに確かめ直す
代わりに、有効な値しか持てない型を作りましょう。`Guess` は値を非公開のフィールドに持つので、
作る方法は下の関数だけになり、どの `Guess` も有効な値を持ちます。構造体 `Guess` と列挙型
`GuessError` が用意されています（エディタに入っています）。メソッド、`TryFrom` による変換、
`parse_guess` を実装してください。

```rust
#[derive(Debug, PartialEq)]
pub struct Guess {
    value: i32,
}

#[derive(Debug, PartialEq)]
pub enum GuessError {
    TooSmall(i32),
    TooLarge(i32),
}

impl Guess {
    pub fn new(value: i32) -> Guess;
    pub fn value(&self) -> i32;
    pub fn compare(&self, secret: i32) -> Ordering;
}

impl TryFrom<i32> for Guess {
    type Error = GuessError;
    fn try_from(value: i32) -> Result<Guess, GuessError>;
}

pub fn parse_guess(input: &str) -> Result<Guess, String>
```

- 1 以上 100 以下の値を有効な値とします。
- `Guess::new(value)` は、`value` が有効ならその値を持つ `Guess` を返します。有効でなければ、
  次のどちらかのメッセージでパニックします。`{value}` は渡された値を 10 進数で書いたものです。
  テストはパニックを捕まえて、メッセージが完全に一致するかを調べます。
  - `value` が 1 より小さいとき：`Guess value must be greater than or equal to 1, got {value}.`
  - `value` が 100 より大きいとき：`Guess value must be less than or equal to 100, got {value}.`
- `value()` は予想が持つ値を返します。
- `compare(secret)` は、予想を `secret` と比べた結果を返します。予想が `secret` より小さければ
  `Ordering::Less`、等しければ `Ordering::Equal`、大きければ `Ordering::Greater` です。
  `secret` は、1 から 100 の範囲外も含めて、どの `i32` でもかまいません。
- `Guess::try_from(value)`（したがって `value.try_into()` も）は、パニックせずに同じ確認を
  します。`value` が有効なら予想を `Ok` で返し、1 より小さければ
  `Err(GuessError::TooSmall(value))`、100 より大きければ `Err(GuessError::TooLarge(value))` を
  返します。
- `parse_guess(input)` は、プレイヤーが入力した予想を読み取ります。
  - まず、`str::trim` と同じように `input` の前後の空白を取り除きます。
  - 残りが `str::parse::<i32>` で受け付けられなければ、結果は `"not a number"` の `Err` です。
    空文字列、`"4.5"` や `"4 2"` のような文字列、`"3000000000"` のように `i32` に収まらない数も
    これに当たります。
  - 数ではあっても有効な値でなければ、結果は `"out of range"` の `Err` です。
  - それ以外の場合、結果はその予想の `Ok` です。

### 例

```rust
Guess::new(42).value()      // 42
Guess::new(30).compare(50)  // Ordering::Less（予想の 30 は正解の 50 より小さい）
Guess::try_from(100)        // 値が 100 の予想の Ok
Guess::try_from(0)          // Err(GuessError::TooSmall(0))

Guess::new(0)    // パニック："Guess value must be greater than or equal to 1, got 0."
Guess::new(200)  // パニック："Guess value must be less than or equal to 100, got 200."

parse_guess(" 42 ")  // 値が 42 の予想の Ok
parse_guess("+7")    // 値が 7 の予想の Ok
parse_guess("0")     // Err("out of range".to_string())
parse_guess("abc")   // Err("not a number".to_string())
```
