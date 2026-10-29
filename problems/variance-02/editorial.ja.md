このエラーは**変性**（variance）から来ています。変性とは、あるライフタイムを持つ型を、
別のライフタイムを持つ同じ型として使ってよいのはいつか、という規則です。

呼び出し側は正しく、ヘルパーが間違っています。`add_word` は `&'a mut Vec<&'a str>` を受け取るので、
可変借用の長さがベクタの中の文字列のライフタイムに結び付けられます。`&mut T` は `T` について*不変*
（invariant）なので、このライフタイムを縮めて借用を短くすることはできず、しかもこのライフタイムは、
後で `seen` を使うすべての箇所を含まなければなりません。そのため最初の `&mut seen` は `seen` が
最後に使われるまで有効なままになり、次の繰り返しでもう一度借用すると E0499、可変借用中の `seen` を
`seen.iter()` で読むと E0502 になります。エラーはループの中を指しますが、原因はヘルパーのシグネチャにあります。

借用には別のライフタイムを持たせて `fn add_word<'a>(seen: &mut Vec<&'a str>, word: &'a str)` とします。
こうすると `seen` の借用は呼び出しの間だけで終わり、単語は `text` のライフタイムを保ちます。
`&'a mut Thing<'a>` は（`impl<'a> Thing<'a>` の中の `&'a mut self` も含めて）、意図したものになることは
まずありません。

### 別解

借用の省略されたライフタイムは、明示的に書くこともできます。2 つのライフタイムは互いに独立しています。

```rust
fn add_word<'s, 'a>(seen: &'s mut Vec<&'a str>, word: &'a str)
```

見た単語を `HashSet<&str>` で覚える方法もあります。`insert` はすでにある単語に対して `false` を返すので、
そのまま `filter` に使え、ヘルパーもライフタイム注釈も要りません。

```rust
use std::collections::HashSet;

pub fn unique_upper(text: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    text.split_whitespace()
        .filter(|word| seen.insert(*word))
        .map(str::to_uppercase)
        .collect()
}
```
