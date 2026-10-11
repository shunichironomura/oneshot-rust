小さなゲームエンジンが、エンティティのコンポーネントを `World` にまとめて持っています。
コンポーネントには、トレイト `Component` を実装した任意の型の値を使えます。`World` はそれらを
トレイトオブジェクトとして 1 つのリストに入れ、具体的な型を手がかりにして取り出します。
トレイトと構造体はエディタに入っています。`World` のメソッドを実装してください。

```rust
use std::any::Any;

pub trait Component: Any {
    fn label(&self) -> String;
}

pub struct World {
    items: Vec<Box<dyn Component>>,
}

impl World {
    pub fn new() -> Self;
    pub fn add<C: Component>(&mut self, c: C);
    pub fn get<C: Component>(&self) -> Option<&C>;
    pub fn get_mut<C: Component>(&mut self) -> Option<&mut C>;
    pub fn count<C: Component>(&self) -> usize;
    pub fn take_all<C: Component>(&mut self) -> Vec<C>;
    pub fn labels(&self) -> Vec<String>;
}
```

- `new` は空の `World` を返します。`add` は `c` を末尾に追加します。したがって、要素は追加した順に並びます。
- 要素の具体的な型がちょうど `C` であるとき、その要素は「`C` である」とします。型が違う要素は、
  ラベルが同じでも区別します。
- `get` は `C` である最初の要素を返します。なければ `None` です。`get_mut` は同じことを可変参照で行い、
  それを通して変更すると、`World` に入っている要素そのものが変わります。
- `count` は `C` である要素の数を返します。
- `take_all` は `C` である要素をすべて `World` から取り除き、`World` に並んでいた順に値として返します。
  残りの要素は元の順番のまま `World` に残ります。`C` である要素がなければ、空の `Vec` を返し、
  `World` は変わりません。
- `labels` はすべての要素の `label()` を順に返します。

ハーネスは独自のコンポーネント型を定義しています。ラベルがそれぞれ `"pos"`、`"vel"` の `Pos(i32)` と
`Vel(i32)`、ラベルが `"tag"` でサイズが 0 の `Tag`、自分の文字列をラベルにする `Name(String)` です。

### 例

```rust
let mut w = World::new();
w.add(Pos(1));
w.add(Vel(2));
w.add(Pos(3));
w.count::<Pos>()    // 2
w.get::<Vel>()      // Some(&Vel(2))
w.take_all::<Pos>() // [Pos(1), Pos(3)]
w.labels()          // ["vel"]
```

```rust
let mut w = World::new();
w.add(Vel(5));
w.add(Tag);
w.add(Vel(7));
w.get_mut::<Vel>().unwrap().0 += 1;
w.get::<Vel>()      // Some(&Vel(6))
w.get::<Pos>()      // None
w.labels()          // ["vel", "tag", "vel"]
```
