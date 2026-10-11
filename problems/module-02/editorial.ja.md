パスも可視性もモジュールを基準に決まります。テンプレートには、それに関する間違いが 6 か所あります。

- `crate` はクレートのルートですが、あなたのファイルはルートではなく、モジュール `crate::user` です
  （ルートはジャッジの `main.rs` です）。そのため `crate::units` は存在しません。`shapes` の中では
  `super` が `crate::user` なので、`super::units::Cm` か、絶対パスの `crate::user::units::Cm` と書きます。
- `checks` の中では、`super` は `crate::user` ではなく `shapes` です。そのため `super::units` も存在しません。
  `super::super::units::Cm` で 2 階層上がるか、`crate::user::units::Cm` を使います。
- タプル構造体のフィールドは、指定しない限り非公開です。`pub struct Cm(u32)` で公開されるのは型だけで、
  中の `u32` は公開されません。`units` の外では、`Cm(width)` で作ることも、`.0` を読むことも、
  パターン `Cm(w)` でマッチすることもできません。フィールドに `pub(super)` を付けると、`crate::user` と
  その中のすべて（`shapes` と `shapes::checks` を含む）から見えるようになります。ハーネスからは見えません。
- ただの `use` は非公開のインポートです。`use checks::is_square;` で `shapes` の中では `is_square` と
  書けるようになりますが、パス `crate::user::shapes::is_square` は非公開です。`pub use` で再エクスポートします。
- 再エクスポートは、元のアイテムより広く公開できません。`pub(super)` の関数を `pub use` するとエラー
  （E0364）になります。`is_square` 自体を `pub` にしてください。モジュール `checks` は非公開のままなので、
  `shapes::checks::is_square` というパスは閉じたままで、公開されたパスは再エクスポートだけになります。
  非公開のモジュールに `pub` のアイテムを置き、`pub use` で再エクスポートするのは、ライブラリの公開パスを
  決めるときによく使う方法です。
- `Rect::new` の `pub(super)` は `crate::user` まで届くので `square` はコンパイルできますが、ハーネスは
  `crate::user` の外にあります。`pub`（ハーネスは同じクレートにあるので `pub(crate)` でも可）にします。

`checks` は `Rect` の非公開フィールド `rect.width` を読んでいますが、これは問題ありません。非公開の
フィールドは、その構造体を定義したモジュールとその子孫すべてから見えます。`checks` は `shapes` の中にあります。

`Cm` のフィールドを `shapes` にだけ見せることはできません。可視性を制限できる先は祖先のモジュールだけで、
`shapes` は `units` の兄弟なので、`pub(in crate::user::shapes)` はエラーになります。いちばん狭くできるのは
両者の共通の親までで、それが `pub(super)`（`pub(in crate::user)` と同じ）の意味です。

エラーは一度にすべては出ません。最初のコンパイルで報告されるのは、解決できない 2 つのインポート、
`is_square` の非公開のインポート、非公開の `new` です。インポートが解決して初めて、コンパイラは `Cm` の
フィールドが非公開であることに気づきます。何を再エクスポートできるかも、`pub use` と書いて初めて検査されます。
報告されたエラーだけを直していくと、3 回のコンパイルが必要です。

### 別解

絶対パスと `pub(crate)` でも書けます。ハーネスは同じクレートにあるので `pub(crate)` で足ります。
ただし再エクスポートを関数より広く公開することはできず、`pub(crate)` の関数を `pub use` するのも E0364 になります。

```rust
pub mod shapes {
    use crate::user::units::Cm;

    pub(crate) use checks::is_square;

    pub struct Rect {
        width: Cm,
        height: Cm,
    }

    impl Rect {
        pub(crate) fn new(width: u32, height: u32) -> Rect {
            Rect { width: Cm(width), height: Cm(height) }
        }

        pub fn area(&self) -> u32 {
            self.width.0 * self.height.0
        }
    }

    mod checks {
        use super::Rect;
        use crate::user::units::Cm;

        pub(crate) fn is_square(rect: &Rect) -> bool {
            let Rect { width: Cm(w), height: Cm(h) } = rect;
            w == h
        }
    }
}

mod units {
    /// A length in whole centimeters.
    pub struct Cm(pub(crate) u32);
}

pub fn square(side: u32) -> shapes::Rect {
    shapes::Rect::new(side, side)
}

pub fn total_area(rects: &[shapes::Rect]) -> u32 {
    rects.iter().map(shapes::Rect::area).sum()
}
```

`checks` は `Cm` を親から取ってくることもできます。`use` 宣言もアイテムの一種なので、`shapes` の非公開の
インポートは、`Rect` と同じように子の `checks` から見えます。

```rust
mod checks {
    use super::{Cm, Rect};

    pub fn is_square(rect: &Rect) -> bool {
        let Rect { width: Cm(w), height: Cm(h) } = rect;
        w == h
    }
}
```
