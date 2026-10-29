ある ToDo アプリは、ジョブを `Vec<Job>` で管理しています。各ジョブは決まった回数の tick で期限切れになります。
リストをその場で更新する関数 `tick` を実装してください。構造体 `Job` はすでにエディタに入っています。

```rust
#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn tick(jobs: &mut Vec<Job>) -> Vec<u32>
```

- `tick` はまず、すべてのジョブの `ttl` を 1 減らします。`ttl` が `0` のものは `0` のままです。次に、完了した
  ジョブ（`done` が `true`）と、減らした後の `ttl` が `0` のジョブをすべて取り除き、取り除いたジョブの
  `id` を元の順序で返します。残ったジョブは順序を保ち、`ttl` は減らした後の値になります。
- `jobs` は空のこともあります。`ttl` は `0` や `u32::MAX` を含む任意の `u32` です。`tick` はパニックしては
  いけません。

### 例

ここではジョブを `(id, title, ttl, done)` と書きます。`[(1, "a", 1, false), (2, "b", 3, false), (3, "c", 5, true)]`
に対して、`tick` は `[1, 3]` を返し、`(2, "b", 2, false)` だけが残ります。
