`#[derive(Clone)]` は、フィールドに本当に必要な境界を調べません。ジェネリックな構造体では、
すべての型引数に境界を付けて `impl<T: Clone> Clone for Handle<T>` を生成します。ただ 1 つの
フィールドである `Rc<RefCell<T>>` は、どんな `T` でも複製できるにもかかわらずです（`Rc` の複製は
ポインタをコピーしてカウントを増やすだけです）。
`impl<T> Handle<T>` の中では `T` について何もわからないので、そこでは `Handle<T>` は `Clone` ではありません。

意外なのは、それでも `self.clone()` が `clone` メソッドを見つけてしまうことです。メソッドの探索では
レシーバに `&` を 1 つ足した型も試され、`&Handle<T>` はどんな `T` でも `Clone` です（共有参照はすべて
`Copy` です）。そのため、`self` が `&Handle<T>` である `share` の `self.clone()` は**参照**をコピーし、
コンパイラは結果の型について「expected `Handle<T>`, found `&Handle<T>`」と怒ります。`fan_out` の
`h.clone()` も `&Handle<T>` になるので、`collect` で `Vec<Handle<T>>` を作れません。

境界を付けずに、`Clone` を手で実装すれば直ります。

```rust
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle { inner: Rc::clone(&self.inner) }
    }
}
```

これで `self.clone()` も `h.clone()` も `Handle<T>` を返すようになり、`share` と `fan_out` はそのままで
動きます。`Rc::clone(&self.inner)` は `self.inner.clone()` と同じですが、ポインタだけをコピーすることが
はっきりします。`#[derive(Default)]` は残してください。新しいハンドルには `T::default()` が必要なので、
こちらの境界 `T: Default` はちょうど正しいものです。

エディタのコードがコンパイルできるようになるだけの修正では足りません。

- `impl<T> Handle<T>` と `fan_out` に `T: Clone` を付けるとコンパイルは通りますが、`Handle<Counter>` は
  依然として `Clone` ではないので、テストがコンパイルできません。
- `share` を `Handle { inner: Rc::clone(&self.inner) }` にして、`fan_out` で `h.share()` を呼ぶ方法も
  コンパイルは通りますが、テストは `Handle<Counter>` の `clone()` も呼びます。
- `share` が `self.clone()` を呼んだまま `fn clone(&self) -> Self { self.share() }` と書くと、
  無限に再帰します。

### 別解

処理を `share` に書き、`clone` からそれを呼ぶ方法です。すべての境界を目で確かめたいなら、`Default` も
手で書けます。

```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct Handle<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        self.share()
    }
}

impl<T: Default> Default for Handle<T> {
    fn default() -> Self {
        Handle::new(T::default())
    }
}

impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Handle { inner: Rc::new(RefCell::new(value)) }
    }

    pub fn share(&self) -> Handle<T> {
        Handle { inner: Rc::clone(&self.inner) }
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.inner.borrow_mut());
    }

    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.inner.borrow())
    }

    pub fn handles(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

pub fn fan_out<T>(h: &Handle<T>, n: usize) -> Vec<Handle<T>> {
    (0..n).map(|_| h.share()).collect()
}
```

`Handle<T>` がどんな `T` でも `Clone` になれば、`vec!` で複製を作ることもできます。`vec!` は要素を
`n - 1` 回複製し、元の値を最後の位置に移動する（`n` が `0` なら破棄する）ので、ハンドルの数は正しくなります。

```rust
pub fn fan_out<T>(h: &Handle<T>, n: usize) -> Vec<Handle<T>> {
    vec![h.clone(); n]
}
```
