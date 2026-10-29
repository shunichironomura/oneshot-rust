このエラーは**変性**（variance）から来ています。変性とは、あるライフタイムを持つ型を、
別のライフタイムを持つ同じ型として使ってよいのはいつか、という規則です。

ヘルパーの `push_unique` に問題はなく、ローカル変数のほうが原因です。`all` は
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

単語をリークさせて `'static` にする方法（`Box::leak`、`String::leak`）もコンパイルは通りますが、
リークしたメモリを Miri が報告します。

### 別解

最初から所有する `String` を集めれば、借用した文字列を使わずに済みます。

```rust
pub fn merge(defaults: &[&'static str], extra: &str) -> Vec<String> {
    let mut all: Vec<String> = defaults.iter().map(|d| d.to_string()).collect();
    for word in extra.split(',').map(str::trim).filter(|w| !w.is_empty()) {
        if !all.iter().any(|a| a == word) {
            all.push(word.to_string());
        }
    }
    all
}
```
