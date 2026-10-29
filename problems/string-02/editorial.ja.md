`String` の `+` は `add(self, s: &str) -> String` を呼びます（The Book の 8 章）。左辺を値で受け取り、
そのバッファに追記して返します。右辺は `&str` でなければなりません。`"/"` は `&str` ですが、`file` は
`String` です。`&file` と書けば、`&String` が参照外し型強制で `&str` になります。

`+` は左辺をムーブするので、`dir + ...` の後では `dir` を使えません。ここでは後で使わないので問題ありません。
後でも使うなら、必要なことを先に調べたりコピーしたりしておきます。`dir.clone() + ...` でもコンパイルは
通りますが、文字列を無駄にコピーします。

### 別解

`format!` は引数を借用するだけなので、何もムーブしません。The Book でも、複数の文字列を組み合わせるときは
`format!` が勧められています。

```rust
pub fn join_path(dir: String, file: String) -> String {
    if dir.is_empty() {
        file
    } else {
        format!("{dir}/{file}")
    }
}
```

`push` と `push_str` はその場で追記します。`+` が内部でしていることと同じです。

```rust
pub fn join_path(dir: String, file: String) -> String {
    if dir.is_empty() {
        return file;
    }
    let mut path = dir;
    path.push('/');
    path.push_str(&file);
    path
}
```
