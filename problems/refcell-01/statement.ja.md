`LimitTracker` は割り当て量（クォータ）をどれだけ使ったかを見張り、上限に近づくと `Messenger` を通して
警告を送ります。`Messenger` トレイトと `LimitTracker` は、すでにエディタに書いてあります。本物のメッセージを
送らずにトラッカーをテストするため、受け取ったメッセージをすべて覚えておくだけのモックのメッセンジャー
`Recorder` を書いてください。`Recorder` のフィールドは自由に決めてください。

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> { /* 実装済み */ }

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> Self;  // 実装済み
    pub fn set_value(&mut self, value: usize);         // 実装済み
}

pub struct Recorder { /* 自由に定義 */ }

impl Recorder {
    pub fn new() -> Recorder;
    pub fn messages(&self) -> Vec<String>;
}

impl Messenger for Recorder { /* send */ }
```

- `set_value` は 1 回の呼び出しで多くても 1 つのメッセージを送ります。`max` の 100% 以上ならエラー、
  90% 以上なら緊急の警告、75% 以上なら警告です。
- `Recorder::new` はメッセージを 1 つも持たないレコーダーを作ります。`send` は `msg` を記録します（トラッカーの
  メッセージに限らず、どんな文字列でも記録します）。`messages` は記録したメッセージすべてのコピーを古い順に返します。
- テストでは、トラッカーがまだレコーダーへの参照を持っている間に `messages` を呼んだり、1 つのレコーダーを
  複数のトラッカーで共有したりします。

`Messenger` トレイト、`LimitTracker`、与えられたシグネチャは変えないでください。

### 例

```rust
let recorder = Recorder::new();
recorder.send("hello");
let mut tracker = LimitTracker::new(&recorder, 100);
tracker.set_value(80); // 80%：警告
tracker.set_value(95); // 95%：緊急の警告
tracker.set_value(10); // 10%：何も送らない
// recorder.messages() == [
//     "hello",
//     "Warning: You've used up over 75% of your quota!",
//     "Urgent warning: You've used up over 90% of your quota!",
// ]
```
