トレイト `Describe` は、値を短い文字列で説明します。トレイトとその実装はエディタに入っていて、
メソッドの本体はすべて正しく書かれています。ところが、`impl` の行のいくつかは、作者が最初に必要とした
1 つの型だけに向けて書かれているので、ハーネス（`tests.rs` タブ）のコンパイルが通りません。
**`impl` の行だけ**（各 `impl` ブロックの `{` より前の部分）を書き換えて、ハーネスがコンパイルでき、
テストに合格するようにしてください。トレイトとメソッドの本体は変更せず、`impl` ブロックを追加したり
削除したりもしないでください。

```rust
pub trait Describe {
    fn describe(&self) -> String;
}

impl Describe for i32 { /* ... */ }
impl Describe for bool { /* ... */ }
impl Describe for str { /* ... */ }
impl Describe for String { /* ... */ }
impl Describe for [i32] { /* ... */ }
impl Describe for Vec<i32> { /* ... */ }
impl Describe for Option<i32> { /* ... */ }
impl Describe for (i32, i32) { /* ... */ }
impl<T: Describe> Describe for &T { /* ... */ }
```

本体は、値を次のように説明します。

- `i32` は 10 進数で、`bool` は `yes` か `no` で書きます。
- 文字列（`str` と `String`）は、`{:?}` で整形したとおりに書きます。つまり二重引用符で囲み、
  `"`、`\`、改行などの制御文字はエスケープします。
- スライスと `Vec` は、`[`、要素の説明をカンマと空白で区切って並べたもの、`]` の順に書きます。
  空なら `[]` です。
- `Some(x)` は `some(`、`x` の説明、`)` の順に書きます。`None` は `none` です。
- 組 `(a, b)` は `(`、`a` の説明、カンマと空白、`b` の説明、`)` の順に書きます。
- 参照は、参照先の値と同じように説明します。

スライスや `Vec` の要素、`Option` の中の値、組の 2 つの要素は、`Describe` を実装した任意の型の値に
なりえます。組の 2 つの要素の型は違っていてもかまいません。ハーネスはこれらの型を入れ子にし、
何重にも重なった参照も説明します。また、文字列スライス（`&str`）、スライスへの参照（`&[i32]`）、
トレイトオブジェクト（`&dyn Describe`）、ハーネス自身の型 `Celsius` を要素として使います。

### 例

```rust
"hi".describe()                     // "\"hi\""
vec![Some(1), None].describe()      // "[some(1), none]"
vec![&5, &6].describe()             // "[5, 6]"
(true, vec!["a", "b"]).describe()   // "(yes, [\"a\", \"b\"])"
Some(String::from("x")).describe()  // "some(\"x\")"
```
