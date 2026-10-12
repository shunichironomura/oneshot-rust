`LimitTracker<'a, T>` は共有参照 `&'a T` を保持します。そのため、トラッカーがメッセンジャーより長生きすることはなく、
いくつものトラッカー（とテスト自身）が同じレコーダーを同時に使えます。その代わり、`send` が受け取れるのは
`&self` だけです。`Recorder` のフィールドが普通の `Vec<String>` だと何も記録できず、`send` の中の
`self.sent.push(...)` は "cannot borrow `self.sent` as mutable, as it is behind a `&` reference" という
エラーになります。コンパイラはトレイトを `&mut self` に変えるよう提案しますが、ここでは変更できませんし、
変えてもうまくいきません。トラッカーが `&recorder` を持っている間は、誰も `&mut` で借用できないからです。

ここで使うのが `RefCell` です（The Book のモックオブジェクトのパターン）。`RefCell<Vec<String>>` は
共有参照越しに変更できます。`borrow_mut()` は `&mut Vec<String>` として使えるガード（`RefMut`）を返し、
`borrow()` は読み取り用のガード（`Ref`）を返します。`messages` は所有権のある `Vec<String>` を返す必要が
あるので、ベクタを `clone` します。`Ref` ガードそのものは返せません。

`RefCell` は借用のルールをなくすのではなく、実行時に検査します。ガードが生きている間に、それと衝突する
`borrow_mut()` を呼ぶと "RefCell already borrowed" でパニックします。模範解答では、どのガードも文の終わりで
破棄される一時値なので、重なることはありません。`let sent = self.sent.borrow();` と書いて、`sent` をまだ
使っている間に `self.sent.borrow_mut().push(...)` を呼ぶようなコードは、コンパイルは通りますが、
最初のメッセージでパニックします。

### 別解

`Cell` は `Copy` でない値にも使えますが、中身を借用することはできません。`take` でベクタを取り出し
（代わりに空のベクタが残ります）、変更してから `set` で戻します。ガードがないのでパニックすることは
ありませんが、そのたびにベクタを出し入れすることになります。

```rust
use std::cell::Cell;

pub struct Recorder {
    sent: Cell<Vec<String>>,
}

impl Recorder {
    pub fn new() -> Recorder {
        Recorder { sent: Cell::new(Vec::new()) }
    }

    pub fn messages(&self) -> Vec<String> {
        let sent = self.sent.take();
        let copy = sent.clone();
        self.sent.set(sent);
        copy
    }
}

impl Messenger for Recorder {
    fn send(&self, msg: &str) {
        let mut sent = self.sent.take();
        sent.push(msg.to_string());
        self.sent.set(sent);
    }
}
```

`Mutex<Vec<String>>` でも `&self` 越しに変更でき、メッセンジャーを複数のスレッドで共有するならこちらを使います。
シングルスレッドのコードなら、`RefCell` のほうが軽い選択です。

```rust
use std::sync::Mutex;

pub struct Recorder {
    sent: Mutex<Vec<String>>,
}

impl Recorder {
    pub fn new() -> Recorder {
        Recorder { sent: Mutex::new(Vec::new()) }
    }

    pub fn messages(&self) -> Vec<String> {
        self.sent.lock().unwrap().clone()
    }
}

impl Messenger for Recorder {
    fn send(&self, msg: &str) {
        self.sent.lock().unwrap().push(msg.to_string());
    }
}
```

`RefCell` を使う場合、`messages` は `self.sent.borrow().to_vec()` や `self.sent.clone().into_inner()` とも書けます。
