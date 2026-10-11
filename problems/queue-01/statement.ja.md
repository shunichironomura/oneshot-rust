次のコードはコンパイルが通り、手元で実行すると「それっぽく」動くこともあります。
しかし**未定義動作（UB）**を含んでおり、Miri で実行すると UB として検出されます。

`Queue<T>` は、単方向連結リストで作った先入れ先出しのキューです。先頭のノードに加えて、
最後のノードを指す生ポインタを持っているので、`push` でリストをたどらずに済みます。

```rust
pub struct Queue<T> { /* 非公開のフィールド */ }

impl<T> Queue<T> {
    pub fn new() -> Self;
    pub fn push(&mut self, elem: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn peek(&self) -> Option<&T>;
    pub fn peek_back(&self) -> Option<&T>;
    pub fn len(&self) -> usize;
}
```

- `new` は空のキューを作ります。
- `push` は `elem` をキューの末尾に追加します。
- `pop` は先頭の要素を取り除いて返します。キューが空なら `None` を返します。
- `peek` は先頭の要素を、`peek_back` は末尾の要素を、取り除かずに返します。キューが空なら
  どちらも `None` を返します。要素が 1 つだけのときは、どちらもその要素を返します。
- `len` は要素の数を返します。
- `pop` で空になったキューも、続けて使えます。
- `pop` が返した要素は呼び出し側のものです。キューが破棄されるときは、中に残っている要素を
  それぞれちょうど 1 回ずつ破棄します。
- キューは普通の値です。テストでは、キューをムーブしたり、関数から返したり、ほかのキューに
  入れたり（`Queue<Queue<String>>`）します。
- 未定義動作やメモリリークは Miri が検出し、不合格になります。

UB を取り除いて、キューが正しく動くようにしてください。公開されているシグネチャは変えないでください。
非公開のフィールドと `Node` は変更してかまいません。`unsafe` を使い続けてもかまいません。
ただし、設計はそのままにしてください。各要素はヒープ上にあるそれぞれ専用のノードに格納し、
ノードは先頭から末尾へとつなぎ、キューは最後のノードを指すポインタを持ちます。こうして、
どの操作も定数時間で済むようにします。要素を `Vec` や `VecDeque` などの標準のコレクションに
格納してはいけません。

### 例

```rust
let mut q = Queue::new();
q.push(String::from("a"));
q.push(String::from("b"));
q.push(String::from("c"));
// peek() は Some("a")、peek_back() は Some("c")、len() は 3
q.pop(); // Some("a")
q.pop(); // Some("b")
q.push(String::from("d"));
// peek() は Some("c")、peek_back() は Some("d")、len() は 2
q.pop(); // Some("c")
q.pop(); // Some("d")
q.pop(); // None
// peek() と peek_back() は None、len() は 0
```

```rust
let item = Rc::new(7);
let mut q = Queue::new();
for _ in 0..3 {
    q.push(Rc::clone(&item));
}
// Rc::strong_count(&item) は 4
let first = q.pop(); // Some(7)
drop(q); // キューに残っている 2 つのクローンを破棄する
// Rc::strong_count(&item) は 2（`item` と `first`）
```
