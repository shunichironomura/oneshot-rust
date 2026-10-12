`Mon = 1` で最初の判別値（discriminant）を決めると、値を書いていない後続のバリアントは直前の値に 1 を
足した値になるので、`Sun` は 7 です。フィールドを持たない列挙型は `as` で整数にキャストできるので、
`number` は `self as u8` だけで書けます。ただしキャストは一方通行で、`n as Weekday` はコンパイルエラー
（non-primitive cast）になります。ほとんどの `u8` の値は曜日ではないからです。失敗しうる変換のために
あるのが `TryFrom` で、結果は `Result` になります。数で `match` するのがいちばん素直な書き方です。
同じ理由で、`next` も `(self as u8 + 1) as Weekday` とは書けません。`match` で 7 つの翌日を並べれば、
`Sun` から `Mon` へ戻ることもはっきり書けます。

標準のトレイトを実装すると、`Weekday` を既存の API で使えるようになります。`impl TryFrom<u8> for Weekday`
を書くと、`std` のブランケット実装によって `u8: TryInto<Weekday>` も実装されます。`TryFrom` と `TryInto` は
2021 エディションからプレリュードに入っているので、`use` なしで `0u8.try_into()` と書けます。同じように、
`impl FromStr` を書くと `"fri".parse::<Weekday>()` が使えるようになります。エラーの型の名前は、`TryFrom`
では `Error`、`FromStr` では `Err` と違うことに注意してください。

`from_str` の `s.to_ascii_lowercase()` は新しい `String` を返します。文字列リテラルのパターンは `String`
ではなく `&str` にマッチするので、`.as_str()` で `match` します。元の `s` はそのまま残っているので、エラーには
渡されたとおりの文字列を入れられます。`&str` は先に所有権のある `String` にする必要があるので、
`DayError::Unknown(s.to_string())` と書きます。

### 別解

表を使った変換です。曜日の配列を 1 つ用意し、`try_from` ではインデックス `n - 1` で引き、`from_str` では
名前の配列と並べて `eq_ignore_ascii_case` で比べます。`next` は番号を経由して書けます。`% 7` で `Sun` の 7 が
0 になるので、結果は必ず `1..=7` に入り、`unwrap` が失敗することはありません。

```rust
const ALL: [Weekday; 7] = [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

const NAMES: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

impl Weekday {
    pub fn next(self) -> Weekday {
        // 1..=6 は 2..=7 に、7（Sun）は 0 + 1 = 1（Mon）になる
        Weekday::try_from(self.number() % 7 + 1).unwrap()
    }
    // number は模範解答と同じ
}

impl TryFrom<u8> for Weekday {
    type Error = DayError;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            1..=7 => Ok(ALL[usize::from(n - 1)]),
            _ => Err(DayError::OutOfRange(n)),
        }
    }
}

impl FromStr for Weekday {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for (day, name) in ALL.into_iter().zip(NAMES) {
            if s.eq_ignore_ascii_case(name) {
                return Ok(day);
            }
        }
        Err(DayError::Unknown(s.to_string()))
    }
}
```
