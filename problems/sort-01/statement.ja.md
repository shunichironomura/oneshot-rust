ゲームのプレイ記録の一覧から、ランキング表を作ってください。

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub name: String,
    pub score: u32,
    pub time: u32,
}

pub fn leaderboard(runs: Vec<Run>) -> Vec<Run>
```

- `score` が高い記録ほど**よい**記録です。`score` が同じなら、`time` が短いほうがよい記録です。
- プレイヤー（`name` で区別し、完全一致で比べる）ごとに、いちばんよい記録だけを残します。
  いちばんよい記録が複数あっても、それらは `score` も `time` も同じなので、どれを残しても構いません。
- 残した記録を、よい順に並べます。`score` と `time` が同じ記録どうしは、`name` の昇順に並べます。

### 例

| 入力 `(name, score, time)` | 出力 |
| --- | --- |
| `eve 80 30`、`bob 90 50`、`eve 95 40`、`amy 90 45` | `eve 95 40`、`amy 90 45`、`bob 90 50` |
| `cat 70 20`、`ann 70 20`、`bob 70 20` | `ann 70 20`、`bob 70 20`、`cat 70 20` |
