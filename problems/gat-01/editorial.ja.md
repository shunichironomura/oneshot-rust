`type Item;` では、impl は `next` のすべての呼び出しに対して**1 つ**の型を決めるので、その型から個々の呼び出しの
`&mut self` の借用を指すことはできません。テンプレートは手元にあるライフタイム `'s` と `'t` を使っていますが、
要素はそれほど長く生きません。窓は `&mut self` を通して `self.slice` から借用し直したものですし、行は
`self.buf` の中にあって、次の呼び出しで上書きされます。rustc はどちらの `Some(...)` についても
「lifetime may not live long enough」を報告します。窓を `&'s mut [T]` にすることも*できません*。
隣り合う窓は重なっているので、呼び出し側が同じ要素への `&mut` を 2 つ持ててしまうからです。
標準ライブラリのスライスに `chunks_mut` はあっても `windows_mut` がないのはこのためです。

ジェネリック関連型（GAT）を使うと、要素の型にライフタイム引数を持たせて、呼び出しごとに決められます。

```rust
pub trait Lender {
    type Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```

`Self::Item<'_>` は `&mut self` のライフタイムを使うので、要素は破棄されるまでレンダーを可変で借用し続けます。
これがこのレンダーを安全にしています。`while let Some(w) = windows.next()` では、各 `w` は次の呼び出しの前に
なくなるので、2 つの窓が同時に存在することはありません。レンダーはそれぞれ別々に借用されるので、
別々のレンダーの要素は一緒に使えます。

`where Self: 'a` は必須です。`next` は `Item<'a>` を `&'a mut self` と一緒に使っていて、これは `Self: 'a` を
意味します。rustc は、この境界を書いていない GAT を拒否します（「missing required bound on `Item`」）。
impl の側にも必要です。書かないと、`&'a str` の場合でも rustc は E0477（「the type `Numbered<'t>` does not
fulfill the required lifetime」）を報告します。そのため impl は次のようになります。

```rust
type Item<'a> = &'a mut [T] where Self: 'a;  // WindowsMut
type Item<'a> = &'a str where Self: 'a;      // Numbered
```

`next` はどちらも `Option<Self::Item<'_>>` を返します。

トレイトにライフタイム引数を付ける方法（`trait Lender<'a>`）は使えません。`count_all` が `L: Lender` と
宣言されているからです。ジェネリックなコードのあちこちに `for<'a> Lender<'a>` が必要にもなります。

### 別解

`next` のライフタイムに名前を付け、`Self: 'a` が表している境界を書き下し、impl では要素の具体的な型を書く
こともできます。

```rust
pub trait Lender {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

impl<'s, T> Lender for WindowsMut<'s, T> {
    type Item<'a> = &'a mut [T] where 's: 'a, T: 'a;

    fn next(&mut self) -> Option<&mut [T]> {
        let window = self.slice.get_mut(self.pos..self.pos + self.size)?;
        self.pos += 1;
        Some(window)
    }
}

impl<'t> Lender for Numbered<'t> {
    type Item<'a> = &'a str where 't: 'a;

    fn next(&mut self) -> Option<&str> {
        let line = self.lines.next()?;
        self.n += 1;
        self.buf.clear();
        write!(self.buf, "{}: {}", self.n, line).unwrap();
        Some(&self.buf)
    }
}
```
