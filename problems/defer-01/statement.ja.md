「後回しの処理」とは、今のスコープを抜けるときに、どのような抜け方をしても（最後まで実行したときも、
途中の `return` でも、パニックでも）必ず実行したい処理のことです。Go にはそのための `defer` 文があります。
Rust では、ドロップされたときに処理を実行するガード値として書けます。そのようなガード `Defer` と、
複数の処理を 1 つのガードにまとめる `defer_all` を実装してください。

```rust
pub struct Defer<F> { /* フィールドは自由 */ }

pub fn defer<F: FnOnce()>(f: F) -> Defer<F>;

impl<F: FnOnce()> Defer<F> {
    pub fn cancel(self);
    pub fn run_now(self);
}

impl<F> Drop for Defer<F> { /* 実装する */ }

pub fn defer_all(actions: Vec<Box<dyn FnOnce()>>) -> Defer<impl FnOnce()>;
```

- `defer(f)` は、処理 `f` を持つガードを返します。`f` は呼び出しません。
- ガードはドロップされるときに処理を実行します。スコープの終わりでも、`drop(guard)` でも、
  パニックでスタックが巻き戻されるときでも同じです。ほかのローカル変数と同じく、1 つのスコープにある
  複数のガードは宣言と逆の順にドロップされるので、処理は最後のガードのものから順に実行されます。
  ガードをムーブしても処理は実行されません。ガードの最後の持ち主がドロップしたときに実行されます。
- `cancel` は、処理を実行せずにガードを消費します。処理はその場でドロップされるので、
  処理がキャプチャしていたものはすべて、`cancel` から戻る前に解放されます。
- `run_now` は、ガードを消費して、処理をすぐに実行します。後でもう一度実行されることはありません。
- 1 つの処理が実行されるのは多くても 1 回です。処理がパニックしたときは、そのパニックはガードを
  ドロップした（または `run_now` を呼んだ）コードに伝わり、処理がもう一度実行されることはありません。
- 処理には `FnOnce()` を実装する型の値なら何でも使えます。ローカル変数を借用するクロージャ、
  `move` クロージャ、1 回しか呼べないクロージャ（キャプチャした `String` を手放すものなど）、
  `Box<dyn FnOnce()>` のどれでも構いません。
- `defer_all(actions)` は 1 つのガードを返します。その処理は `actions` のすべての処理を
  **最後の**要素から先頭に向かって実行します（`actions` は空のこともあります）。それまではどの処理も
  実行しません。このガードに `cancel` を呼ぶと、どの処理も実行せず、すべての処理をその場で解放します。
  `run_now` を呼ぶと、すべての処理をすぐに実行します。
- これらの処理の 1 つがパニックしても、残りの処理は同じ順に（後ろから先頭へ）すべて実行されます。
  その後、同じペイロードのまま、パニックがガードの `drop`（または `run_now`）から外に伝わります。
- テストでは、すでに別のパニックで巻き戻している最中に処理がパニックすることはありません
  （Rust はプログラムを強制終了させます）。また、`defer_all` に渡す処理のうち、パニックするのは
  多くても 1 つです。

上のコードはエディタに入っています。`defer`、`cancel`、`run_now`、`defer_all` のシグネチャは
変えないでください。`Defer<F>` の定義とその `Drop` の実装は自由です。

### 例

```rust
let log = RefCell::new(Vec::new());
{
    let _a = defer(|| log.borrow_mut().push("a"));
    let _b = defer(|| log.borrow_mut().push("b"));
    log.borrow_mut().push("body");
} // 先に _b、次に _a がドロップされる
// log: ["body", "b", "a"]

defer(|| log.borrow_mut().push("never")).cancel(); // 何も追加しない
defer(|| log.borrow_mut().push("now")).run_now();  // すぐに "now" を追加する
```

```rust
let log = RefCell::new(Vec::new());
let result = catch_unwind(AssertUnwindSafe(|| {
    let _g = defer(|| log.borrow_mut().push("cleanup"));
    panic!("oops");
}));
// result はペイロードが "oops" の Err で、log は ["cleanup"]
```

次の例では、`log` は `Rc<RefCell<Vec<&str>>>` です。`push(&log, s)` は、`log` に `s` を追加する
`Box<dyn FnOnce()>` を返します。

```rust
let all = defer_all(vec![push(&log, "1"), push(&log, "2"), push(&log, "3")]);
// log はまだ空
drop(all);
// log: ["3", "2", "1"]
```

```rust
let result = catch_unwind(AssertUnwindSafe(|| {
    let _all = defer_all(vec![push(&log, "1"), Box::new(|| panic!("2")), push(&log, "3")]);
}));
// log は ["3", "1"] で、result はペイロードが "2" の Err
```
