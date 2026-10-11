模範解答が持つのは、前から次に返す要素、ステップ、残りの要素数の 3 つです。`next` も `next_back` も
この 1 つの個数から 1 つずつ取るので、両端が行き違うことはありません。`next_back` 用の位置も要りません。
個数を 1 減らした後なら、最後の要素は前端より `remaining * step` だけ大きい値です。

**`ExactSizeIterator` は `size_hint` の上に成り立っています。** 必須のメソッドはありません。既定の `len()` は
`size_hint()` を呼び、下限と上限が一致することを `assert` で確かめます。一方、`Iterator` の既定の
`size_hint()` は `(0, None)` を返します。そのため、`size_hint` を実装せずに `impl ExactSizeIterator` を空で
書くと、最初の `len()` でパニックします。`len` だけを上書きしても足りません。`size_hint` は間違ったままで、
`skip` や `take` などのアダプタは自分の `len` を `size_hint` から計算するからです。

**オーバーフロー。** 範囲は `i64` 全体にわたることがあり、ステップは `u64` です。

- `end - start` は `2^64 - 1` になることがあり、`i64` ではオーバーフローしますが `u64` には収まります。
  `end.abs_diff(start)` で求められます。
- 要素数はこの距離をステップで割って切り上げた値です。`(distance + step - 1) / step` は大きな値で
  オーバーフローしますが、`u64::div_ceil` はオーバーフローしません。
- `step as i64` は `2^63` 以上のステップを負の数にしてしまいます。`i64::wrapping_add_unsigned`
  （または `checked_add_unsigned`）なら、キャストせずに `i64` に `u64` を足せます。
- 前端を最後の要素の先へ進めるとオーバーフローすることがあります。`new(i64::MAX - 5, i64::MAX, 4)` では、
  `i64::MAX - 1` の次の値は `i64::MAX` を超えます。この値は返さないので、模範解答ではここだけ回り込ませています。
  返す値はどれも範囲の要素なので、それを求める足し算が回り込むことはありません。

`new` は何よりも先に `step != 0` を確かめるので、ステップが 0 なら範囲が空でもパニックします。

### 別解

両端から取り出せて正確な長さを持つステップ付きのイテレータは std にもあります。`Range<usize>` に対する
`step_by` です。`start` からの距離 `0..end - start` をステップごとにたどり、それぞれを `start` に足します。
ステップが `0` なら `step_by` 自身がパニックします。

```rust
use std::iter::StepBy;
use std::ops::Range;

pub struct StepRange {
    start: i64,
    /// The distances of the remaining items from `start`.
    offsets: StepBy<Range<usize>>,
}

impl StepRange {
    pub fn new(start: i64, end: i64, step: u64) -> StepRange {
        // usize is 64 bits wide on the judge, so these conversions are lossless.
        let span = if start < end { end.abs_diff(start) as usize } else { 0 };
        // `step_by` panics if the step is 0.
        let offsets = (0..span).step_by(step as usize);
        StepRange { start, offsets }
    }

    fn item(&self, offset: usize) -> i64 {
        self.start.wrapping_add_unsigned(offset as u64)
    }
}

impl Iterator for StepRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.offsets.next().map(|offset| self.item(offset))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.offsets.size_hint()
    }
}

impl DoubleEndedIterator for StepRange {
    fn next_back(&mut self) -> Option<i64> {
        self.offsets.next_back().map(|offset| self.item(offset))
    }
}

impl ExactSizeIterator for StepRange {}
```

`Range<i64>` ではうまくいきません。`ExactSizeIterator` ではないので、それに対する `step_by` には
`next_back` がないからです。

`i128` なら途中の値がすべて収まるので、前端と後端の要素を持つ形でもオーバーフローを気にせずに書けます。
前端が後端を追い越したら範囲は空です。

```rust
pub struct StepRange {
    /// The next item from the front and from the back. The range is empty when `front > back`.
    front: i128,
    back: i128,
    step: i128,
}

impl StepRange {
    pub fn new(start: i64, end: i64, step: u64) -> StepRange {
        assert!(step != 0, "step must not be zero");
        let (start, end, step) = (i128::from(start), i128::from(end), i128::from(step));
        // In i128, no difference or sum of these values can overflow.
        let back = if start < end {
            start + (end - 1 - start) / step * step
        } else {
            start - 1
        };
        StepRange { front: start, back, step }
    }
}

impl Iterator for StepRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.front > self.back {
            return None;
        }
        let item = self.front;
        self.front += self.step;
        Some(item as i64)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.front > self.back {
            0
        } else {
            ((self.back - self.front) / self.step + 1) as usize
        };
        (n, Some(n))
    }
}

impl DoubleEndedIterator for StepRange {
    fn next_back(&mut self) -> Option<i64> {
        if self.front > self.back {
            return None;
        }
        let item = self.back;
        self.back -= self.step;
        Some(item as i64)
    }
}

impl ExactSizeIterator for StepRange {}
```
