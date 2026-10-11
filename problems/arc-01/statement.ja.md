`std::sync::Arc<T>` のようにスレッド間で共有できる参照カウント付きポインタ `MyArc<T>` を、
ヒープ確保・生ポインタ・アトミック変数を使ってゼロから作ってください（弱参照はありません）。

```rust
pub struct MyArc<T> { /* 非公開のフィールドは自由に決める */ }

impl<T> MyArc<T> {
    pub fn new(value: T) -> Self;
    pub fn strong_count(this: &Self) -> usize;
    pub fn get_mut(this: &mut Self) -> Option<&mut T>;
    pub fn ptr_eq(a: &Self, b: &Self) -> bool;
}

impl<T> Clone for MyArc<T> { /* 実装する */ }
impl<T> Deref for MyArc<T> { type Target = T; /* 実装する */ }
impl<T> Drop for MyArc<T> { /* 実装する */ }
```

- `MyArc::new(value)` は `value` を新しく確保したヒープ領域に移し、それを指す最初のハンドルを
  返します。
- `clone` は同じ値を指す別のハンドルを返します。値そのものは複製しません。`T` が `Clone` を
  実装している必要はありません。
- ハンドルを参照外しすると、値にアクセスできます（`*a`、`a.len()` など）。
- `MyArc::strong_count(&a)` は、`a` の値を指すハンドルのうち、いま存在するものの数です
  （`a` 自身も含みます）。
- `MyArc::get_mut(&mut a)` は、`a` が値を指す唯一のハンドルなら値への可変参照を返し、
  そうでなければ `None` を返します。
- `MyArc::ptr_eq(&a, &b)` は、`a` と `b` が同じ値を指すハンドルのときに `true` です。
  1 つの値を指すのは、`new` が返したハンドルと、そこから（直接、またはほかの複製を経由して）
  `clone` したすべてのハンドルです。別々の `new` の呼び出しで作った値は、等しくても、
  サイズ 0 の型でも、同じ値にはなりません。
- 最後でないハンドルを破棄しても、カウントが変わるだけです。最後のハンドルが破棄されたときに、
  値をちょうど 1 回破棄し、確保した領域を解放します。それがどのスレッドで起きても同じです。
- ハンドルは、複数のスレッドで同時に複製・使用・別スレッドへの送信・破棄をして構いません。
  `MyArc<T>` は、`T` が `Send` と `Sync` の両方を実装するときに `Send` と `Sync` を実装し、
  そうでなければどちらも実装しません（`Arc<T>` と同じです）。
- `MyArc<T>` は `Arc<T>` と同じく `T` について共変です。`MyArc<&'a str>` が必要なところに
  `MyArc<&'static str>` を使えます。たとえば、次の関数はコンパイルが通らなければなりません。

```rust
fn shorten<'a>(a: MyArc<&'static str>) -> MyArc<&'a str> {
    a
}
```

`Arc`、`Rc` など、`std` の参照カウント付きの型は使わないでください。`unsafe` が必要になります。
エディタにある非公開の構造体 `Inner<T>` は案の 1 つにすぎず、変更や削除をして構いません。
未定義動作、データ競合、メモリリークは Miri が検出し、不合格になります。カウントがオーバーフローに
近づくことはありません。

### 例

```rust
let a = MyArc::new(String::from("hello"));
let b = MyArc::clone(&a);
// *b == "hello"、a.len() == 5、MyArc::strong_count(&a) == 2、MyArc::ptr_eq(&a, &b) は true
drop(a);
// MyArc::strong_count(&b) == 1
let c = MyArc::new(String::from("hello"));
// MyArc::ptr_eq(&b, &c) は false

let mut v = MyArc::new(vec![1, 2]);
MyArc::get_mut(&mut v).unwrap().push(3);
let w = MyArc::clone(&v);
// MyArc::get_mut(&mut v) は None
drop(w);
MyArc::get_mut(&mut v).unwrap().push(4);
// *v == [1, 2, 3, 4]

let words = MyArc::new(vec!["a", "bb", "ccc"]);
let (n, joined) = thread::scope(|s| {
    let mine = MyArc::clone(&words);
    let t1 = s.spawn(move || mine.len()); // ハンドルを別スレッドへ移す
    let t2 = s.spawn(|| words.concat()); // &MyArc を別スレッドと共有する
    (t1.join().unwrap(), t2.join().unwrap())
});
// n == 3、joined == "abbccc"、MyArc::strong_count(&words) == 1
```
