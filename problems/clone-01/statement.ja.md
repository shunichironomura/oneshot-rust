次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`Handle<T>` は、複数の持ち主で共有する `T` 型の値へのハンドルです。値は `Rc<RefCell<T>>` の中にあり、
あるハンドルから作ったハンドルはどれも同じ値を指します。

```rust
pub struct Handle<T> { /* 非公開 */ }

impl<T> Handle<T> {
    pub fn new(value: T) -> Self;
    pub fn share(&self) -> Handle<T>;
    pub fn update(&self, f: impl FnOnce(&mut T));
    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R;
    pub fn handles(&self) -> usize;
}

pub fn fan_out<T>(h: &Handle<T>, n: usize) -> Vec<Handle<T>>
```

- `new(value)` は新しい値を作り、その最初のハンドルを返します。
- `h.clone()` と `h.share()` は、どちらも `h` と同じ値を指す新しいハンドルを返します。値そのものは
  コピーしません。`Handle<T>` は**どんな** `T` に対しても `Clone` を実装していなければなりません。
  例の `Counter` のように `Clone` を実装していない型も含みます。テストはそのようなハンドルに対して
  `clone()` を直接呼ぶこともあります。
- `Handle<T>` は、`T` が `Default` を実装していれば `Default` も実装します。`Handle::default()` は
  新しい値 `T::default()` を作り、その最初のハンドルを返します。
- `update(f)` は値への可変参照を渡して `f` を呼びます。`read(f)` は値への共有参照を渡して `f` を呼び、
  `f` の返り値を返します。あるハンドルを通して加えた変更は、同じ値を指すすべてのハンドルから見えます。
- `handles()` は、同じ値を指すハンドルのうち、いま生きているものの数を `self` も含めて返します。
  ハンドルを破棄すると数が減り、最後のハンドルを破棄したときに値も破棄されます。
- `fan_out(h, n)` は、`h` と同じ値を指す新しいハンドルを `n` 個返します（`n` が `0` なら空の `Vec`）。

公開されているシグネチャは変えないでください。上の決まりを守っていれば、それ以外は変更して構いません。

### 例

```rust
struct Counter { n: u32 } // Clone も Default も実装していない

let h = Handle::new(Counter { n: 0 });
let v = fan_out(&h, 2);
v[0].update(|c| c.n += 5);
h.read(|c| c.n)  // 5
h.handles()      // 3

let a = Handle::new(Counter { n: 1 });
let b = a.clone();
b.update(|c| c.n *= 10);
a.read(|c| c.n)  // 10
a.handles()      // 2
drop(b);
a.handles()      // 1
```
