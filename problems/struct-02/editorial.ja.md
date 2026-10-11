`Default` は手で実装します。`#[derive(Default)]` は各フィールドの既定値を使うので、名前が空文字列、
`retries` が `0` になってしまいます。実装しておけば、`..Default::default()` で書かなかったフィールドを
すべて埋められるので、`with_tags` には既定値と違うところだけを書けば済みます。

構造体更新記法 `..base` は「残りをコピーする」ものではありません。代入と同じように、残りのフィールドを
`base` から**ムーブ**します（`Copy` なフィールドはコピーします）。`quiet` では `base` を所有しているので、
その `name` と `tags` を新しい値にムーブしても問題ありません。一方、`renamed` の `base` は借用です。
`Config { name: name.to_string(), ..*base }` と書くと `*base` から `tags` をムーブすることになり、
参照の先からはムーブできないので E0507 になります。`..base.clone()` なら、作ったばかりのコピーから
フィールドを取り出せます（`..base` だけでは、`Config` ではなく `&Config` なのでやはりコンパイルできません）。

`with_tags` の `tags.iter()` が返す要素は `&&str` です。`to_string()` は使えますが、`map(String::from)` は
コンパイルできません。`String` が実装しているのは `From<&str>` で、`From<&&str>` ではないからです。

`describe` では `cfg` を分解しています。`cfg` は参照なので、`name`、`retries`、`verbose`、`tags` は参照として
束縛されますが、`format!` では値と同じように表示されます。`join(",")` はタグとタグの間にだけ区切りを
入れるので、末尾にカンマは付かず、タグがなければ空文字列になります。

### 別解

`..base.clone()` は古い名前まで複製して、すぐに捨てています。`tags` も明示的に書けばこれを避けられます。
`..*base` に残るフィールドは `retries` と `verbose` だけで、どちらも `Copy` なので、参照の先から
ムーブするものはありません。

```rust
pub fn renamed(base: &Config, name: &str) -> Config {
    Config {
        name: name.to_string(),
        tags: base.tags.clone(),
        ..*base
    }
}
```

先に複製してから、コピーのフィールドを 1 つ書き換える方法もあります。同じように `quiet` も、所有している
引数をその場で書き換えられます。引数に `mut` を付けてもシグネチャは変わりません。

```rust
pub fn renamed(base: &Config, name: &str) -> Config {
    let mut cfg = base.clone();
    cfg.name = name.to_string();
    cfg
}

pub fn quiet(mut base: Config) -> Config {
    base.verbose = false;
    base.retries = 0;
    base
}
```

`with_tags` で `String::from` を使いたいときは、先に `copied()` で `&&str` を `&str` にします。

```rust
pub fn with_tags(tags: &[&str]) -> Config {
    Config {
        tags: tags.iter().copied().map(String::from).collect(),
        ..Config::default()
    }
}
```
