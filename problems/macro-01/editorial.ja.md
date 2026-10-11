`macro_rules!` のルールは、マッチャーとトランスクライバーの組です。`$($key:expr => $value:expr),+` は
カンマで区切られた 1 つ以上の `key => value` にマッチし、`$(,)?` は末尾にあってもなくてもよいカンマを受け付けます。
トランスクライバーの `$( map.insert($key, $value); )+` は、マッチしたペアごとに 1 回ずつ繰り返されます。
繰り返しの中にはマッチャーのメタ変数がなければなりません。`0usize $(+ 1)*` が「attempted to repeat an
expression containing no syntax variables」で拒否されるのはこのためです。

式として使うマクロは、1 つの式に展開されなければなりません。複数の文を実行したいときはブロックに展開します。
`=> {{ ... }}` の外側の波かっこはトランスクライバーの区切りで、内側の波かっこがブロックです。
その中で宣言したローカル変数（`map`、`max`、`next`）は衛生的（hygienic）で、呼び出し側の変数とは、
名前が同じでもぶつかりません。

一方、展開結果の中のパスは、マクロを呼び出した場所で解決されます。テストは `HashMap` を `use` していないので、
`HashMap::new()` とだけ書くとコンパイルが通りません。`::std::collections::HashMap::new()` ならどこでも使えます。
マクロの名前も同じです。`macro_rules!` のマクロを名前だけで呼べるのは、定義より後ろ（同じモジュールか
その子モジュール）か、`use` した場所だけです。テストが `crate::user::count!` とパスで呼ぶと、展開結果の中の
再帰呼び出し `count!(...)` はテストのモジュールで探され、見つかりません（「cannot find macro `count` in
this scope」）。`$crate` は常にマクロを定義したクレートを指すので、`$crate::user::count!` ならどこからでも呼べます。

`max_of!` は各引数をちょうど 1 回ずつローカル変数に束縛し、次の値が真に大きいときだけ最大値を置き換えます。
そのため、等しい値のうち最も左のものが残ります。`let max = ...` でシャドーイングすれば `let mut` が要らず、
引数が 1 つのときに使われない `mut` の警告も出ません。思いつきやすい
`if $a > max_of!($($rest),+) { $a } else { max_of!($($rest),+) }` は残りの引数を 2 回展開するので、
副作用が 2 回起き、`Copy` でない変数はムーブした後に使うことになります。`Iterator::max` や
`std::cmp::max` は `Ord` を要求し（`f64` は実装していません）、等しい最大値が複数あると**最後**のものを返します。

`count!` は定数式でなければならないので、`vec![...].len()` は使えません。定数の中では `Vec` を作れないからです。
`1usize + $crate::user::count!($($tail)*)` なら定数式になります。トークンツリー 1 つにつき再帰が 1 段深くなり、
再帰の上限は既定で 128 で、この問題には十分です。`$t:tt` でマッチすると、`(b c)` のようなグループ全体が
1 つのトークンツリーになります。

### 別解

`HashMap::from` はペアの配列を受け取ります。配列の要素は左から順に評価され、重複したキーは後のもので上書きされます。
`hashmap! {}` のときも空の配列の型が文脈から推論されるので、ルールは 1 つで済みます。

```rust
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        ::std::collections::HashMap::from([$(($key, $value)),*])
    };
}
pub(crate) use hashmap;
```

`max_of!` も、各引数を 1 回だけローカル変数に束縛し、再帰呼び出しを `$crate` 経由にすれば、再帰で書けます。

```rust
macro_rules! max_of {
    ($only:expr $(,)?) => {
        $only
    };
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let first = $first;
        let rest = $crate::user::max_of!($($rest),+);
        if rest > first { rest } else { first }
    }};
}
pub(crate) use max_of;
```

再帰を使わない `count!` です。各トークンツリーを `stringify!` で文字列にし、スライスの長さを求めます。
`<[&str]>::len` は `const fn` です。代わりに `[...].len()` と書くと、空の配列の要素の型がわからないので、
`count!()` でコンパイルが通りません。

```rust
macro_rules! count {
    ($($t:tt)*) => {
        <[&str]>::len(&[$(stringify!($t)),*])
    };
}
pub(crate) use count;
```
