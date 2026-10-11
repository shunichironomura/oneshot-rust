どの関数もスライスのメソッド 1 つで書けて、添字を使ったループは要りません。

- `chunk_by`（Rust 1.77 で安定化）は、となり合う 2 つの要素 `a` と `b` ごとにクロージャを呼び、
  `false` が返ったところで新しいグループを始めます。`a == b` なら等しい値の連なり、`a < b` なら
  狭義単調増加の連なりになります。どのグループも `v` の空でない部分スライスなので `group[0]` は
  安全で、`v` が空ならグループは 1 つもできません。
- `windows(3)` はとなり合う 3 つの要素 `[v[i], v[i + 1], v[i + 2]]` を順に返します。`i` 番目の窓の
  真ん中は添字 `i + 1` なので、`+ 1` が必要です。長さが 3 未満のスライスには窓がありません。
- `chunks_exact(2)` はそろったペアだけを返し、`remainder()` は余り（空のスライスか要素 1 つ）を
  返します。`map` はイテレータを値で受け取るので、`remainder()` は `map` を呼ぶ**前に**
  取り出しておきます。後で呼ぶと「ムーブされた値の借用」のエラーになります。

`rising_runs` は `chunk_by` が返すグループを、コピーせずにそのまま返します。
ライフタイムの省略規則により、`fn rising_runs(v: &[i32]) -> Vec<&[i32]>` は
`fn rising_runs<'a>(v: &'a [i32]) -> Vec<&'a [i32]>` という意味になります。つまり結果のスライスは
`v` を借用します。`v.chunk_by(...)` が返すのは、ちょうどそういうスライスです。

落とし穴は、添字を使って自分でループを書いたときにあります。

- `for i in 1..v.len() - 1` は問題なさそうに見えますが、`v` が空だと `v.len() - 1` は `usize` の
  `0 - 1` です。ジャッジはデバッグビルドと同じくオーバーフロー検査を有効にしているので、
  `attempt to subtract with overflow` でパニックします。`runs` や `pairs` の `0..v.len() - 1` も
  同じです。
- ループの後で最後の連なりを追加すると、`v` が空のときに `[]` ではなく `[[]]` を返してしまいます。
- `b > a` の代わりに `b - a > 0` で比べると、`i32::MIN` と `i32::MAX` がとなり合ったときに
  オーバーフローします。値はそのまま比べてください。

### 別解

空のスライスをきちんと扱えば、添字のループでも書けます。`saturating_sub` はアンダーフローせずに
`0` で止まります。`runs` は `last_mut` で最後のグループを伸ばせば、添字なしで書けます。

```rust
pub fn runs(v: &[i32]) -> Vec<(i32, usize)> {
    let mut result: Vec<(i32, usize)> = Vec::new();
    for &x in v {
        match result.last_mut() {
            Some((value, len)) if *value == x => *len += 1,
            _ => result.push((x, 1)),
        }
    }
    result
}

pub fn rising_runs(v: &[i32]) -> Vec<&[i32]> {
    let mut result = Vec::new();
    if v.is_empty() {
        return result;
    }
    let mut start = 0;
    for i in 1..v.len() {
        if v[i] <= v[i - 1] {
            result.push(&v[start..i]);
            start = i;
        }
    }
    result.push(&v[start..]);
    result
}

pub fn local_peaks(v: &[i32]) -> Vec<usize> {
    (1..v.len().saturating_sub(1))
        .filter(|&i| v[i - 1] < v[i] && v[i] > v[i + 1])
        .collect()
}
```

ペアを取り出した後で `remainder()` を読みたいなら、イテレータを借用するだけの `by_ref()` を
通して回します。

```rust
pub fn pairs(v: &[i32]) -> (Vec<(i32, i32)>, Option<i32>) {
    let mut chunks = v.chunks_exact(2);
    let pairs = chunks.by_ref().map(|pair| (pair[0], pair[1])).collect();
    (pairs, chunks.remainder().first().copied())
}
```

`as_chunks`（Rust 1.88 で安定化）はスライスを固定長の配列の並びと残りに分けるので、配列パターンで
ペアを分解できます。

```rust
pub fn pairs(v: &[i32]) -> (Vec<(i32, i32)>, Option<i32>) {
    let (chunks, rest) = v.as_chunks::<2>();
    (chunks.iter().map(|&[a, b]| (a, b)).collect(), rest.first().copied())
}
```
