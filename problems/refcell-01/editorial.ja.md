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

`set_value` では、高いレベルから順に `else if` で調べれば、送るメッセージは多くても 1 つになります。
`value * 100 >= max * 90` は割合を整数のまま正確に比べる式で、与えられた範囲ならオーバーフローもしません。
整数の割り算には注意してください。`value / max * 100` は、`value` が `max` より小さければいつも `0` です。
`max * 3 / 4` はしきい値を切り捨てます（`max = 7` のとき、`5` は 71% しかないのに `5 >= 7 * 3 / 4` が成り立ちます）。
割合を四捨五入する（74.6 を 75 にする）のも、同じように間違えます。

### 別解

The Book では浮動小数点数の比で比べています。値が 1,000,000 以下なら、`f64` の精度で十分で、
正確に計算した場合と同じ答えになります。

```rust
pub fn set_value(&mut self, value: usize) {
    self.value = value;
    let percentage_of_max = self.value as f64 / self.max as f64;
    if percentage_of_max >= 1.0 {
        self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
        self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
        self.messenger.send("Warning: You've used up over 75% of your quota!");
    }
}
```

先にメッセージを選んで、送るのを 1 か所にまとめることもできます。切り捨てたパーセントが 90 に届くのは
ちょうど `value * 100 >= max * 90` のときなので、ここでは整数の割り算で構いません。`100..` は 100 以上に
マッチする範囲パターンで、アームは上から順に試されます。

```rust
pub fn set_value(&mut self, value: usize) {
    self.value = value;
    let percent = self.value * 100 / self.max;
    let message = match percent {
        100.. => "Error: You are over your quota!",
        90.. => "Urgent warning: You've used up over 90% of your quota!",
        75.. => "Warning: You've used up over 75% of your quota!",
        _ => return,
    };
    self.messenger.send(message);
}
```

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
