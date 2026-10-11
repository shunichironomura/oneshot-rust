Build a leaderboard from a list of game runs.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub name: String,
    pub score: u32,
    pub time: u32,
}

pub fn leaderboard(runs: Vec<Run>) -> Vec<Run>
```

- A run is **better** than another if its `score` is higher, or if the scores are equal and its `time`
  is shorter.
- Keep only the best run of each player (`name`, compared exactly). If a player has several equally
  good best runs, they are identical in score and time, so keep any one of them.
- Sort the kept runs from best to worst. Runs with the same score and time are sorted by `name` in
  ascending order.

### Examples

| Input `(name, score, time)` | Output |
| --- | --- |
| `eve 80 30`, `bob 90 50`, `eve 95 40`, `amy 90 45` | `eve 95 40`, `amy 90 45`, `bob 90 50` |
| `cat 70 20`, `ann 70 20`, `bob 70 20` | `ann 70 20`, `bob 70 20`, `cat 70 20` |
