次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`Transaction` は操作を集め、その結果を共有のログに記録します。結果はコミットかロールバックの
どちらかです。コミットされずに破棄（drop）されたトランザクションはロールバックされます。

```rust
pub type Log = Rc<RefCell<Vec<String>>>;

impl Transaction {
    pub fn begin(log: &Log) -> Transaction;
    pub fn push(&mut self, op: &str);
    pub fn commit(self) -> usize;
    pub fn into_ops(self) -> Vec<String>;
}

impl Drop for Transaction { /* ... */ }
```

- `begin` は `log` に書き込む空のトランザクションを始めます。`begin` と `push` はログに何も
  書き込みません。
- `push` は操作 `op`（どんな文字列でもかまいません）をトランザクションに追加します。
- `commit` はログに `"commit:<ops>"` を追加し、操作の数を返します。`<ops>` は操作を追加した順に
  `,` でつないだものです。操作がなければ、行は `"commit:"` だけになります。
- `into_ops` はトランザクションをロールバックし、その操作を追加した順に返します。
- ロールバックすると、ログに `"rollback:<n>"` を追加します。`<n>` はそのトランザクションの操作の
  数です（空のトランザクションなら `"rollback:0"`）。
- コミットも `into_ops` による消費もされていないトランザクションは、破棄されるとロールバックします。
- 1 つのトランザクションがログに追加するのは**ちょうど 1 行**です。コミットしたトランザクションが
  さらにロールバックすることはなく、`into_ops` のロールバックも 1 回だけです。
- なくなったトランザクションがログを生かし続けてはいけません。`log` に書き込むトランザクションが
  すべてなくなったら、`Rc::strong_count(&log)` は `1` に戻ります。

`Log` と公開されているシグネチャは変えないでください。`Transaction` の非公開のフィールドと
ヘルパーは変更して構いません。

### 例

```rust
let log: Log = Rc::new(RefCell::new(Vec::new()));
let mut t = Transaction::begin(&log);
t.push("debit");
t.push("credit");
let n = t.commit(); // n == 2
// log: ["commit:debit,credit"]
```

```rust
let mut a = Transaction::begin(&log);
a.push("x");
a.push("y");
let ops = a.into_ops(); // ops == ["x", "y"]。"rollback:2" を追加
{
    let mut b = Transaction::begin(&log);
    b.push("z");
    let _c = Transaction::begin(&log);
} // 先に _c が破棄され（"rollback:0"）、次に b が破棄される（"rollback:1"）
```
