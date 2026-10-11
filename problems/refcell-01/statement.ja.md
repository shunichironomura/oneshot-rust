`LimitTracker` は割り当て量（クォータ）をどれだけ使ったかを見張り、上限に近づくと `Messenger` を通して
警告を送ります。本物のメッセージを送らずにこれをテストするため、受け取ったメッセージをすべて覚えておくだけの
モックのメッセンジャー `Recorder` も書いてください。

トレイト `Messenger` と `LimitTracker` のフィールドはエディタに入っています。
`Recorder` のフィールドは自由に決めてください。

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> Self;
    pub fn set_value(&mut self, value: usize);
}

pub struct Recorder { /* 自由に定義 */ }

impl Recorder {
    pub fn new() -> Recorder;
    pub fn messages(&self) -> Vec<String>;
}

impl Messenger for Recorder { /* send */ }
```

- `LimitTracker::new` は、`messenger` に報告し、割り当て量が `max` のトラッカーを作ります。何も送りません。
- `set_value` は、これまでの使用量として `value` を保存します。そして次の表で最初に当てはまる行の
  メッセージを送ります。つまり、1 回の呼び出しで送るメッセージは**多くても 1 つ**です。

| 条件 | 送るメッセージ |
| --- | --- |
| `value` が `max` 以上 | `Error: You are over your quota!` |
| `value` が `max` の 90% 以上 | `Urgent warning: You've used up over 90% of your quota!` |
| `value` が `max` の 75% 以上 | `Warning: You've used up over 75% of your quota!` |
| それ以外 | 何も送らない |

- 割合は正確に比べます。たとえば `max = 20` のとき、`value` が `15` ならちょうど 75% なので警告を送り、
  `14` なら何も送りません。
- 各呼び出しは新しい `value` だけを見て決めます。前回の呼び出しと同じメッセージになる場合も、もう一度送ります。
- `1 <= max <= 1_000_000`、`value <= 1_000_000` です。
- `Recorder::new` はメッセージを 1 つも持たないレコーダーを作ります。`send` は `msg` を記録します（上の 3 つに
  限らず、どんな文字列でも記録します）。`messages` は記録したメッセージすべてのコピーを古い順に返します。
- テストでは、トラッカーがまだレコーダーへの参照を持っている間に `messages` を呼んだり、1 つのレコーダーを
  複数のトラッカーで共有したり、テスト側で定義したメッセンジャーの型で `LimitTracker` を使ったりします。

`Messenger` トレイトと、与えられたシグネチャは変えないでください。

### 例

```rust
let recorder = Recorder::new();
let mut tracker = LimitTracker::new(&recorder, 100);
tracker.set_value(80); // 80%：警告
tracker.set_value(95); // 95%：緊急の警告
tracker.set_value(10); // 10%：何も送らない
// recorder.messages() == [
//     "Warning: You've used up over 75% of your quota!",
//     "Urgent warning: You've used up over 90% of your quota!",
// ]
```

```rust
let recorder = Recorder::new();
recorder.send("hello");
let mut tracker = LimitTracker::new(&recorder, 20);
tracker.set_value(14); // 70%：何も送らない
tracker.set_value(15); // ちょうど 75%：警告
tracker.set_value(25); // 割り当て量を超えている：エラー
// recorder.messages() == [
//     "hello",
//     "Warning: You've used up over 75% of your quota!",
//     "Error: You are over your quota!",
// ]
```
