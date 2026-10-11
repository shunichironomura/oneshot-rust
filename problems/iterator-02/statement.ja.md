`StepRange` は `start` から `step` ずつ増えていき、`end` の手前で止まる範囲です。両端から取り出すことができ、
残りの要素数もいつでもわかります。これを実装してください。

```rust
pub struct StepRange { /* 自由に定義 */ }

impl StepRange {
    pub fn new(start: i64, end: i64, step: u64) -> StepRange;
}

impl Iterator for StepRange {
    type Item = i64;
    // ...
}

impl DoubleEndedIterator for StepRange { /* ... */ }

impl ExactSizeIterator for StepRange {}
```

`StepRange` のフィールドと実装は自由に書いてください（ひな形はエディタに入っています）。

- 要素は `start`、`start + step`、`start + 2 * step`、… のうち `end` より小さいものです（`end` 自体は
  要素になりません）。要素は正確な整数で、桁あふれして回り込むことはありません。`start >= end` なら
  範囲は空です。
- `next` は前から昇順に要素を返します。`next_back` は後ろから、最大の要素から順に小さいほうへ返します。
- `next` と `next_back` は混ぜて呼べます。どちらの端から取り出しても、各要素はちょうど 1 回だけ返されます。
  すべての要素を返し終えたら、どちらのメソッドも `None` を返し、その後も `None` を返し続けます。
- `len()` はまだ返していない要素の個数です。`size_hint()` は正確で、いつ呼んでも `(len, Some(len))` を
  返します。
- `step` が `0` なら `new` はパニックします。範囲が空でもパニックします。それ以外でパニックしてはいけません。
- `i64::MIN`、`i64::MAX`、`u64::MAX` も含め、どの `start`、`end`、`step` でも動かなければなりません。
  要素は最大で `u64::MAX` 個にもなるので、必要になったときに計算してください。`new`、`next`、
  `next_back`、`size_hint` はどれも定数時間で終わらなければなりません。ジャッジは 64 ビットの
  ターゲットなので、`usize` は 64 ビットです。

### 例

```rust
StepRange::new(-10, 10, 7).collect::<Vec<_>>()       // [-10, -3, 4]
StepRange::new(-10, 10, 7).rev().collect::<Vec<_>>() // [4, -3, -10]

let mut r = StepRange::new(-10, 10, 7);
r.next() // Some(-10)
r.len()  // 2

StepRange::new(i64::MAX - 5, i64::MAX, 4).collect::<Vec<_>>()    // [i64::MAX - 5, i64::MAX - 1]
StepRange::new(i64::MIN, i64::MAX, u64::MAX).collect::<Vec<_>>() // [i64::MIN]
```
