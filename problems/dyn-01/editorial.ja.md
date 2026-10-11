`Box<dyn Unit>` を使うには、`Unit` が **dyn 互換**（以前の呼び名は「オブジェクト安全」）でなければなりません。
具体的な型を知らなくても、すべてのメソッドを vtable 経由で呼べる必要があります。これを満たさないメソッドが 2 つあります。

- `promoted` は `Self` を返します。`dyn Unit` の向こうにある具体的な型、つまり返り値のサイズがわかりません。
- `beats` は `U` についてジェネリックです。vtable にはメソッドごとに 1 つの項目しかありませんが、
  ジェネリックなメソッドは `U` ごとに別の関数になります。

メソッドに `where Self: Sized` を付けると、そのメソッドはトレイトオブジェクトから除外されます。`dyn Unit` は
`Sized` ではないので、そのメソッドを `dyn Unit` に対して呼べなくなるだけで、トレイトの残りは dyn 互換になります。
`Knight` と `Archer` は `Sized` なので、ハーネスはこれまでどおり両方のメソッドを呼べます。
コンパイラはトレイト全体を検査するので、2 つのうち片方だけを直しても通りません。

### 別解

`beats` はそもそもジェネリックである必要がありません。相手を `&dyn Unit` で受け取れば
そのままで dyn 互換になり、`Box<dyn Unit>` の値に対しても呼べます。この場合 `where Self: Sized` が要るのは
`promoted` だけです。

```rust
pub trait Unit {
    fn name(&self) -> String;
    fn power(&self) -> u32;
    fn promoted(&self) -> Self
    where
        Self: Sized;

    fn beats(&self, other: &dyn Unit) -> bool {
        self.power() > other.power()
    }
}
```

呼び出し側では `&Knight` や `&Archer` が `&dyn Unit` に型強制されるので、呼び出し方は変わりません。
