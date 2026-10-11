`Lock` は名前の付いた資源で、取得したときと解放したときに共有のログへ書き込みます。
`Lock` を破棄（drop）すると解放されます。`Lock` と、複数のロックをまとめて扱う 3 つの関数を
実装してください。

```rust
pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Lock {
    name: String,
    log: Log,
}

impl Lock {
    pub fn acquire(name: &str, log: &Log) -> Lock;
    pub fn name(&self) -> &str;
    pub fn into_name(self) -> String;
}

impl Drop for Lock { /* 実装する */ }

pub fn release_all(locks: Vec<Lock>);
pub fn swap_in(slot: &mut Lock, name: &str);
pub fn keep_first(locks: Vec<Lock>) -> Option<Lock>;
```

- `acquire` は `log` に書き込む `name` という名前のロックを作り、`log` に `"acquire <name>"` を
  追加します。
- ロックが破棄されると、そのログに `"release <name>"` を追加します。
- `name` はロックの名前を返します。名前はどんな文字列でもよく、空文字列のこともあります。
- `into_name` はロックを消費して、その名前を返します。このときロックは**解放しません**。
  ログには、そのときも後からも何も追加されません。
- `release_all` は `locks` のロックをすべて解放します。**最後の**ロックから順に解放します。
- `swap_in` は `slot` のロックを、同じログに書き込む `name` という名前の新しいロックに
  置き換えます。先に新しいロックを取得し、その後で古いロックを解放します。
- `keep_first` は `locks` の最初のロックを返し（`locks` が空なら `None`）、ほかのロックを
  すべて `locks` の中の順に解放します。
- 解放されたロックや `into_name` で消費されたロックは、ログを生かし続けてはいけません。
  `log` に書き込むロックがすべてなくなったら、`Rc::strong_count(&log)` は `1` に戻ります。

上のコードはエディタに入っています。`Log` と公開されているシグネチャは変えないでください。
`Lock` の非公開のフィールドは変更して構いません。

### 例

```rust
let log: Log = Rc::new(RefCell::new(Vec::new()));
let a = Lock::acquire("a", &log);
let b = Lock::acquire("b", &log);
let c = Lock::acquire("c", &log);
release_all(vec![a, b, c]);
// log: ["acquire a", "acquire b", "acquire c", "release c", "release b", "release a"]
```

```rust
let mut x = Lock::acquire("x", &log); // "acquire x" を追加
swap_in(&mut x, "y");                 // "acquire y", "release x" を追加
// x.name() == "y"
```

```rust
let a = Lock::acquire("a", &log);
let b = Lock::acquire("b", &log);
let c = Lock::acquire("c", &log);
let first = keep_first(vec![a, b, c]); // "release b", "release c" を追加
let a = first.unwrap();                // a.name() == "a"
a.into_name();                         // "a" を返し、何も追加しない
```
