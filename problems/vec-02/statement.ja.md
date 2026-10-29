ある ToDo アプリは、ジョブを `Vec<Job>` で管理しています。同じジョブが 2 回追加されてしまうことがあります。
重複をその場で取り除く関数 `dedup_titles` を実装してください。構造体 `Job` はすでにエディタに入っています。

```rust
#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize
```

- `dedup_titles` は、`jobs` の中でそれより前にあるジョブと `title` が等しいジョブを、隣り合っているかどうかに
  かかわらずすべて取り除きます。つまり、各タイトルについて最初のジョブだけが残り、ほかのフィールドは
  関係ありません。タイトルはそのまま比べます（`"Build"`、`"build"`、`"build "` はすべて別のタイトルです）。
  残ったジョブは順序を保ちます。取り除いたジョブの数を返します。
- `jobs` は空のこともあります。

### 例

タイトルが `"a"`、`"b"`、`"a"`、`"c"`、`"b"` のジョブ 1 〜 5 に対して、`dedup_titles` は `2` を返し、
ジョブ 1、2、4 が残ります。
