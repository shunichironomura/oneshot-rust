`Guess` は値を非公開のフィールドに持つので、モジュールの外のコード（テストも含みます）は
`Guess { value: 0 }` と書けません。`Guess` は必ず `new` か `try_from` で作られ、どちらも範囲を
確かめるので、`Guess` を受け取った関数は値を確かめ直さずに信頼できます。フィールドを読むには
`value()` を使います。

2 つのコンストラクタは同じ確認を共有しています。`try_from` が問題を `GuessError` で報告し、
`new` はそれぞれのエラーを対応するパニックにします。`new` で `Guess::try_from(value).unwrap()`
（や `.expect(...)`）と書いてもパニックはしますが、メッセージは
``called `Result::unwrap()` on an `Err` value: TooSmall(0)`` のようになり、求められたものに
なりません。`panic!` は `format!` と同じようにメッセージを組み立てるので、`{value}` で数を
埋め込めます。2 つのメッセージの取り違えにも注意してください。「less than or equal to 100」は
値が**大きすぎる**ときのメッセージです。Rust Book の 11 章では、まさにこの 2 つのメッセージを
入れ替えて、`#[should_panic(expected = "...")]` がその誤りを見つける様子を示しています。

実装するのは `TryFrom` だけで十分です。標準ライブラリには
`impl<T, U> TryInto<U> for T where U: TryFrom<T>` という包括的な実装（ブランケット実装）が
あるので、`Guess: TryFrom<i32>` が成り立てば `value.try_into()` も使えます（`From` を実装すると
`Into` が使えるようになるのと同じです）。

### 別解

`assert!` は条件の後に、`panic!` と同じ形式のメッセージを取るので、`new` で範囲を直接確かめる
こともできます。`try_from` は模範解答のままです。

```rust
pub fn new(value: i32) -> Guess {
    assert!(value >= 1, "Guess value must be greater than or equal to 1, got {value}.");
    assert!(value <= 100, "Guess value must be less than or equal to 100, got {value}.");
    Guess { value }
}
```

`try_from` は値の範囲で `match` することもできます。`..=0`、`1..=100`、`101..` の 3 つの
パターンですべての `i32` を網羅しているので、`_` のアームがなくてもコンパイルが通ります。

```rust
fn try_from(value: i32) -> Result<Guess, GuessError> {
    match value {
        ..=0 => Err(GuessError::TooSmall(value)),
        1..=100 => Ok(Guess { value }),
        101.. => Err(GuessError::TooLarge(value)),
    }
}
```
