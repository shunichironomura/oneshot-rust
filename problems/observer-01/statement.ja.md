サブジェクト（subject）は、購読しているリスナー（listener）にメッセージを送ります。
`subscribe` は `Subscription` を返し、リスナーはその値が生きている間だけ購読を続けます。
`Subscription` をドロップすると購読が解除されます。リスナーは、サブジェクトから呼ばれている最中に
そのサブジェクトを使えます。たとえば別のリスナーを購読させたり、自分の `Subscription` を
ドロップしたりできます。`Subject` と `Subscription` を実装してください。

```rust
pub struct Subject { /* フィールドは自由 */ }
pub struct Subscription { /* フィールドは自由 */ }

impl Subject {
    pub fn new() -> Subject;
    pub fn subscribe(&self, f: impl Fn(&str) + 'static) -> Subscription;
    pub fn notify(&self, msg: &str) -> usize;
    pub fn listener_count(&self) -> usize;
}
```

- `new` は、リスナーのいないサブジェクトを作ります。
- `subscribe(f)` は、リスナー `f` を購読させ、その `Subscription` を返します。リスナーは、その
  `Subscription` がドロップされるまで購読を続けます。ドロップのされ方は問いません。`drop` でも、
  スコープの終わりでも、`let _ = ...` によるその場でのドロップでも、それを持っている値
  （たとえば別のリスナー）がドロップされたときでも同じです。
- `notify(msg)` は、開始した時点で購読しているリスナーを、購読した順に 1 つずつ `msg` を渡して
  呼び出し、呼び出したリスナーの数を返します。自分の番が来る前に購読を解除されたリスナーは
  呼び出しません。
- `listener_count` は、購読しているリスナーの数を返します。
- 購読の解除はすぐに効きます。そのとき実行中のリスナー（たとえば自分の `Subscription` を
  ドロップしたリスナー）でも同じです。それ以降、`listener_count` はそのリスナーを数えず、どの
  `notify` もそのリスナーを呼び出しません。実行中の呼び出しの中から始めた `notify` も同じです。
  実行中の呼び出し自体は、そのまま続きます。
- リスナーは実行中に、同じサブジェクトの `subscribe`、`notify`、`listener_count` を呼び出しても、
  どの `Subscription` をドロップしても構いません。どれもパニックしてはいけません。
  リスナーの中から始めた `notify` も同じ規則に従い、その時点で購読しているリスナーを呼び出します。
  実行中のリスナーも、購読したままなら呼び出します。したがって、`notify` の最中に購読した
  リスナーはその `notify` では呼び出されませんが、すぐに購読した状態になります。`listener_count` は
  そのリスナーを数え、その後に始まった `notify` は、リスナーの中から始めたものでも、そのリスナーを
  呼び出します。
- サブジェクトがリスナーを生かし続けてはいけません。`Subscription` がドロップされたら、その
  リスナーと、リスナーがキャプチャしたものすべてを、その場で解放します。ただし、`notify` の実行中に
  ドロップされた場合は、実行中のいちばん外側の `notify` から戻るまでリスナーが生きていても構いません。
- リスナーは自分のサブジェクトの `Rc` を持っていても構いません。`Subscription` はサブジェクトより
  長生きしても構いません。そのときにドロップしても、ほかには何も起きません。メモリリークが
  あってはいけません。

上のコードはエディタに入っています。シグネチャは変えないでください。2 つの構造体のフィールドは
自由です。

### 例

```rust
let subject = Subject::new();
let a = subject.subscribe(|msg| println!("a got {msg}"));
let _b = subject.subscribe(|msg| println!("b got {msg}"));
subject.listener_count(); // 2
subject.notify("hi");     // "a got hi"、"b got hi" の順に出力し、2 を返す
drop(a);
subject.listener_count(); // 1
subject.notify("bye");    // "b got bye" を出力し、1 を返す
```

最初に呼ばれたときに、自分で購読をやめるリスナーの例です。

```rust
let subject = Rc::new(Subject::new());
let _a = subject.subscribe(|msg| println!("a got {msg}"));

let slot: Rc<RefCell<Option<Subscription>>> = Rc::new(RefCell::new(None));
let (me, s) = (Rc::clone(&slot), Rc::clone(&subject));
let once = subject.subscribe(move |msg| {
    drop(me.borrow_mut().take()); // 自分の Subscription をドロップする
    println!("once got {msg}, {} left", s.listener_count());
});
*slot.borrow_mut() = Some(once);

let _c = subject.subscribe(|msg| println!("c got {msg}"));
subject.notify("1"); // "a got 1"、"once got 1, 2 left"、"c got 1" を出力し、3 を返す
subject.notify("2"); // "a got 2"、"c got 2" を出力し、2 を返す
```
