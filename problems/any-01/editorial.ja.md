`is`、`downcast_ref`、`downcast_mut`、`downcast` は `dyn Any` と `Box<dyn Any>` の固有メソッドで、
`Any` トレイトのメソッドではありません。そのため `dyn Component` に対しては呼べず、先にコンポーネントを
`dyn Any` にする必要があります。Rust 1.86 からは、これは**トレイトのアップキャスト**と呼ばれる普通の型強制で
できます。`Any` は `Component` のスーパートレイトなので、`&dyn Component` は `&dyn Any` に、
`&mut dyn Component` は `&mut dyn Any` に、`Box<dyn Component>` は `Box<dyn Any>` に変換できます。
具体的な型の情報は残るので、ダウンキャストはコンポーネントの本当の型を調べます。また、`Component: Any` に
よってどのコンポーネント型も `'static` になります。これは `downcast_ref::<C>()` が `C` に求める条件で、
`add` にも必要です（`Box<dyn Component>` は `Box<dyn Component + 'static>` の省略形です）。

罠は、**何を**アップキャストするかです。要素は `Box<dyn Component>` で、`Box<dyn Component>` 自身も
`'static` な型なので `Any` を実装しています。`item` が `&Box<dyn Component>` のとき（`for item in &self.items`
など）、`item as &dyn Any` や `let any: &dyn Any = item;` は箱の中を見ません。`Box` そのものを、
型が `Box<dyn Component>` の `dyn Any` にしてしまいます。これは警告なしにコンパイルが通り、ダウンキャストは
すべて失敗するので、`get` は `None` を、`count` は `0` を返します。`item.type_id()` も同じです。
メソッドの探索は、コンポーネントにたどり着く前に `<Box<dyn Component> as Any>::type_id` を見つけます。
（`item.label()` は問題ありません。`Box` は `Component` を実装していないからです。）コンポーネントは
`&**item` です。1 つ目の `*` で参照の先の `Box` に、2 つ目の `*` で `Box` の中に進みます。

`take_all` ではコンポーネントを取り出す必要があります。`Box<dyn Any>::downcast::<C>()` は
`Result<Box<C>, Box<dyn Any>>` を返し、`*boxed` で `Box<C>` から値を取り出せます。ただし、ダウンキャストに
失敗したときに返ってくるのは `Box<dyn Any>` で、これを `Box<dyn Component>` に戻すことはできません
（トレイトへのダウンキャストはありません）。そこで、先に参照を通して `is::<C>()` で型を確かめ、合っている
箱だけをアップキャストします。こうすれば `unwrap` は失敗しません。`std::mem::take` で要素をすべて取り出し、
2 つの `Vec` に分ければ、どちらも順番が保たれます。

### 別解

`Vec::extract_if` を使うと、合う要素を順に取り除き、残りはそのまま残せます。

```rust
pub fn take_all<C: Component>(&mut self) -> Vec<C> {
    self.items
        .extract_if(.., |item| (&**item as &dyn Any).is::<C>())
        .map(|item| *(item as Box<dyn Any>).downcast::<C>().unwrap())
        .collect()
}
```

Rust 1.86 より前はアップキャストができず、ヘルパートレイトとブランケット実装を使うのが定番の回避策でした。
どの型もブランケット実装から `AsAny` を得るので、ハーネスの `impl Component for ...` はそのままコンパイルできます。
ここでも同じ罠があります。`Box<dyn Component>` もブランケット実装を得るので、`&Box<dyn Component>` に
対して `item.as_any()` を呼ぶと、箱そのものが返ります。`**item` に対して呼んでください。

```rust
use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

pub trait Component: AsAny {
    fn label(&self) -> String;
}

pub struct World {
    items: Vec<Box<dyn Component>>,
}

impl World {
    pub fn new() -> Self {
        World { items: Vec::new() }
    }

    pub fn add<C: Component>(&mut self, c: C) {
        self.items.push(Box::new(c));
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        self.items.iter().find_map(|item| (**item).as_any().downcast_ref::<C>())
    }

    pub fn get_mut<C: Component>(&mut self) -> Option<&mut C> {
        self.items
            .iter_mut()
            .find_map(|item| (**item).as_any_mut().downcast_mut::<C>())
    }

    pub fn count<C: Component>(&self) -> usize {
        self.items.iter().filter(|item| (***item).as_any().is::<C>()).count()
    }

    pub fn take_all<C: Component>(&mut self) -> Vec<C> {
        let mut taken = Vec::new();
        let mut kept = Vec::new();
        for item in std::mem::take(&mut self.items) {
            if (*item).as_any().is::<C>() {
                taken.push(*item.into_any().downcast::<C>().unwrap());
            } else {
                kept.push(item);
            }
        }
        self.items = kept;
        taken
    }

    pub fn labels(&self) -> Vec<String> {
        self.items.iter().map(|item| item.label()).collect()
    }
}
```
