`Lazy` は、最初に必要になったときに初めて計算される値を持つ構造体です。値を計算する関数 `init` から作ります。
構造体は仮のフィールドを持った形でエディタに入っています。そのフィールドを必要なフィールドに置き換えて、
4 つのメソッドを実装してください。

```rust
pub struct Lazy<T, F: FnOnce() -> T> { /* フィールドは自由に決める */ }

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Self;
    pub fn get(&mut self) -> &T;
    pub fn is_ready(&self) -> bool;
    pub fn into_value(self) -> T;
}
```

- `new` は `init` を呼ばずに保存します。
- `get` は値への参照を返します。値がまだ計算されていなければ、先に `init` を呼んでその結果を保存します。
  2 回目以降の呼び出しでは `init` を呼ばず、保存した値を返します。
- `is_ready` は、値がすでに計算されていれば（つまり `init` が呼ばれていれば） `true` を、そうでなければ
  `false` を返します。`is_ready` 自身が値を計算することはありません。
- `into_value` は `Lazy` を消費して値を返します。値がまだ計算されていなければ、ここで `init` を呼びます。
  計算済みなら `init` を呼ばず、保存した値（`get` が返したもの）を返します。
- `init` が呼ばれるのは 1 つの `Lazy` につき多くても 1 回で、呼ぶのは最初の `get` か `into_value` です。
  どちらも呼ばれなければ（たとえば使われないまま `Lazy` が破棄されたときは）、`init` は一度も呼ばれません。
- `init` には、型が `FnOnce() -> T` を実装している値なら何でも渡せます（関数、クロージャ、
  ボックス化したクロージャなど）。`move || name` のように、キャプチャした `String` を手放してしまうため
  1 回しか呼べないクロージャも含まれます。`T` はどんな型でもよく、`Clone`、`Copy`、`Default` を
  実装しているとは限りません。

シグネチャは変えず、トレイト境界も追加しないでください。

### 例

```rust
let name = String::from("Ferris");
let mut greeting = Lazy::new(move || name + "!");
greeting.is_ready()   // false
greeting.get()        // "Ferris!"（ここで init が呼ばれる）
greeting.is_ready()   // true
greeting.get()        // "Ferris!"（init はもう呼ばれない）
greeting.into_value() // String::from("Ferris!")
```

```rust
let calls = Cell::new(0);
let lazy = Lazy::new(|| {
    calls.set(calls.get() + 1);
    vec![1, 2, 3]
});
// calls.get() は 0
lazy.into_value() // vec![1, 2, 3]。calls.get() は 1 になる
```
