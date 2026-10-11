型の中でライフタイムを書かない `Box<dyn Trait>` は `Box<dyn Trait + 'static>` の意味になります。これが
`Box` に入ったトレイトオブジェクトの、既定のライフタイム境界です。そのため、フィールド
`Vec<Box<dyn Fn(&str) -> bool>>` には何も借用していない（または `'static` なデータしか借用していない）
ルールしか入れられず、`rule` が何かを借用しているかもしれない `add` は E0310 でエラーになります。
コンパイラは `impl Fn(&str) -> bool + 'static` を提案します。これで `add` はコンパイルできますが、
ローカル変数を借用するクロージャは追加できなくなり、今度は `words` を借用するクロージャを持つ `banning` が
エラーになります。

ルールに借用させるには、`Validator` にライフタイム引数を持たせ、それをトレイトオブジェクトの境界に使います。
`Box<dyn Fn(&str) -> bool + 'a>` は「少なくとも `'a` の間は生きているものだけを借用してよい、`Box` に入った
ルール」という意味です。メソッドから `'a` を書けるように、`impl` ブロックは `impl<'a> Validator<'a>` にします。

- `add` は `impl Fn(&str) -> bool + 'a` を受け取ります。
- `add_boxed` は `Box<dyn Fn(&str) -> bool + 'a>` を受け取ります。変えなくてもコンパイルは通りますが、
  `'static` なルールしか受け取れません。ハーネスは `suffix` を借用するクロージャを `Box` に入れて渡します。
  `'static` は `'a` より長く生きるので、ただの `Box<dyn Fn(&str) -> bool>` も引き続き渡せます。
- `banning` は `Validator<'_>`、つまり `words` を借用する `Validator` を返します。

`'a` を付けるのはトレイトオブジェクトだけで、ルールの引数には付けません。`dyn Fn(&str) -> bool` は高階の
`dyn for<'x> Fn(&'x str) -> bool` の省略形で、どんなライフタイムの文字列でも受け取れます。
`dyn Fn(&'a str) -> bool + 'a` にすると、`check` にはルールが借用しているものと同じだけ生きる文字列しか
渡せなくなりますが、ハーネスは `Validator` より後に作った文字列を検査します。

最後に、`passing` の返り値の省略されたライフタイムは `&self` から来ますが、返すのは `items` の中の文字列で、
`Validator` の中身ではありません。文字列に専用のライフタイムを付けて、
`passing<'s>(&self, items: &[&'s str]) -> Vec<&'s str>` とします。コンパイラはこのライフタイムに `'a` という
名前を提案しますが、`impl<'a>` の中ではその名前はすでに使われている（E0496）ので、別の名前にします。
`&'s self` にすると返り値が `Validator` を借用し続けるので、`drop(v)` がエラーになります。`&'s [&'s str]` に
するとスライスを借用することになり、あるケースではそれが一時的な `Vec` です。`&'a str` にすると、返り値を
ルールが借用している変数より長く使えません。

### 別解

`add` は `impl Trait` の代わりに型引数でも書けます。`banning` はライフタイムに名前を付けても書けます。

```rust
pub fn add<F>(&mut self, rule: F)
where
    F: Fn(&str) -> bool + 'a,
{
    self.rules.push(Box::new(rule));
}
```

```rust
pub fn banning<'w>(words: &'w [String]) -> Validator<'w> {
    let mut v = Validator::new();
    v.add(move |s| !words.iter().any(|w| s.contains(w.as_str())));
    v
}
```
