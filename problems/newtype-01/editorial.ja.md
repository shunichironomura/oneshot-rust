`set.contains("alice")` や `map.get("bob")` に `&str` を渡せるのは `Borrow<str>` のおかげですが、これには約束があります。
`Username` とその `borrow()` とで、`Eq`、`Ord`、`Hash` が同じ答えを返さなければなりません。ハッシュセットは
`"alice"` を探すとき、`&str` を `str` の `Hash` でハッシュします。一方、セットの中のユーザー名は `Username` の
`Hash` で置き場所が決まっています。2 つのハッシュが違えば、`contains` は見当違いの場所を探して、たいていは
`false` を返します。コンパイルエラーにもパニックにもならず、UB ですらなく、ただ結果が間違います。`BTreeMap` にも
順序について同じ問題があります。`get("bob")` は `"bob"` とキーを `str` の `Ord` で比べますが、木は `Username` の
`Ord` で並べられています。

約束を守るいちばん簡単な方法は derive です。フィールドが 1 つの構造体では、derive した `PartialEq`、`Eq`、
`PartialOrd`、`Ord`、`Hash` はそのフィールドのものを呼ぶだけで、`String` の比較とハッシュは `str` とまったく同じです。
それぞれが `self.0` に処理を任せる形なら、手で実装しても構いません（別解を参照）。一方、同じくらいもっともらしく
見えても、検索を壊すコードがあります。`state.write(self.0.as_bytes())` では、`str` の `Hash` がバイト列の後に
書き込む `0xff` の 1 バイトが抜けます。`self.0.as_bytes().hash(state)` はスライスの長さを先にハッシュします。
長さを先に比べる順序は `str` の順序と食い違います。

「`&str` のように使える」振る舞いの残りは `Deref<Target = str>` が担います。`deref` の中の `&self.0` は
`&String` ですが、参照外し型強制によってシグネチャどおりの `&str` になります。`u.len()` のようなメソッド呼び出しは、
`Username` に `len` がないので自動で参照外しされて `str` のメソッドになります。また、`&str` を受け取る関数の引数や
`let s: &str = &u;` のように `&str` が求められる場所では、`&u` が `&str` に型強制されます。ただし `S: AsRef<str>`
のようなジェネリックな引数では型強制は起きません。`AsRef<str>` も実装するのはそのためです。`DerefMut` はないので、
`Username` の名前を書き換えて無効にすることはできません。

`PartialEq<&str>` では右辺が `&str` なので、`other` は `&&str` です。`self.0 == other` は、`String: PartialEq<&&str>`
がないのでコンパイルが通りません。1 回参照外しして `self.0 == *other` と書けば、`String` と `&str` の比較になります。

`parse` では文字数を `chars().count()` で数えます。`len()` はバイト数なので、`"aé"`（2 文字、3 バイト）が
`TooShort` になりません。文字の判定には、ASCII だけを表す範囲パターン `'a'..='z'` と `'0'..='9'`（または
`is_ascii_lowercase` と `is_ascii_digit`）を使います。`is_lowercase` や `is_alphanumeric` は、`é` のような文字や
`٣` のような数字も受け入れてしまいます。

`fmt::Display::fmt(&self.0, f)` は `str` の `Display` に処理を任せるので、`{:>10}` のような幅や寄せの指定も効きます。
`f.write_str(&self.0)` でも同じ名前を表示できますが、これらの指定は無視されます（テストでは使いません）。

### 別解

テンプレートの比較とハッシュの `impl` を手で埋めることもできます。どれも `String` に処理を任せるので、
derive したものと同じく `str` と一致します。`#[derive(Debug, Clone)]` とテンプレートの `use`
（`Ordering`、`Hash`、`Hasher`）はそのまま残してください。

```rust
impl PartialEq for Username {
    fn eq(&self, other: &Username) -> bool {
        self.0 == other.0
    }
}

impl Eq for Username {}

impl PartialOrd for Username {
    fn partial_cmp(&self, other: &Username) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Username {
    fn cmp(&self, other: &Username) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for Username {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
```

`parse` は 1 つのループで全部の文字を調べることもできます。最初の文字は英小文字でなければならず、
英小文字であれば、残りの文字に対するチェックにも通ります。

```rust
pub fn parse(s: &str) -> Result<Username, NameError> {
    let name = s.trim();
    match name.chars().count() {
        0..=2 => return Err(NameError::TooShort),
        3..=16 => {}
        _ => return Err(NameError::TooLong),
    }
    for (i, c) in name.chars().enumerate() {
        if i == 0 && !c.is_ascii_lowercase() {
            return Err(NameError::BadStart);
        }
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return Err(NameError::BadChar(c));
        }
    }
    Ok(Username(name.to_string()))
}
```

1 行の本体は `Deref` を通して書くこともできます。`**self` と `**other` はどちらも `str` で、`self` は
`write_str` が受け取る `&str` に型強制されます。

```rust
impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

impl PartialEq<&str> for Username {
    fn eq(&self, other: &&str) -> bool {
        **self == **other
    }
}
```
