`Drop` を実装した型は分解できません。`let Transaction { log, ops } = self;` も、
`self.ops` を返すことも、`self` からフィールドをムーブで取り出すので、どちらもコンパイルエラーに
なります（E0509）。値には後で `drop` が実行され、`drop` はすべてのフィールドがそろった
`&mut self` を受け取るからです。`self` を値で受け取っても `Drop` は止まりません。メソッドの中の
`self` はふつうのローカル変数で、全体をどこかにムーブしない限り、メソッドの終わりで破棄されます。

そのため、コンパイラが提案する修正はコンパイルが通りますが、ここでは誤りです。
`ref log, ref ops` にするとパターンは借用するだけなので、`commit` の終わりで `self` が破棄され、
`drop` が `"commit:debit,credit"` の直後に `"rollback:2"` を追加します。`self.ops.clone()` に
すると、`into_ops` は自分で 1 回ロールバックし、`self` が破棄されるときにもう 1 回ロールバック
します。

`drop` は、トランザクションがすでに終わっていることを知る必要があります。模範解答では
`finished` フラグを追加し、`commit` と `into_ops` で立てて、`drop` はそれが `false` のときだけ
ロールバックします。フラグを書き換えるため、メソッドは `mut self` で受け取ります（引数の `mut` は
シグネチャの一部ではありません）。`into_ops` は、そのうえで `std::mem::take` を使って操作を
取り出します。`take` は代わりに空の `Vec` を残します。ロールバックの記録はその**前に**行って
ください。取り出した後は `self.ops.len()` が 0 になるからです。同じ理由で、操作を取り出して
ロールバックを `drop` に任せると `"rollback:0"` が記録されます。

ほかにも思いつきやすい修正がありますが、うまくいきません。`std::mem::forget(self)` は `drop` を
飛ばせますが、フィールドの破棄も飛ばしてしまいます。`log` の `Rc` が解放されないまま残るので、
`Rc::strong_count(&log)` は 1 に戻らず、Miri はメモリリークを報告します。フラグの代わりに
「操作が残っていない」ことを目印にすると、空のトランザクションで失敗します。空のトランザクションも
破棄されたときには `"rollback:0"` を記録しなければならないからです。

### 別解

ログへのハンドルを `Option` にして、取り出すことでトランザクションを終わらせる方法です。
`rollback` 自体が高々 1 回しか記録しなくなるので、`drop` は条件なしで呼べ、フラグとの食い違いも
起きません。

```rust
use std::cell::RefCell;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Transaction {
    /// `None` once the outcome has been logged.
    log: Option<Log>,
    ops: Vec<String>,
}

impl Transaction {
    pub fn begin(log: &Log) -> Transaction {
        Transaction { log: Some(Rc::clone(log)), ops: Vec::new() }
    }

    pub fn push(&mut self, op: &str) {
        self.ops.push(op.to_string());
    }

    pub fn commit(mut self) -> usize {
        if let Some(log) = self.log.take() {
            log.borrow_mut().push(format!("commit:{}", self.ops.join(",")));
        }
        self.ops.len()
    }

    pub fn into_ops(mut self) -> Vec<String> {
        self.rollback();
        std::mem::take(&mut self.ops)
    }

    fn rollback(&mut self) {
        if let Some(log) = self.log.take() {
            log.borrow_mut().push(format!("rollback:{}", self.ops.len()));
        }
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        self.rollback();
    }
}
```

元のコードのように分解して使いたいなら、フィールドを `Drop` を実装しない内側の構造体に移し、
それを `Option` に入れておきます。`take()` で内側の値を丸ごと取り出せば自由に分解でき、
`drop` はまだ `Some` のときだけロールバックします。`else` の分岐が実行されることはありません。
値が `None` になるのは `commit` と `into_ops` の中だけだからです。

```rust
struct Inner {
    log: Log,
    ops: Vec<String>,
}

pub struct Transaction {
    /// `Some` until `commit` or `into_ops` takes it out.
    inner: Option<Inner>,
}

impl Transaction {
    pub fn begin(log: &Log) -> Transaction {
        Transaction { inner: Some(Inner { log: Rc::clone(log), ops: Vec::new() }) }
    }

    pub fn push(&mut self, op: &str) {
        if let Some(inner) = &mut self.inner {
            inner.ops.push(op.to_string());
        }
    }

    pub fn commit(mut self) -> usize {
        let Some(Inner { log, ops }) = self.inner.take() else { return 0 };
        log.borrow_mut().push(format!("commit:{}", ops.join(",")));
        ops.len()
    }

    pub fn into_ops(mut self) -> Vec<String> {
        let Some(Inner { log, ops }) = self.inner.take() else { return Vec::new() };
        log.borrow_mut().push(format!("rollback:{}", ops.len()));
        ops
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if let Some(Inner { log, ops }) = &self.inner {
            log.borrow_mut().push(format!("rollback:{}", ops.len()));
        }
    }
}
```
