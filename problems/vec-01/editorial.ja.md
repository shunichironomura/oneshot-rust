`Job` は `Clone` ではないので、ジョブをリストから出すには、ムーブで取り出すしかありません。
`jobs.iter()` のループの中で `jobs.remove(i)` を呼ぶと、ループが `jobs` を借用し続けているのでエラーになります
（しかも、インデックスがずれていきます）。

`Vec::extract_if` は、まさに「条件に合う要素を取り出す」メソッドです。範囲内の各要素について順に 1 回ずつ
述語を呼び、`true` を返した要素を**値で**返し、残りの要素はその場に順序どおり残します。
記憶だけで書くと間違えやすい点が 2 つあります。最初の引数は範囲（ベクタ全体なら `..`）です。
また、イテレータは遅延評価なので、消費せずに捨てると何も取り除かれません（使われていない `ExtractIf` には
コンパイラが警告を出します）。

述語は `&mut Job` を受け取るので、`tick` は同じ走査の中で `ttl` を減らして判定できます。
`saturating_sub` を使ってください。`job.ttl - 1` は `ttl` が `0` のときオーバーフローでパニックします。
デバッグビルド（と Miri）は算術オーバーフローを検査するからです。

### 別解

`retain_mut` ならクロージャが `&mut Job` を受け取れます。取り除いた `id` は、クロージャの外にあるベクタに
`push` できます。

```rust
pub fn tick(jobs: &mut Vec<Job>) -> Vec<u32> {
    let mut removed = Vec::new();
    jobs.retain_mut(|job| {
        job.ttl = job.ttl.saturating_sub(1);
        let keep = !job.done && job.ttl > 0;
        if !keep {
            removed.push(job.id);
        }
        keep
    });
    removed
}
```
