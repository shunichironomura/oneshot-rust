`FnOnce` を呼ぶとそのクロージャは消費されるので、呼ぶには構造体から取り出す必要があります。ところが、
`&mut self` 越しにフィールドをムーブで取り出すことはできません。ほかの誰かが所有している構造体に穴が
空いてしまうからです。`init: F` として保存し、`get` の中で `(self.init)()` と書くと E0507
（「cannot move out of `self.init` which is behind a mutable reference」）になります。

解決策は `Option<F>` で保存することです。`Option::take` はクロージャを取り出して代わりに `None` を残すので、
構造体に穴は空きません。残った `None` は、`init` を使い終えたという記録にもなります。値は `Option<T>`
に保存し、あとは `get_or_insert_with` に任せます。値が `None` のときだけクロージャを呼び、結果を保存して、
その参照を返してくれます。このクロージャが使うのはフィールド `self.init` だけです。クロージャは
フィールド単位でキャプチャするので、`get_or_insert_with` が同時に可変借用している `self.value` とは
衝突しません。

次の短い書き方はコンパイルが通るので注意が必要です。

```rust
self.value.get_or_insert_with(self.init.take().unwrap())
```

`F` 自体が `FnOnce() -> T` なので引数の型には合いますが、引数は呼び出しの**前に**評価されます。
`get` のたびに `init` を取り出すことになり、2 回目の呼び出しでは `None` を `unwrap` してパニックします。
`into_value` の `self.value.unwrap_or_else(self.init.unwrap())` も同じで、`get` の後に呼ぶとパニックします。
式をクロージャで包めば（`|| (self.init.take().unwrap())()`）、本当に必要になるまで評価が遅れます。

`into_value` は `self` を値で受け取るので、フィールドをそのままムーブで取り出せます。`get` と
`into_value` の `expect` が失敗するのは、以前の `init` の呼び出しがパニックして、クロージャも値も
残っていない場合だけです。

`F: Fn` や `F: Clone` のような境界を追加すれば `init` をムーブせずに呼べ、`T: Clone` があれば
`into_value` で値を複製できます。しかしテストでは、キャプチャしたものを手放すクロージャや、
複製できない値が使われます。

標準ライブラリにも同じ役割の型があります。`std::cell::LazyCell`（とそのスレッド安全版の
`std::sync::LazyLock`）は、内部可変性を使って `&self` 越しに値を計算します。

### 別解

先にクロージャを取り出し、取り出せたかどうかで判断する方法です。`into_value` は `get` を使って書けます
（`mut self` は束縛を可変にするだけで、シグネチャは変わりません）。

```rust
pub fn get(&mut self) -> &T {
    if let Some(init) = self.init.take() {
        self.value = Some(init());
    }
    self.value.as_ref().expect("an earlier call to init panicked")
}

pub fn into_value(mut self) -> T {
    self.get();
    self.value.expect("an earlier call to init panicked")
}
```

列挙型を使うと状態がはっきりし、クロージャと値が同時に存在することがなくなります。`mem::replace` を使えば
`&mut self` 越しにクロージャを取り出せますが、代わりに置いておく値が必要です。そのための状態が
`Running` です。

```rust
use std::mem;

enum State<T, F> {
    Pending(F),
    Running,
    Ready(T),
}

pub struct Lazy<T, F: FnOnce() -> T> {
    state: State<T, F>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Self {
        Lazy { state: State::Pending(init) }
    }

    pub fn get(&mut self) -> &T {
        if let State::Pending(_) = self.state {
            // Take the closure out, leaving `Running` behind while `init` runs.
            if let State::Pending(init) = mem::replace(&mut self.state, State::Running) {
                self.state = State::Ready(init());
            }
        }
        match &self.state {
            State::Ready(value) => value,
            _ => panic!("an earlier call to init panicked"),
        }
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, State::Ready(_))
    }

    pub fn into_value(self) -> T {
        match self.state {
            State::Pending(init) => init(),
            State::Ready(value) => value,
            State::Running => panic!("an earlier call to init panicked"),
        }
    }
}
```
