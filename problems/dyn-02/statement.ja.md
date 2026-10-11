`Validator` はルールのリストを持ちます。ルールは文字列スライスを受け取り、その文字列を受け入れるかどうかを
返す関数です。本体はすべて書かれていますが、このままではコンパイルできません。**構造体の定義とシグネチャだけ**
（`struct` とそのフィールド、`impl` の行、`fn` の行）を書き換えて、ハーネス（`tests.rs` タブ）がコンパイルでき、
テストに合格するようにしてください。本体は変更しないでください。

```rust
pub struct Validator {
    rules: Vec<Box<dyn Fn(&str) -> bool>>,
}

impl Validator {
    pub fn new() -> Self
    pub fn add(&mut self, rule: impl Fn(&str) -> bool)
    pub fn add_boxed(&mut self, rule: Box<dyn Fn(&str) -> bool>)
    pub fn check(&self, s: &str) -> bool
    pub fn passing(&self, items: &[&str]) -> Vec<&str>
}

pub fn banning(words: &[String]) -> Validator
```

- `add(rule)` はクロージャや普通の関数をルールとして追加します。`add_boxed(rule)` はすでに `Box` に入っている
  ルールを追加します。
- `check(s)` は、すべてのルールが `s` を受け入れるとき `true` を返します。ルールのない `Validator` は
  どんな文字列も受け入れます。
- `passing(items)` は、`items` のうち `check` に通るものを元の順に返します。
- `banning(words)` は、ルールを 1 つだけ持つ `Validator` を返します。そのルールは、`words` のどれかを
  部分文字列として含む文字列をすべて拒否します（大文字と小文字は区別します）。`words` が空なら、
  どんな文字列も受け入れます。

ハーネスのルールは、最小の長さ、予約済みの名前のリスト、禁止する単語のリストなど、テストのローカル変数を
借用します。またハーネスは、`Validator` より後に作った文字列を検査したり、`passing` に一時的な `Vec` で
`items` を渡したりします。さらに、`passing` が返した `Vec` を、`Validator` や、そのルールが借用している
変数がなくなった後も使い続けます。

### 例

```rust
let min = 3;
let reserved = vec![String::from("admin"), String::from("root")];
let mut v = Validator::new();
v.add(|s| s.len() >= min);
v.add(|s| !reserved.iter().any(|r| r == s));
v.check("alice") // true
v.check("bo")    // false（短すぎる）
v.check("root")  // false（予約済み）
```

```rust
let words = vec![String::from("spam"), String::from("ad")];
let mut v = banning(&words);
let no_space: Box<dyn Fn(&str) -> bool> = Box::new(|s| !s.contains(' '));
v.add_boxed(no_space);
v.passing(&["hello", "spammer", "bad", "good day"]) // ["hello"]
```

```rust
let items = ["kiwi", "banana", "fig"];
let max = 4;
let mut v = Validator::new();
v.add(|s| s.len() <= max);
let short = v.passing(&items);
drop(v);
// short は ["kiwi", "fig"]
```
