模範解答では、各リスナーを所有するのはその `Subscription` で、サブジェクトは `Weak` しか持ちません。
こうすると、購読の解除は `Rc` をドロップするだけで済みます。`Subscription` には `Drop` の実装も、
サブジェクトへ戻る手段も要りません。リスナーは最後の `Rc` がなくなった時点で解放されます。
サブジェクトは何も所有しないので、`Rc<Subject>` をキャプチャしたリスナーがあっても循環参照になりません。

クロージャはそれぞれ名前のない別々の型を持つので、リストにはトレイトオブジェクトとして格納します。
`Listener { callback: Rc::new(f) }` では、フィールドの型が `Rc<dyn Fn(&str)>` なので、コンパイラが
`Rc<F>` を `Rc<dyn Fn(&str)>` に変換します（サイズ不定型への型強制）。`callback(msg)` で、`Rc` 越しに
クロージャを呼び出せます。`Rc<dyn Fn(&str)>` は `Rc<dyn Fn(&str) + 'static>` の略記です。
`subscribe` が `'static` を要求するのはこのためで、呼び出し側から借用したものはリストに
入れられません。

**リストを借用したままリスナーを呼ばないでください。** メソッドは `&self` を受け取るので、リストは
`RefCell` に入れます。次のようなループはコンパイルできます。

```rust
for weak in self.listeners.borrow().iter() {
    if let Some(listener) = weak.upgrade() {
        (listener.callback)(msg);
    }
}
```

しかし `borrow()` が返す `Ref` はループの間ずっと生きているので、リスナーが `subscribe` を呼んだ時点で、
その中の `borrow_mut()` が "RefCell already borrowed" でパニックします。`RefCell` は借用規則を実行時に
検査します。呼び出し元へ呼び返すコールバックは、その規則を破る典型的な例です。そこで `notify` は、
まずリストをコピーし（`Weak` の `Vec` なので安く複製できます）、誰かを呼ぶ前に借用を終えます。
このコピーのおかげで、`notify` の最中に購読したリスナーがその `notify` で呼ばれることもありません。
添字でループして毎回借用し直す方法でもパニックは避けられますが、新しいリスナーまで呼んでしまいますし、
`subscribe` が解除済みの要素を取り除くと添字がずれます。借用したまま、`notify` から戻るまで
`subscribe` が新しいリスナーを待たせておく方法でもパニックは避けられます。しかし `notify` の最中に
購読したリスナーも、すぐに購読した状態になります。`listener_count` はそのリスナーを数え、その後に
始めた `notify` は、リスナーの中から始めたものでも、そのリスナーを呼び出さなければなりません。

**各 `Weak` のアップグレードは、その番が来てから行います。** こうすると、同じ `notify` の中で先に
購読を解除されたリスナーはアップグレードに失敗し、呼ばれずに済みます。最初にコピー全体を
アップグレードしてしまうと、すべてのリスナーが最後まで生き続け、解除されたリスナーまで呼んでしまいます。

**`Listener` でコールバックを包む理由。** すぐに思いつくのは、サブジェクトに `Weak<dyn Fn(&str)>` を
持たせ、`Subscription` に `Rc<dyn Fn(&str)>` を持たせる設計です。これはリスナーが自分で購読を解除する
までは動きます。リスナーの実行中、`notify` は `upgrade` が返した `Rc` を持っています。実行中の
クロージャを解放するわけにはいかず、そもそも `Weak` のままでは呼び出せないので、持たざるを得ません。
そのため `Subscription` がなくなっても、強参照カウントは 1 のままです。`strong_count() > 0` では
そのリスナーも数えてしまいます（2 つ目の例は `3 left` と出力します）。リスナーの中から始めた
`notify` も、そのリスナーをもう一度アップグレードして呼び出してしまいます。強参照カウントは
`Subscription` の中の `Rc` だけではなく、すべての `Rc` を数えるからです。

模範解答は「購読中」と「生きている」を分けています。`Subscription` は唯一の `Rc<Listener>` を持ち、
`Listener` は `Rc<dyn Fn(&str)>` を持ちます。`notify` は `Weak<Listener>` をアップグレードして
コールバックを複製し、呼び出す前に `Rc<Listener>` をドロップします。リスナーの実行中に生かしておくのは
コールバックだけなので、`Subscription` がドロップされた瞬間から、`Weak<Listener>` は
アップグレードできなくなります。

`subscribe` は `retain` で解除済みの `Weak` を取り除くので、リストが増え続けることはありません。
`listener_count` は、まだアップグレードできるものを数えます。`Subscription` に付けた `#[must_use]` に
より、`subject.subscribe(f);` のような呼び出しには警告が出ます。これでは購読がその場で
解除されてしまうからです。

