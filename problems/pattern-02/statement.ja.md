次のコードはコンパイルが通りません。直してください。

このコードは Rust 2021 向けに書かれ、そこではコンパイルが通っていましたが、ジャッジは Rust 2024 として
コンパイルします。小さな得点表を扱うコードです。`Player` はエディタに入っています。

```rust
pub struct Player {
    pub name: String,
    pub score: u32,
}

pub fn name_len(nick: &Option<String>) -> usize;
pub fn leader(players: &[Player]) -> Option<&str>;
pub fn bonus_total(scores: &[(String, u32)]) -> u32;
pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize>;
pub fn initials(entries: &[(char, u32)]) -> String;
```

- `name_len` はニックネームの長さをバイト数で（`String::len` と同じ数え方で）返します。`None` なら `0` です。
- `leader` は得点が最も高いプレイヤーの名前を返します。同点なら、そのうち最初のプレイヤーを選びます。
  `players` が空なら `None` を返します。
- `bonus_total` は得点の合計を返します。ただし、名前が 3 バイトより長いエントリは 5 点多く数えます。
  結果は必ず `u32` に収まります。
- `rounds_over` は、得点が `par` より大きいラウンドの位置（0 から数える）を小さい順に返します。
  `par` と等しい得点は含みません。
- `initials` は `entries` の文字を順につなげた文字列を返します。

シグネチャは変えないでください。

### 例

| 呼び出し | 結果 |
| --- | --- |
| `name_len(&Some("anna".to_string()))` | `4` |
| anna（3 点）、bo（9 点）、cy（4 点）に対する `leader(&players)` | `Some("bo")` |
| `bonus_total(&[("anna".to_string(), 1), ("bo".to_string(), 2)])` | `8`（`1 + 5 + 2`） |
| `rounds_over(&[3, 7, 5, 9], 5)` | `[1, 3]` |
| `initials(&[('a', 1), ('b', 2)])` | `"ab"` |
