`dedup`、`dedup_by`、`dedup_by_key` が取り除くのは**連続した**重複だけなので、`[a, b, a, c, b]` は
そのまま残ります。代わりに、出てきたタイトルを `HashSet` に覚えておきます。`HashSet::insert` は、すでにある
タイトルには `false` を返します。それがちょうど取り除くジョブで、`retain` は残りのジョブを順序どおりに残します。

ただし、`jobs.retain(|job| seen.insert(job.title.as_str()))` はコンパイルが通りません。`retain` は各要素を
その呼び出しの間だけクロージャに貸すので、クロージャの外にある `seen` に `&str` をしまえないからです
（「borrowed data escapes outside of closure」）。所有権を持つ `String` をしまえば解決します。

### 別解

`extract_if` も使えます。`count` がイテレータを消費します。

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
