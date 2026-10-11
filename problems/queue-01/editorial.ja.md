テンプレートのコメントには、`Box` をムーブしてもノード自体はヒープ上の同じ場所にあるので、
`raw_tail` は有効なままだと書かれています。前提は正しいのですが、結論が間違っています。
ポインタが使えるのは、そのメモリにアクセスする*許可*がまだ残っている間だけです。Miri の
エイリアシングモデル（Stacked Borrows）は `Box` を `&mut` と同じように扱います。`Box` をムーブしたり
関数に渡したりすると、その `Box` がノードにたどり着く唯一の手段だと表明したことになり、それ以前に
`Box` から作ったポインタはすべて無効になります。テンプレートでは `raw_tail` を作った直後に
`self.head = Some(new_tail)`（または `(*self.tail).next = Some(new_tail)`）で `Box` をムーブしているので、
次の `push` は無効になったポインタを通して書き込むことになります。

修正の方針は、ノードを `Box` で所有するのをやめることです。`Box::into_raw` で新しいノードを生ポインタに変え、
それ以降は `head`、各 `next`、`tail` をすべてただの生ポインタにします。こうすれば、ムーブされて
`tail` を無効にするような `Box` は残りません。`pop` では `Box::from_raw` で先頭のノードを 1 回だけ
`Box` に戻し、要素を取り出してノードを解放します。

コンパイラから見ると生ポインタは何も所有していないので、次の 2 つのことが必要になります。

- ノードは自動では解放されなくなります。`impl Drop` がないと、空でないキューを破棄したときに
  ノードと要素がリークし、Miri がそれを報告します。キューが空になるまで `pop` を繰り返せば、
  すべてのノードが解放され、すべての要素がちょうど 1 回ずつ破棄されます。
- `pop` で最後のノードを解放したら、`tail` を null に戻さなければなりません。そうしないと、次の
  `push` が解放済みのメモリに書き込みます。`peek_back` は `as_ref` を使っているので、null ポインタに
  対しては `None` を返します。

`Box` によるリンクを残したままの中途半端な修正もうまくいきません。ムーブした後でポインタを作る方法
（`self.tail = &mut **slot`）は、その `Box` が再びムーブされるまでしか持ちません。`pop` は 2 番目のノードの
`Box` を `head` へムーブする（`self.head = head.next`）ので、そのノードが最後のノードなら `tail` は
無効になります。要素が 1 つだけのキューをムーブする（関数から返す、別のキューに `push` する）だけでも、
`head` フィールド、つまり最後のノードを所有する `Box` がムーブされます。先にポインタを作ってから
`Box::into_raw(boxed)` や `mem::forget(boxed)` を呼ぶ方法も、同じ理由で失敗します。`Box` を関数に渡すことは
ムーブだからです。`Box::into_raw` が返したポインタを使ってください。

### 別解

`Option<NonNull<Node<T>>>` を使うと、「ノードがない」ことを null ポインタではなく `None` で表せます。
`Box::leak` も `Box::into_raw` と同じようにノードを手放します。`*mut T` と違って `NonNull<T>` は共変なので、
`Box` を使った場合と同じく、この `Queue<&'static str>` は `Queue<&'a str>` が必要なところで使えます。

```rust
use std::ptr::NonNull;

pub struct Queue<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    elem: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: None, len: 0 }
    }

    pub fn push(&mut self, elem: T) {
        let node = NonNull::from(Box::leak(Box::new(Node { elem, next: None })));
        match self.tail {
            // SAFETY: `tail` points to the last node, which is alive.
            Some(mut tail) => unsafe { tail.as_mut().next = Some(node) },
            None => self.head = Some(node),
        }
        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head?;
        // SAFETY: the node was leaked in `push` and is reclaimed only here, once.
        let node = unsafe { Box::from_raw(head.as_ptr()) };
        self.head = node.next;
        if self.head.is_none() {
            self.tail = None;
        }
        self.len -= 1;
        Some(node.elem)
    }

    pub fn peek(&self) -> Option<&T> {
        // SAFETY: the node is alive while `self` is borrowed.
        self.head.map(|node| unsafe { &node.as_ref().elem })
    }

    pub fn peek_back(&self) -> Option<&T> {
        // SAFETY: the node is alive while `self` is borrowed.
        self.tail.map(|node| unsafe { &node.as_ref().elem })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
```

生ポインタをまったく使わずに、最後のノードを `Rc` で直前のノードと `tail` が共有する方法もあります。
次のノードへのリンクは共有された `Rc` を通して書き換えるので `RefCell` が必要ですが、`elem` は
セルの外に置くので、`peek` はそのまま `&T` を返せます。`pop` が先頭のノードを `head` から（最後のノード
だったなら `tail` からも）外せば、そのノードを指す `Rc` は手元の 1 つだけになり、`Rc::into_inner` で
ノードを取り出せます。循環はないので、`Drop` の実装は要りません。その代わり、実行時に参照カウントと
借用のチェックのコストがかかります。

```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct Queue<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
    len: usize,
}

struct Node<T> {
    elem: T,
    next: RefCell<Option<Rc<Node<T>>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: None, len: 0 }
    }

    pub fn push(&mut self, elem: T) {
        let node = Rc::new(Node { elem, next: RefCell::new(None) });
        match self.tail.take() {
            Some(old_tail) => *old_tail.next.borrow_mut() = Some(Rc::clone(&node)),
            None => self.head = Some(Rc::clone(&node)),
        }
        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take()?;
        self.head = old_head.next.borrow_mut().take();
        if self.head.is_none() {
            // `tail` also points to the last node; let go of it too.
            self.tail = None;
        }
        self.len -= 1;
        // Nothing else refers to the node any more, so it can be unwrapped.
        Rc::into_inner(old_head).map(|node| node.elem)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail.as_deref().map(|node| &node.elem)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
```

この問題の外では、キューには `std::collections::VecDeque`（伸長可能なリングバッファ）を使うのが普通です。
