すべての呼び出しに合う具体的な要素の型はなく、参照外し型強制（deref coercion）も役に立ちません。
`&String` を `&str` に変換することはできますが、スライスや配列、`Vec` の中の要素を変えることは
ありません。`&Vec<String>` は `&[String]` にはなっても、`&[&str]` にはなりません。そこで、
スライスのまま要素の型をジェネリックにして `&[S]` とし、本体が使っているトレイトで境界を付けます。
`as_ref()` は `AsRef<str>` のメソッドで、`&str` と `String` のどちらも実装しています。
`&["ab", "c"]` は `&[&str]` に、`&Vec<String>` は `&[String]` に型強制され、`S` は要素の型から
推論されます。

- `words: &[impl AsRef<str>]` では**うまくいきません**。引数の `impl Trait` の型には名前がないので、
  `join_upper::<String>` で指定できる型引数がなくなるからです（「function takes 0 generic
  arguments」）。`&[]` だけでは要素の型がわからないので、ハーネスにはターボフィッシュが必要です。
- `w.as_ref()` があいまいにならないのも境界のおかげです。テンプレートの `w: &String` では `as_ref` の
  候補が複数あり（`String` は `AsRef<str>`、`AsRef<[u8]>`、`AsRef<OsStr>`、`AsRef<Path>` を実装しています）、
  本体まで「type annotations needed」でエラーになります。ジェネリックな関数の中では `S` について
  わかっているのは `S: AsRef<str>` だけなので、`as_ref` は `&str` を返します。
