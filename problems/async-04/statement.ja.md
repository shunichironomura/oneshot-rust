次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

非同期タスクが、`std::sync::Mutex` に入れた `Vec<String>` のログを共有しています。

```rust
pub async fn yield_now();
pub async fn record_all(log: &Mutex<Vec<String>>, items: Vec<String>) -> usize;
pub async fn forward_all(inbox: &Mutex<Vec<String>>, log: &Mutex<Vec<String>>) -> usize;
```

- `yield_now()` の Future は、最初にポーリングされたときは（タスクを起こしてから）`Poll::Pending` を、
  次は `Poll::Ready(())` を返します。このままで正しく動きます。
- `record_all(log, items)` は、要素を順に 1 つずつ処理します。要素ごとに、`log` をロックして要素を
  追加し、`log` の新しい長さを記録し、`log` のロックを解除してから、`yield_now()` を `await` します。
  最後の追加の後に記録した長さに解決されます。`items` が空のときは、yield せずに `log` の現在の長さに
  解決されます。
- `forward_all(inbox, log)` は、`inbox` をロックしてメッセージをすべて一度に取り出し（`inbox` は
  空になります）、ロックを解除します。それから、取り出したメッセージを順番どおりに `record_all` で
  `log` に記録し、`record_all` と同じ値に解決されます。`inbox` と `log` は別々のミューテックスです。

テストハーネスは、自前の小さなエグゼキュータで Future を実行します（使えるのは `std` だけです）。
メインスレッドで作った Future を別のスレッドで最後まで実行するテストがあるので、Future は `Send` で
なければなりません。2 つの Future を、両方が完了するまで交互に（1 つ目、2 つ目、1 つ目、……の順に）
ポーリングするテストもあります。Future が待っている間に、ほかの Future やハーネス自身が `log` や
`inbox` をロックして要素を追加することがあります。

シグネチャと `yield_now` は変えないでください。`std::sync::Mutex` と yield はそのまま使ってください。

### 例

- `log` が空のとき、`record_all(&log, vec!["a".into(), "b".into()])` は `2` に解決され、`log` は
  `["a", "b"]` になります。追加のたびに 1 回ずつ、合わせて 2 回 `Poll::Pending` を返します。
- `inbox` が `["x", "y"]`、`log` が `["start"]` のとき、`forward_all(&inbox, &log)` は `3` に解決され、
  `log` は `["start", "x", "y"]` に、`inbox` は空になります。
