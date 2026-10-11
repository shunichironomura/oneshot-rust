3 つのエラーはどれも**変性**（variance）から来ています。変性とは、あるライフタイムを持つ型を、
別のライフタイムを持つ同じ型として使ってよいのはいつか、という規則です。

`merge` では、ヘルパーの `push_unique` に問題はなく、ローカル変数のほうが原因です。`all` は
`Vec<&'static str>` と宣言されているので、`&mut all` は `&mut Vec<&'static str>` です。`&mut T` は
`T` について*不変*（invariant）なので、これをもっと短い `'a` の `&mut Vec<&'a str>` として渡すことは
できません。それができると、呼ばれた側が、`'static` の文字列しか入っていないはずのベクタに
短命な文字列を入れられてしまうからです。そのため `'a` は `'static` に決まり、`extra` を借用している
`word` はそこまで長く生きられません。`Vec<&'static str>` にはそもそも `word` を入れられないので、
`push_unique` のシグネチャをどう変えても解決しません。

代わりに `Vec<&str>` と書くか、注釈を書かないようにします。共有参照と `Vec` は*共変*（covariant）で、
`&'static str` はもっと短い任意の `'a` の `&'a str` として使えます。そのため
`defaults.to_vec()` が返す `Vec<&'static str>` は、デフォルトにも `extra` の単語にも合う
ライフタイムの `Vec<&'a str>` になれます。引数 `defaults` の型にある `'static` を、
ローカル変数にまで持ち込む必要はありません。

`unique_upper` では、呼び出し側は正しく、ヘルパーが間違っています。`add_word` は
`&'a mut Vec<&'a str>` を受け取るので、可変借用の長さがベクタの中の文字列のライフタイムに
結び付けられます。不変なのでこのライフタイムを縮めて借用を短くすることはできず、しかも
このライフタイムは、後で `seen` を使うすべての箇所を含まなければなりません。そのため最初の
`&mut seen` は `seen` が最後に使われるまで有効なままになり、次の繰り返しでもう一度借用すると
E0499、可変借用中の `seen` を `seen.iter()` で読むと E0502 になります。`push_unique` と同じように、
借用には別のライフタイムを持たせて `fn add_word<'a>(seen: &mut Vec<&'a str>, word: &'a str)` と
します。`&'a mut Thing<'a>` は（`impl<'a> Thing<'a>` の中の `&'a mut self` も含めて）、
意図したものになることはまずありません。

単語をリークさせて `'static` にする方法（`Box::leak`、`String::leak`）もコンパイルは通りますが、
リークしたメモリを Miri が報告します。

### 別解

`add_word` の借用の省略されたライフタイムは、明示的に書くこともできます。2 つのライフタイムは
互いに独立しています。

```rust
fn add_word<'s, 'a>(seen: &'s mut Vec<&'a str>, word: &'a str)
```

`merge` は最初から所有する `String` を集めれば、借用した文字列を使わずに済みます。
`unique_upper` は、見た単語を `HashSet<&str>` で覚える方法もあります。`insert` はすでにある単語に
対して `false` を返すので、そのまま `filter` に使え、ヘルパーもライフタイム注釈も要りません。

```rust
use std::collections::HashSet;

pub fn merge(defaults: &[&'static str], extra: &str) -> Vec<String> {
    let mut all: Vec<String> = defaults.iter().map(|d| d.to_string()).collect();
    for word in extra.split(',').map(str::trim).filter(|w| !w.is_empty()) {
        if !all.iter().any(|a| a == word) {
            all.push(word.to_string());
        }
    }
    all
}

pub fn unique_upper(text: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    text.split_whitespace()
        .filter(|word| seen.insert(*word))
        .map(str::to_uppercase)
        .collect()
}
```
