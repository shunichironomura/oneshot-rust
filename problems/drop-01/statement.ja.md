`Lock` は名前の付いた資源で、取得したときと解放したときに共有のログへ書き込みます。
`Lock` を破棄（drop）すると解放されます。`Lock` と、それを使う 4 つの関数を実装してください。

```rust
pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Lock {
    name: String,
    log: Log,
}

impl Lock {
    pub fn acquire(name: &str, log: &Log) -> Lock;
    pub fn name(&self) -> &str;
}

impl Drop for Lock { /* 実装する */ }

pub fn with_lock<R>(name: &str, log: &Log, f: impl FnOnce() -> R) -> R;
pub fn release_all(locks: Vec<Lock>);
pub fn swap_in(slot: &mut Lock, name: &str);
pub fn keep_first(locks: Vec<Lock>) -> Option<Lock>;
```

- `acquire` は `log` に書き込む `name` という名前のロックを作り、`log` に `"acquire <name>"` を
  追加します。
- ロックが破棄されると、そのログに `"release <name>"` を追加します。
- `name` はロックの名前を返します。名前はどんな文字列でもよく、空文字列のこともあります。
- `with_lock` は `log` に書き込む `name` という名前のロックを取得し、ロックを持ったまま `f` を呼び、
  その後でロックを解放して、`f` の戻り値を返します。したがって、`f` が `log` に追加するものは
  `"acquire <name>"` より後、`"release <name>"` より前に入ります。
- `release_all` は `locks` のロックをすべて、**最後の**ロックから先頭に向かって順に解放します。
- `swap_in` は `slot` のロックを、同じログに書き込む `name` という名前の新しいロックに
  置き換えます。先に新しいロックを取得し、その後で古いロックを解放します。
- `keep_first` は `locks` の最初のロックを返し（`locks` が空なら `None`）、ほかのロックを
  すべて、`locks` での並び順に解放します。

上のコードはエディタに入っています。`Log` と公開されているシグネチャは変えないでください。

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
// first.unwrap().name() == "a"
```

```rust
let n = with_lock("m", &log, || {
    log.borrow_mut().push("work".to_string());
    42
});
// n == 42 で、この呼び出しで "acquire m", "work", "release m" が追加される
```
