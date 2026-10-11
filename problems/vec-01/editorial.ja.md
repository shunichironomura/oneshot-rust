`Job` は `Clone` ではないので、ジョブをリストから出すには、ムーブで取り出すしかありません。
`jobs.iter()` のループで見えるのは `&Job` なので、`out.push(*job)` は拒否されます（「cannot move out of
`*job` which is behind a shared reference」）。そのループの中で `jobs.remove(i)` を呼ぶのも、ループが
`jobs` を借用し続けているのでエラーになります（しかも、インデックスがずれていきます）。

`Vec::extract_if` は、まさに「条件に合う要素を取り出す」メソッドです。範囲内の各要素について順に 1 回ずつ
述語を呼び、`true` を返した要素を**値で**返し、残りの要素はその場に順序どおり残します。
記憶だけで書くと間違えやすい点が 2 つあります。最初の引数は範囲（ベクタ全体なら `..`）です。
また、イテレータは遅延評価なので、消費せずに捨てると何も取り除かれません（使われていない `ExtractIf` には
コンパイラが警告を出します）。

述語は `&mut Job` を受け取るので、`tick` は同じ走査の中で `ttl` を減らして判定できます。
`saturating_sub` を使ってください。`job.ttl - 1` は `ttl` が `0` のときオーバーフローでパニックします。
デバッグビルド（と Miri）は算術オーバーフローを検査するからです。

`dedup`、`dedup_by`、`dedup_by_key` が取り除くのは**連続した**重複だけなので、`[a, b, a, c, b]` は
そのまま残ります。代わりに、出てきたタイトルを `HashSet` に覚えておきます。ただし、
`jobs.retain(|job| seen.insert(job.title.as_str()))` はコンパイルが通りません。`retain` は各要素を
その呼び出しの間だけクロージャに貸すので、クロージャの外にある `seen` に `&str` をしまえないからです
（「borrowed data escapes outside of closure」）。所有権を持つ `String` をしまえば解決します。

`move_to_front` で `jobs.swap(0, i)` とすると、ほかのジョブの順序が崩れます。先頭部分 `jobs[..=i]` を右に 1 つ
回転すれば、ジョブ `i` が先頭に来て、ジョブ `0..i` は 1 つずつ後ろにずれます。

### 別解

`extract_if` を使わない方法です。`std::mem::take` でベクタ全体を取り出し（代わりに空のベクタが残ります）、
`partition` で分けてから、残りを戻します。

```rust
pub fn take_done(jobs: &mut Vec<Job>) -> Vec<Job> {
    let (done, rest) = std::mem::take(jobs).into_iter().partition(|job| job.done);
    *jobs = rest;
    done
}
```

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

`dedup_titles` にも `extract_if` が使えます。`count` がイテレータを消費します。

```rust
pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize {
    let mut seen = HashSet::new();
    jobs.extract_if(.., |job| !seen.insert(job.title.clone())).count()
}
```

タイトルを複製したくなければ、先に `seen` がタイトルを借用している間に残すジョブを決めておき、
後で残りを取り除きます。`retain` が始まる前に借用は終わっています。また、`retain` は要素を順に
1 回ずつ訪れます。

```rust
pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize {
    let mut seen = HashSet::new();
    let keep: Vec<bool> = jobs.iter().map(|job| seen.insert(job.title.as_str())).collect();
    let mut keep = keep.into_iter();
    let before = jobs.len();
    jobs.retain(|_| keep.next().unwrap());
    before - jobs.len()
}
```

`move_to_front` は、ジョブを取り除いてからインデックス 0 に挿入し直しても書けます。

```rust
pub fn move_to_front(jobs: &mut Vec<Job>, id: u32) -> bool {
    let Some(i) = jobs.iter().position(|job| job.id == id) else {
        return false;
    };
    let job = jobs.remove(i);
    jobs.insert(0, job);
    true
}
```
