`Rc` は共有アクセスしか与えないので、`self.lines[i] = ...` はコンパイルが通りません。`Rc::make_mut` は
`Rc` 版の clone-on-write です。ほかの `Rc` が同じ値を指していれば、値をこの `Rc` だけが持つ新しい領域に
複製します。そうでなければ、その場の値への `&mut` を返します。`Rc::get_mut` は厳しい版で、値が共有されて
いると `None` を返します。

2 つの `Rc` の `==` は値を比べるので、別々に作った `Doc` でも行が等しければ共有しているように見えてしまいます。
`Rc::ptr_eq` はポインタを比べます。

### 別解

`set_line` は、`make_mut` がしていることを `Rc::strong_count` と `Rc::get_mut` で書くこともできます。
複製するのは `Vec` であることに注意してください。`self.lines.clone()` では `Rc` が複製されるだけです。

```rust
pub fn set_line(&mut self, i: usize, text: &str) {
    if Rc::strong_count(&self.lines) > 1 {
        self.lines = Rc::new(Vec::clone(&self.lines));
    }
    Rc::get_mut(&mut self.lines).unwrap()[i] = text.to_string();
}
```
