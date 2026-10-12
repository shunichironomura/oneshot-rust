3 つとも同じ考え方です。まず調べて、本当に何かが変わるときだけコピーします。

`Cow<'_, str>` は `Borrowed(&str)` か `Owned(String)` のどちらかで、`'_` によって借用した結果が `s` に
結びつきます。`s.trim_ascii()` はすでに `s` のスライスなので、内側を変える必要がなければ、何も確保せずに
そのまま `Cow::Borrowed` で返せます。`String` を作るのは、2 文字以上の連なりか、普通の空白でない連なりが
あるときだけです。`str::trim` と `split_whitespace` は Unicode の定義の空白を使い、ASCII より
広い範囲を空白として扱うので注意してください。ASCII 版は `trim_ascii` と `split_ascii_whitespace` です。

`[i32]` の所有版は（`ToOwned` によって）`Vec<i32>` なので、`Cow<'_, [i32]>` は `&[i32]` か `Vec<i32>` を
持ちます。`to_mut` は `&mut Vec<i32>` を返します。`Borrowed` のときは、まずスライスを新しい `Vec` にコピーして
`Owned` に切り替えます。`Owned` のときは、持っている `Vec` をそのまま渡すだけです。無条件に呼ぶと、`max` を
超える要素がなくてもコピーしてしまうので、先に `iter().any(...)` で調べます。`for x in v.iter()` のループの
中で `to_mut` を呼ぶと、ループが `v` を借用したままなのでコンパイルが通りません。

`Rc` は共有アクセスしか与えないので、`self.lines[i] = ...` はコンパイルが通りません。`Rc::make_mut` は
`Rc` 版の clone-on-write です。ほかの `Rc` が同じ値を指していれば、値をこの `Rc` だけが持つ新しい領域に
複製します。そうでなければ、その場の値への `&mut` を返します。`Rc::get_mut` は厳しい版で、値が共有されて
いると `None` を返します。

2 つの `Rc` の `==` は値を比べるので、別々に作った `Doc` でも行が等しければ共有しているように見えてしまいます。
`Rc::ptr_eq` はポインタを比べます。

### 別解

正規化した文字列を作ってから、トリムしたスライスと比べる方法です。短く書けますが、結果を借用で返すときでも
確保が発生します。

```rust
pub fn normalize_ws(s: &str) -> Cow<'_, str> {
    let trimmed = s.trim_ascii();
    let joined = trimmed.split_ascii_whitespace().collect::<Vec<_>>().join(" ");
    if joined == trimmed {
        Cow::Borrowed(trimmed)
    } else {
        Cow::Owned(joined)
    }
}
```

`clamp_all` はバリアントで `match` しても書けます。`Owned` の `Vec` はその場で書き換え、`Borrowed` の
スライスは必要なときだけ新しい `Vec` に置き換えます。`Borrowed` のアームの `s` は `v` を借用していますが、
最後に使うのは右辺なので、`*v` への代入は受け入れられます。

```rust
pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32) {
    match v {
        Cow::Owned(vec) => {
            for x in vec.iter_mut() {
                *x = (*x).min(max);
            }
        }
        Cow::Borrowed(s) => {
            if s.iter().any(|&x| x > max) {
                *v = Cow::Owned(s.iter().map(|&x| x.min(max)).collect());
            }
        }
    }
}
```

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
