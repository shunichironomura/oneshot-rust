すべてのハンドルは 1 つのヒープ領域 `Inner<T>` を共有し、そこに値とカウントを並べて置きます。
`new` はそれを `Box` に入れ、`Box::leak`（または `Box::into_raw`）で生ポインタにします。`clone` は
そのポインタをコピーしてカウントに 1 を足します。カウントを 0 にしたハンドルが、`Box::from_raw` で
ポインタを `Box` に戻して破棄します。これで値が破棄され、メモリが解放されます。`fetch_sub` は
変更前のカウントを返すので、`1` を見るハンドルはちょうど 1 つです。カウントを読む操作と減らす操作を
別々に行うと、2 つのスレッドがどちらも「自分は最後ではない」と考えたり（リーク）、どちらも
「自分が最後だ」と考えたり（二重解放）します。`Clone` を derive で済ませることもできません。
`#[derive(Clone)]` は `T: Clone` を要求し、カウントを変えずにポインタをコピーするだけです。

**変性はフィールドの型で決まります。** `*mut X` は `X` について不変なので、`*mut Inner<T>` の
フィールドを持つと `MyArc<&'static str>` は `MyArc<&'a str>` の部分型ではなくなり、`shorten` は
"the struct `MyArc<T>` is invariant over the parameter `T`" というエラーになります。
`NonNull<X>` と `*const X` は共変です。共有されたハンドルが渡すのは `&T` だけなので、共変にしても
健全です（`get_mut` は `&mut MyArc<T>` を必要とし、こちらは不変です）。

**`Send` と `Sync` は自分で実装する必要があります。** 生ポインタ（と `NonNull`）はどちらも実装して
いないからです。どちらにも `T: Send + Sync` が必要です。別々のスレッドのハンドルが同時に `&T` を
渡すので、`T: Sync` が要ります。また、最後のハンドルを破棄したスレッドが `T` を破棄するので、
ハンドルを送ることは値を送ることと同じで、`T: Send` が要ります。ほかのスレッドにある `&MyArc<T>`
からはそのスレッドでハンドルを複製できるので、`Sync` にも同じ境界が要ります。`Send` の境界が
`T: Send` だけだと、`MyArc<Cell<u8>>` を 2 つのスレッドに複製して両方から `set` を呼べてしまい、
安全なコードでデータ競合が起きます。

**メモリ順序。** `fetch_add` や `fetch_sub` は、どの順序を指定してもカウント自体は正しく保ちます。
順序が関係するのは、その前後にあるほかのメモリアクセスです。

- `clone` は `Relaxed` で構いません。呼び出し側がすでにハンドルを持っているので値が消えることはなく、
  ほかのスレッドに見せるべきものもありません。
- `drop` は `Release` で減らします。これで、このスレッドが値に対して行ったことはすべて、この減算より
  前に起きたことになります。最後のハンドルは、解放する前に `Acquire`（ここではフェンス）が必要です。
  これで、ほかのスレッドでの値の使用がすべて解放より前に起きたことになります。`Relaxed`
  （またはフェンスのない `Release`）では、ほかのスレッドが最後に値を使ったことと解放との間に
  順序がなく、Miri はその 2 つのデータ競合を報告します。参照を作ること（Miri のメッセージでは
  "retag"）も使用に数えられます。
- `get_mut` も同じ理由で、カウントを `Acquire` で読む必要があります。ほかのスレッドはハンドルを破棄する
  前に値を読んでいるので、その読み出しは `&mut T` を通した書き込みより前に起きなければなりません。
- `strong_count` はその時点の値を返すだけなので、`Relaxed` で構いません。

Nomicon の `Arc` には、「ドロップチェッカーのため」の `PhantomData<ArcInner<T>>` フィールドもあります。
`MyArc` は自分の `Drop` 実装を持っているので、コンパイラは `MyArc<T>` の破棄で `T` が使われうると
すでにみなしており、ここではこのフィールドがあってもドロップチェッカーの判断は変わりません。
標準ライブラリでは `Drop` 実装に不安定な `#[may_dangle]` 属性を付けてこの前提を外しているので、
このフィールドが必要になります。

### 別解

`AcqRel` の減算 1 回で、`Release` の減算と `Acquire` のフェンスの代わりになります。最後のハンドル
だけでなく、すべての減算が Acquire になるので、少しだけコストが増えます。

```rust
impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        if self.inner().count.fetch_sub(1, Ordering::AcqRel) == 1 {
            // SAFETY: this was the last handle, so nothing else can reach the allocation.
            drop(unsafe { Box::from_raw(self.ptr.as_ptr()) });
        }
    }
}
```

`*const Inner<T>` のフィールドも共変です。ポインタは `Box::into_raw` から得たものなので、`*mut` に
キャストして書き込んでも構いません。

```rust
use std::ops::Deref;
use std::sync::atomic::{self, AtomicUsize, Ordering};

struct Inner<T> {
    count: AtomicUsize,
    value: T,
}

pub struct MyArc<T> {
    ptr: *const Inner<T>,
}

unsafe impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

impl<T> MyArc<T> {
    pub fn new(value: T) -> Self {
        MyArc { ptr: Box::into_raw(Box::new(Inner { count: AtomicUsize::new(1), value })) }
    }

    fn inner(&self) -> &Inner<T> {
        unsafe { &*self.ptr }
    }

    pub fn strong_count(this: &Self) -> usize {
        this.inner().count.load(Ordering::Relaxed)
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if this.inner().count.load(Ordering::Acquire) == 1 {
            Some(unsafe { &mut (*this.ptr.cast_mut()).value })
        } else {
            None
        }
    }

    pub fn ptr_eq(a: &Self, b: &Self) -> bool {
        a.ptr == b.ptr
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        self.inner().count.fetch_add(1, Ordering::Relaxed);
        MyArc { ptr: self.ptr }
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner().value
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        if self.inner().count.fetch_sub(1, Ordering::Release) == 1 {
            atomic::fence(Ordering::Acquire);
            drop(unsafe { Box::from_raw(self.ptr.cast_mut()) });
        }
    }
}
```

値とカウントを別々の `Box` に入れることもできます。その場合、`ptr_eq` ではカウントへのポインタを
比べなければなりません。サイズ 0 の値の `Box` はメモリを確保しないので、2 つの `MyArc::new(())` の
値へのポインタが等しくなるからです。
