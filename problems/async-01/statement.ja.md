次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

テストハーネスは、次の 3 つの非同期関数を自前の小さなエグゼキュータで実行します
（使えるのは `std` だけで、非同期ランタイムはありません）。

```rust
pub fn greet(name: String) -> impl Future<Output = String>;
pub async fn sum_all(futs: Vec<Pin<Box<dyn Future<Output = u32>>>>) -> u32;

pub struct Tree {
    pub children: Vec<Tree>,
}

pub async fn depth(t: &Tree) -> usize;
```

- `greet(name)` が返す Future は、`Hello, {name}!`（`{name}` は `name` に置き換える）を
  出力します。`name` は空文字列を含め、どんな文字列でもかまいません。
- `sum_all(futs)` は、ベクタの順に Future を 1 つずつ `await` します。各 Future が完了してから、
  次の Future を初めてポーリングします。結果は出力の合計で、ベクタが空なら `0` です。
  合計は必ず `u32` に収まります。ハーネスが渡す Future の中には、最初にポーリングされたときには
  まだ完了しないもの（`Poll::Pending` を 1 回返すもの）があります。
- `depth(t)` の結果は木の深さ（段の数）です。子のないノードは `1`、そうでなければ子の深さの
  最大値に 1 を足したものです。

シグネチャと構造体 `Tree` は変えないでください。`depth` の再帰は非同期のままにしてください。
各子の深さは、引き続き再帰呼び出しを `await` して求めます。

### 例

- `greet("Ann".to_string())` の Future は `"Hello, Ann!"` を出力します。
- `1`、`2`、`3` を出力する 3 つの Future を渡すと、`sum_all` の結果は `6` です。
- 根に子が 2 つあり、1 つ目の子にさらに子が 1 つある木の深さは `3` です。
