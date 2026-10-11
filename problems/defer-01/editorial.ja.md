`FnOnce` を呼び出すとその値は消費されますが、`drop` が受け取るのは `&mut self` だけで、参照の先からは
フィールドをムーブできません（E0507）。そこでガードは処理を `Option<F>` に入れておき、`drop` では
`self.action.take()` を呼びます。これで処理を取り出し、代わりに `None` を残せます。`None` は
「もうすることがない」という意味にもなるので、`cancel` と `run_now` も簡単に書けます。

`drop` の中で処理を呼ぶには、`Drop` の実装に `F: FnOnce()` が必要です。ところが `Drop` の実装は、
構造体そのものより多くを要求できません。存在しうるすべての `Defer<F>` で動かなければならず、
Rust には「`F: FnOnce()` のときだけドロップする」という仕組みがないからです。`Defer<F>` と宣言した
構造体に `impl<F: FnOnce()> Drop for Defer<F>` を書くとエラーになります（E0367）。模範解答では、
構造体にも同じ境界を付けて `pub struct Defer<F: FnOnce()>` としています。普通、境界は構造体の定義より
impl に書くものですが、境界が必要な `Drop` の実装は例外です。

`cancel` と `run_now` は `self` を値で受け取りますが、`Defer` は `Drop` を実装しているので、
フィールドをムーブして取り出すことはできません。`let Defer { action } = self;` も `self.action.unwrap()` も
エラーになります（E0509）。メソッドの最後で `self` に対して `drop` が実行されるからです。
`mut self` で受け取れば（この `mut` はシグネチャの一部ではありません）、`cancel` は `self.action = None` と
書けます。これでクロージャとそれがキャプチャしていたものがその場でドロップされ、後で `drop` が
実行されても何もしません。`std::mem::forget(self)` もコンパイルは通りますが、クロージャが
ドロップされません。クロージャがキャプチャした `Rc` はずっと解放されず、Miri がメモリリークを報告します。
処理を実行するのはドロップそのものなので、`run_now` は `drop(self)` だけで済みます。

`defer_all` では、`defer` に渡すクロージャが `actions` の所有権を持ちます。クロージャの型には名前が
ありませんが、返り値の型を `Defer<impl FnOnce()>` とすれば、クロージャを持つガードを返せます。
難しいのはパニックの規則です。単純なループ `for action in actions.into_iter().rev() { action() }` は
最初のパニックで止まります。
巻き戻しの間にイテレータがドロップされますが、残りの `Box` はドロップされるだけで、呼び出されません。
模範解答は、処理を 1 つずつガードにして、後はドロップグルーに任せています。要素のデストラクタの 1 つが
パニックしても、Rust は巻き戻しの間に `Vec` の残りの要素をドロップします（構造体の残りのフィールドや、
スコープの残りのローカル変数も同じです）。大事な点が 2 つあります。

- ガードはクロージャの中で、クロージャが実行されるときに作ります。`defer_all` の中で作って
  クロージャにキャプチャさせると、`cancel` がクロージャをドロップしたときにガードもドロップされ、
  すべての処理が実行されてしまいます。
- `Vec` は要素を先頭から順にドロップするので、順序はガードを作る前の `rev()` で決めます。
  `guards.into_iter().rev()` で 1 つずつドロップする方法は、パニックが起きると順序が崩れます。
  `IntoIter` はまだ取り出していない要素を先頭から順にドロップするので、`[1, 2, 3, 4, 5]` で `4` が
  パニックすると、`5, 1, 2, 3` の順に実行されます。

ガードを使うときは、`_g` のような名前の変数に束縛してください。`let _ = defer(...)` は何も束縛しないので、
ガードはすぐにドロップされ、処理もその場で実行されます。

### 別解

処理を呼び出す関数ポインタを持たせれば、`Defer<F>` に境界を付けずに済みます。関数ポインタは
`F: FnOnce()` がわかっている `defer` で作ります。何もキャプチャしないクロージャは `fn` ポインタに
型強制できます。`cancel`、`run_now`、`defer_all` は模範解答と同じです。

```rust
pub struct Defer<F> {
    action: Option<F>,
    /// Calls the action. Set by `defer`, where `F: FnOnce()` is known.
    call: fn(F),
}

pub fn defer<F: FnOnce()>(f: F) -> Defer<F> {
    Defer { action: Some(f), call: |f| f() }
}

impl<F> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(action) = self.action.take() {
            (self.call)(action);
        }
    }
}
```

`defer_all` では、`catch_unwind` でパニックを 1 つずつ受け止め、すべての処理を実行してから最初のパニックを
`resume_unwind` で再開する方法もあります。`Box<dyn FnOnce()>` は `UnwindSafe` ではないので、
`AssertUnwindSafe` が必要です。模範解答と違い、パニックする処理が複数あっても動きます
（ドロップグルーでは 2 つ目のパニックで強制終了します）。

```rust
use std::panic::{self, AssertUnwindSafe};

pub fn defer_all(actions: Vec<Box<dyn FnOnce()>>) -> Defer<impl FnOnce()> {
    defer(move || {
        let mut first_panic = None;
        for action in actions.into_iter().rev() {
            if let Err(payload) = panic::catch_unwind(AssertUnwindSafe(action)) {
                first_panic.get_or_insert(payload);
            }
        }
        if let Some(payload) = first_panic {
            panic::resume_unwind(payload);
        }
    })
}
```

ガードに残りを任せる方法もあります。最後の処理を取り出し、残りの実行を後回しにしてから、それを呼びます。
パニックしても、巻き戻しの間にガードが残りを実行します。

```rust
/// Runs `actions` from the last one to the first.
fn run_from_last(mut actions: Vec<Box<dyn FnOnce()>>) {
    if let Some(last) = actions.pop() {
        // Runs the others after `last`, even if `last` panics.
        let _rest = defer(move || run_from_last(actions));
        last();
    }
}

pub fn defer_all(actions: Vec<Box<dyn FnOnce()>>) -> Defer<impl FnOnce()> {
    defer(move || run_from_last(actions))
}
```
