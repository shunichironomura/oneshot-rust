`Option::unwrap` は `Option` を**値で**受け取ります。`self.state` は `&mut self`（`content` では `&self`）の
先にあり、借用している構造体からフィールドだけをムーブすることはできません。`*self` に穴が開いてしまうからです。
そのため `self.state.unwrap()` はどれもエラー E0507 になります。箱を `s` にムーブする
`if let Some(s) = self.state` も同じです。

どう直すかは、それぞれの呼び出しが何を必要とするかで決まります。

- 状態を変えるメソッド（`request_review`、`approve`、`reject`）は `self: Box<Self>` を受け取るメソッドを呼びます。
  これらは古い状態を**消費して**新しい状態を返すので、参照ではなく箱そのものが必要です。`Option::take` は
  値をムーブして取り出し、代わりに `None` を残します。フィールドが `Option` になっているのはまさにこのためです。
  取り出した後で新しい状態を格納します。
- `content` と `add_text` は状態を見るだけです。`Option::as_ref` は `&Option<Box<dyn State>>` を
  `Option<&Box<dyn State>>` に変え、メソッド呼び出しは箱を通して状態に届きます。`add_text` で
  `self.state` を読みながら `self.content` に追加できるのは、別々のフィールドだからです。また、`text_to_add` が
  返す `&'a str` は状態ではなく `text` を借用しています。

状態を変えるメソッドでは、コンパイラの提案に従うだけではうまくいきません。`as_ref()` や `as_mut()` で
得られるのは参照だけです。そこから `self: Box<Self>` のメソッドを呼ぶと、参照の先から箱をムーブすることになり、
また E0507 になります。提案される `if let Some(ref s)` も同じ理由で失敗します。
`Box<dyn State>` は `Clone` を実装していないので、`clone()` も使えません。

`add_text` で `take()` を使ってもコンパイルは通りますが、状態を戻し忘れると記事には `None` が残り、
それ以降は何も動かなくなります。

### 別解

状態を変える 3 つのメソッドは同じ形をしているので、`take` をヘルパーにまとめられます。`Option::map` は
状態があるときだけ遷移を適用し、`as_deref` は `Option<&dyn State>` を返します。

```rust
pub fn add_text(&mut self, text: &str) {
    if let Some(state) = &self.state {
        self.content.push_str(state.text_to_add(text));
    }
}

pub fn content(&self) -> &str {
    self.state.as_deref().map_or("", |s| s.content(self))
}

pub fn request_review(&mut self) {
    self.transition(|s| s.request_review());
}

pub fn approve(&mut self) {
    self.transition(|s| s.approve());
}

pub fn reject(&mut self) {
    self.transition(|s| s.reject());
}

fn transition(&mut self, f: impl FnOnce(Box<dyn State>) -> Box<dyn State>) {
    self.state = self.state.take().map(f);
}
```

トレイトのメソッドそのものを関数として渡すこともできます。ここでの `State::approve` は
`<dyn State as State>::approve` のことで、`Box<dyn State>` から `Box<dyn State>` への関数です。
`request_review` と `reject` も同じように書けます。

```rust
pub fn approve(&mut self) {
    self.transition(State::approve);
}

fn transition(&mut self, f: fn(Box<dyn State>) -> Box<dyn State>) {
    if let Some(s) = self.state.take() {
        self.state = Some(f(s));
    }
}
```

元の 1 行の形を残したいなら、`take().unwrap()` でも動きます。状態は必ず戻されるからです。

```rust
pub fn approve(&mut self) {
    self.state = Some(self.state.take().unwrap().approve());
}
```