### 別解

サブジェクトがリスナーを所有し、各 `Subscription` が `drop` で自分の要素を取り除く方法もあります。
このとき `Subscription` にはリストへ戻る手段が必要です。ここではサブジェクトの中の
`Rc<RefCell<...>>` への `Weak` を持たせています。この `Weak` は、サブジェクトがなくなると
アップグレードできなくなります。注意点は 2 つです。

- `drop` では、リストを借用したまま、取り除いたリスナーをドロップしてはいけません。リスナーは別の
  `Subscription` を持っていることがあり、その `drop` がもう一度リストを借用するからです。単純な
  `listeners.borrow_mut().retain(...)` はその場でリスナーをドロップするので、
  "RefCell already borrowed" でパニックします。この別解では、要素を取り除いて借用を終えてから、
  それをドロップしています。
- `notify` は ID だけをコピーし、各リスナーの番が来る直前に、改めてリストから探します。

```rust
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

type Listeners = RefCell<Vec<(u64, Rc<dyn Fn(&str)>)>>;

pub struct Subject {
    listeners: Rc<Listeners>,
    next_id: Cell<u64>,
}

pub struct Subscription {
    listeners: Weak<Listeners>,
    id: u64,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        // The subject may already be gone.
        let Some(listeners) = self.listeners.upgrade() else { return };
        let removed = {
            let mut listeners = listeners.borrow_mut();
            let index = listeners.iter().position(|(id, _)| *id == self.id);
            index.map(|i| listeners.remove(i))
        };
        // Drop the listener only now that the list is no longer borrowed: it may own other
        // subscriptions, whose `drop` borrows the list again.
        drop(removed);
    }
}

impl Subject {
    pub fn new() -> Subject {
        Subject { listeners: Rc::new(RefCell::new(Vec::new())), next_id: Cell::new(0) }
    }

    pub fn subscribe(&self, f: impl Fn(&str) + 'static) -> Subscription {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        self.listeners.borrow_mut().push((id, Rc::new(f)));
        Subscription { listeners: Rc::downgrade(&self.listeners), id }
    }

    pub fn notify(&self, msg: &str) -> usize {
        let ids: Vec<u64> = self.listeners.borrow().iter().map(|(id, _)| *id).collect();
        let mut called = 0;
        for id in ids {
            // Look the listener up again: it may have been unsubscribed in the meantime.
            let listener = self
                .listeners
                .borrow()
                .iter()
                .find(|(i, _)| *i == id)
                .map(|(_, f)| Rc::clone(f));
            if let Some(f) = listener {
                f(msg);
                called += 1;
            }
        }
        called
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.borrow().len()
    }
}
```

サブジェクトには `Weak<dyn Fn(&str)>` を持たせたまま、`Subscription` が生きているかどうかを別の目印で
判定する方法もあります。目印は、`Subscription` だけが持つ `Rc<()>` です。

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone)]
struct Entry {
    /// Only the `Subscription` holds this `Rc`: it can be upgraded while the listener is
    /// subscribed, even while `notify` holds the callback.
    subscribed: Weak<()>,
    callback: Weak<dyn Fn(&str)>,
}

pub struct Subject {
    entries: RefCell<Vec<Entry>>,
}

pub struct Subscription {
    _subscribed: Rc<()>,
    _callback: Rc<dyn Fn(&str)>,
}

impl Subject {
    pub fn new() -> Subject {
        Subject { entries: RefCell::new(Vec::new()) }
    }

    pub fn subscribe(&self, f: impl Fn(&str) + 'static) -> Subscription {
        let subscribed = Rc::new(());
        let callback: Rc<dyn Fn(&str)> = Rc::new(f);
        let mut entries = self.entries.borrow_mut();
        entries.retain(|e| e.subscribed.strong_count() > 0);
        entries.push(Entry {
            subscribed: Rc::downgrade(&subscribed),
            callback: Rc::downgrade(&callback),
        });
        Subscription { _subscribed: subscribed, _callback: callback }
    }

    pub fn notify(&self, msg: &str) -> usize {
        let snapshot = self.entries.borrow().clone();
        let mut called = 0;
        for entry in snapshot {
            if entry.subscribed.strong_count() == 0 {
                continue;
            }
            if let Some(callback) = entry.callback.upgrade() {
                callback(msg);
                called += 1;
            }
        }
        called
    }

    pub fn listener_count(&self) -> usize {
        self.entries.borrow().iter().filter(|e| e.subscribed.strong_count() > 0).count()
    }
}
```
