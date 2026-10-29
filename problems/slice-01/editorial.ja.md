`chunk_by`（Rust 1.77 で安定化）は、となり合う 2 つの要素 `a` と `b` ごとにクロージャを呼び、
`false` が返ったところで新しいグループを始めます。`a < b` なら、グループはちょうど狭義単調増加の
連なりになります。どのグループも `v` の空でない部分スライスで、`v` が空ならグループは 1 つもできません。

`rising_runs` は `chunk_by` が返すグループを、コピーせずにそのまま返します。
ライフタイムの省略規則により、`fn rising_runs(v: &[i32]) -> Vec<&[i32]>` は
`fn rising_runs<'a>(v: &'a [i32]) -> Vec<&'a [i32]>` という意味になります。つまり結果のスライスは
`v` を借用します。`v.chunk_by(...)` が返すのは、ちょうどそういうスライスです。

落とし穴は、添字を使って自分でループを書いたときにあります。

- `for i in 0..v.len() - 1` は問題なさそうに見えますが、`v` が空だと `v.len() - 1` は `usize` の
  `0 - 1` です。ジャッジはデバッグビルドと同じくオーバーフロー検査を有効にしているので、
  `attempt to subtract with overflow` でパニックします。
- ループの後で最後の連なりを追加すると、`v` が空のときに `[]` ではなく `[[]]` を返してしまいます。
- `b > a` の代わりに `b - a > 0` で比べると、`i32::MIN` と `i32::MAX` がとなり合ったときに
  オーバーフローします。値はそのまま比べてください。

### 別解

空のスライスをきちんと扱えば、添字のループでも書けます。

```rust
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
```
