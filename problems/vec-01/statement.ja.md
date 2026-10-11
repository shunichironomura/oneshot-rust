ある ToDo アプリは、ジョブを `Vec<Job>` で管理しています。リストをその場で更新する 4 つの関数を実装してください。
構造体 `Job` はすでにエディタに入っています。

```rust
#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn take_done(jobs: &mut Vec<Job>) -> Vec<Job>
pub fn tick(jobs: &mut Vec<Job>) -> Vec<u32>
pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize
pub fn move_to_front(jobs: &mut Vec<Job>, id: u32) -> bool
```

- `take_done` は `done` が `true` のジョブをすべて `jobs` から取り除き、それらを返します。返すジョブも
  `jobs` に残るジョブも、元の順序を保ちます。
- `tick` はまず、すべてのジョブの `ttl` を 1 減らします。`ttl` が `0` のものは `0` のままです。次に、完了した
  ジョブ（`done` が `true`）と、減らした後の `ttl` が `0` のジョブをすべて取り除き、取り除いたジョブの
  `id` を元の順序で返します。残ったジョブは順序を保ち、`ttl` は減らした後の値になります。
- `dedup_titles` は、`jobs` の中でそれより前にあるジョブと `title` が等しいジョブを、隣り合っているかどうかに
  かかわらずすべて取り除きます。つまり、各タイトルについて最初のジョブだけが残り、ほかのフィールドは
  関係ありません。タイトルはそのまま比べます（`"Build"`、`"build"`、`"build "` はすべて別のタイトルです）。
  残ったジョブは順序を保ちます。取り除いたジョブの数を返します。
- `move_to_front` は、`id` フィールドが引数 `id` と等しいジョブのうち最初のものをインデックス 0 に移し、
  ほかのジョブは元の順序のままにして、`true` を返します。その `id` のジョブがなければ、`false` を返し、
  `jobs` は変えません。同じ `id` のジョブが複数あることもあります。
- `jobs` は空のこともあります。`ttl` は `0` や `u32::MAX` を含む任意の `u32` です。どの関数もパニックしては
  いけません。

### 例

ここではジョブを `(id, title, ttl, done)` と書きます。

- `[(1, "a", 5, false), (2, "b", 5, true), (3, "c", 5, false), (4, "d", 5, true)]` に対して、`take_done` は
  ジョブ 2 と 4 をこの順に返し、ジョブ 1 と 3 が残ります。
- `[(1, "a", 1, false), (2, "b", 3, false), (3, "c", 5, true)]` に対して、`tick` は `[1, 3]` を返し、
  `(2, "b", 2, false)` だけが残ります。
- タイトルが `"a"`、`"b"`、`"a"`、`"c"`、`"b"` のジョブ 1 〜 5 に対して、`dedup_titles` は `2` を返し、
  ジョブ 1、2、4 が残ります。
- `id` が `[1, 2, 3, 4]` のジョブに対して、`move_to_front(&mut jobs, 3)` は `true` を返し、`id` は
  `[3, 1, 2, 4]` になります。
