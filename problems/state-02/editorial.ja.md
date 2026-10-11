`Post` 自体は正しく書かれています。`take()` は `Option` から箱をムーブして取り出し、次の状態を
格納するまで `None` を残します。`add_text` と `content` は状態を借用するだけです。エラーになるのは、
`Box<dyn State>` 型の `s` に対する `s.request_review()`、`s.approve()`、`s.reject()` の 3 か所です。

これらのメソッドは `self` を**値で**受け取ります。メソッドの探索は箱を通って中の `dyn State` に届くので、
呼び出すには箱から状態をムーブして取り出し、引数として渡さなければなりません。しかし `dyn State` の
サイズは実行時にしかわかりません（`Draft` かもしれず、`PendingReview` かもしれません）。サイズが
静的にわからない値はムーブできないので、これは E0161（"cannot move a value of type `dyn State`"）に
なります。トレイトの定義自体は受け入れられ、`Draft` に対してならこれらのメソッドを呼べます。
呼べないのは、トレイトオブジェクトを通したときだけです。

レシーバを `self: Box<Self>` にすると、メソッドはサイズのわかっている箱そのものを受け取ります。
これなら `Box<dyn State>` に対して呼べて、しかも古い状態を消費できます。状態の遷移に必要なのは
まさにこれです。各実装は `Box<Draft>` のような具体的な型の箱を受け取ります。ここで `Box::new(self)` と
書くと箱をさらに箱に入れることになり、`Box<Draft>` は `State` を実装していないのでエラーになります
（E0277）。`self` をそのまま返してください。`Box<Draft>` は `Box<dyn State>` に型強制され（unsizing
coercion）、元のメモリ領域がそのまま使われます。`PendingReview::approve` は `mut self: Box<Self>` を
受け取り、箱を通して `approvals` を増やします。

うまくいきそうで、うまくいかない修正が 2 つあります。

- 遷移のメソッドに `where Self: Sized` を付ける。これは、トレイトを `dyn Trait` として使えなくする
  メソッド（`Self` を返すメソッドや、ジェネリックなメソッド）に対するいつもの対処法ですが、
  このトレイトはもともと `dyn State` として使えます。メソッドが `dyn State` から外れるだけで、
  呼び出しは "the `approve` method cannot be invoked on a trait object" というエラーになります。
- `(*s).approve()` と書く。箱からのムーブを明示しただけなので、同じ理由で失敗します。

### 別解

遷移のメソッドを `&self` で受け取り、次の状態を作って返すこともできます。これでも動きますが、
状態が変わらないときも含めて遷移のたびに新しい箱を確保することになり、`PendingReview` は承認の回数を
新しい値にコピーしなければなりません。`Post` はテンプレートのままです。

```rust
trait State {
    fn request_review(&self) -> Box<dyn State>;
    fn approve(&self) -> Box<dyn State>;
    fn reject(&self) -> Box<dyn State>;

    // `text_to_add` と `content` は元のまま
}

impl State for PendingReview {
    fn request_review(&self) -> Box<dyn State> {
        Box::new(PendingReview { approvals: self.approvals })
    }

    fn approve(&self) -> Box<dyn State> {
        if self.approvals + 1 >= 2 {
            Box::new(Published)
        } else {
            Box::new(PendingReview { approvals: self.approvals + 1 })
        }
    }

    fn reject(&self) -> Box<dyn State> {
        Box::new(Draft)
    }
}

// `Draft` と `Published` は、状態が変わらないところで `Box::new(Draft)` や
// `Box::new(Published)` を返す
```

`self: Box<Self>` を使う場合、`Post` の 3 つの遷移は同じ形をしているので、`take` を 1 つのヘルパーに
まとめられます。ここでの `State::approve` は `<dyn State as State>::approve` のことで、
`Box<dyn State>` から `Box<dyn State>` への関数です。`Option::map` は、状態があるときだけそれを適用します。

```rust
pub fn request_review(&mut self) {
    self.transition(State::request_review);
}

pub fn approve(&mut self) {
    self.transition(State::approve);
}

pub fn reject(&mut self) {
    self.transition(State::reject);
}

fn transition(&mut self, f: fn(Box<dyn State>) -> Box<dyn State>) {
    self.state = self.state.take().map(f);
}
```
