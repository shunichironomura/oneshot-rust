どの関数にも間違いが 1 つずつあり、3 つとも `String` と `str` の仕組み（The Book の 8 章）に関するものです。
別々の関数にあるので、コンパイラは 3 つまとめて報告します。

- **`word[0]`**：`str` は整数で添字アクセスできません。中身は UTF-8 のバイト列で、1 文字が 1〜4 バイトを
  占めるので、「0 番目」がバイトなのか文字なのかが決まらず、Rust はどちらかを明示させます。
  `word.chars().next()` は最初の `char` を `Option` で返します。`None` になるのは空文字列のときだけで、
  `split_whitespace` が空の単語を返すことはありません。コンパイルは通るけれど間違っている直し方も
  2 つあります。`word.as_bytes()[0] as char` は `é` の最初のバイト（`0xC3`）を取り出して文字 `Ã` に
  してしまいます。`&word[..1]` はパニックします。`str` の範囲はバイト単位で、バイト位置 1 は `é` の
  途中にあり、文字の境界ではないからです。
- **`dir + "/" + file`**：`String` の `+` は `add(self, s: &str) -> String` を呼びます。左辺を値で受け取り、
  そのバッファに追記して返します。右辺は `&str` でなければなりません。`"/"` は `&str` ですが、`file` は
  `String` です。`&file` と書けば、`&String` が参照外し型強制で `&str` になります。`dir` はこの後
  使わないので、ムーブされても問題ありません。
- **`msg + "!"`**：同じ理由で `+` は `msg` をムーブするので、次の行の `msg.is_empty()` はムーブ済みの値を
  使うことになります。先に空かどうかを調べてから結果を作ります。`msg.clone() + "!"` でもコンパイルは
  通りますが、文字列を無駄にコピーします。

`to_uppercase` は新しい `String` を返し、長さが変わることもあります（`"straße"` は `"STRASSE"` になります）。
`char` ごとに大文字にして、それぞれの結果の最初の 1 文字だけを残すと、2 つ目の `S` が消えてしまいます。

### 別解

`format!` は引数を借用するだけなので、何もムーブしません。The Book でも、複数の文字列を組み合わせるときは
`format!` が勧められています。`String` は `FromIterator<String>` を実装しているので、頭文字はイテレータから
`collect` でも作れます。

```rust
pub fn initials(full_name: &str) -> String {
    full_name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|first| format!("{first}."))
        .collect()
}

pub fn join_path(dir: String, file: String) -> String {
    if dir.is_empty() {
        file
    } else {
        format!("{dir}/{file}")
    }
}

pub fn shout(msg: String) -> String {
    if msg.is_empty() {
        String::new()
    } else {
        format!("{}!", msg.to_uppercase())
    }
}
```

`push` と `push_str` はその場で追記します。`+` が内部でしていることと同じです。`to_uppercase` は
`&self` を取るので、`msg.to_uppercase() + "!"` は `msg` をムーブしません。

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

pub fn shout(msg: String) -> String {
    if msg.is_empty() {
        return String::new();
    }
    msg.to_uppercase() + "!"
}
```
