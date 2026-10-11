エラーは 2 つあり、片方だけ直しても通りません。

1 つ目は、借用されたままの `sink` を `sink.into_inner()` でムーブしていることです（E0505）。
借用はふつう最後に使われた所で終わりますが、`Tracer` は `Drop` を実装しているので、コンパイラは
トレーサーのドロップで `self.sink` が使われるかもしれないとみなします。そのため、トレーサーは
ドロップされるまで `sink` を借用し続けます。ローカル変数がドロップされるのは閉じかっこの所で、
末尾の式 `sink.into_inner()` を評価した**後**です。仮にコンパイルが通ったとしても、`"end"` の行は
書き込まれるのが遅すぎて、返す結果には入りません。`sink` をムーブする前に、`drop(tracers)` で
トレーサーを明示的にドロップしましょう。`Vec` は要素を先頭から順にドロップするので、求められている
順になります。

2 つ目に、`drop(tracers)` を足しただけでは「`sink` does not live long enough」（E0597）になります。
ローカル変数は宣言と逆の順にドロップされるので、`tracers` より後に宣言した `sink` が先にドロップされます。
通常の経路では、その時点で `tracers` はすでに `drop` にムーブされていますが、借用チェッカーはパニックで
スタックを巻き戻すときの経路も検査します。`drop(tracers)` より前に `Tracer::new`、`push`、`borrow_mut`
のどれかがパニックすると、巻き戻しで `sink` がドロップされ、その後に `tracers` がドロップされます。
すると `Tracer` の `drop` が破棄済みの記録先に書き込んでしまいます。これが**ドロップチェック**です。
型が `Drop` を実装している値は、借用しているものすべてより厳密に先に破棄されなければなりません。
`sink` を `tracers` より先に宣言すれば解決します。

どちらも `Drop` の実装があるから起きることです。`Drop` の実装がなければ、`Tracer` をドロップしても
`sink` には触れないことをコンパイラが知っているので、テンプレートのままでもコンパイルが通ります
（もちろん `"end"` の行は 1 つも書き込まれません）。

次の 2 つは、思いつきやすいもののうまくいかない直し方です。

- `tracers.clear()` や `std::mem::take(&mut tracers)` で `"end"` の行は書き込まれますが、（空になった）
  `Vec<Tracer<'_>>` は関数の最後まで生きていて、そのドロップは借用の使用とみなされます。
  ベクタそのものをムーブするか、スコープから出す必要があります。
- `sink.into_inner()` を `sink.take()` や `sink.borrow().clone()` に変えると、`sink` を先に宣言すれば
  コンパイルは通ります。しかし行を取り出した後にトレーサーがドロップされるので、`"end"` の行が失われます。

### 別解

トレーサーを内側のブロックに入れる方法です。ブロックの閉じかっこでドロップされるので、`sink` を
ムーブする前に片付きます。

```rust
pub fn trace(names: &[&str]) -> Vec<String> {
    let sink = RefCell::new(Vec::new());
    {
        let mut tracers = Vec::new();
        for name in names {
            tracers.push(Tracer::new(&sink, name));
        }
        sink.borrow_mut().push("run".to_string());
    }
    sink.into_inner()
}
```

先にトレーサーをドロップしておけば、`RefCell::take` でも取り出せます。

```rust
pub fn trace(names: &[&str]) -> Vec<String> {
    let sink = RefCell::new(Vec::new());
    let tracers: Vec<_> = names.iter().map(|name| Tracer::new(&sink, name)).collect();
    sink.borrow_mut().push("run".to_string());
    drop(tracers);
    sink.take()
}
```
