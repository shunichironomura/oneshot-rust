ブログのタグは大文字と小文字を区別しません。`Rust`、`rust`、`RUST` は同じタグですが、タグは書かれたときの
つづりを覚えています。型 `Tag`（エディタに入っています）と、それを使う 2 つの関数を実装してください。

```rust
#[derive(Debug, Clone)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag;
    pub fn as_str(&self) -> &str;
}

impl PartialEq for Tag { /* 実装する */ }
impl Eq for Tag {}
impl Hash for Tag { /* 実装する */ }
impl fmt::Display for Tag { /* 実装する */ }

pub fn unique_tags(words: &[&str]) -> Vec<Tag>
pub fn tag_counts(words: &[&str]) -> HashMap<Tag, usize>
```

- `Tag::new(s)` は、つづりがちょうど `s` のタグを作ります。前後の空白は取り除きません（`s` は空でも
  構いません）。`as_str` はそのつづりを返し、`Display` はそのつづりをそのまま表示します。
- 2 つのタグは、つづりが **ASCII の大文字・小文字を無視して**等しいときに等しいとします（`A`–`Z` と
  `a`–`z` を同じとみなします）。それ以外の文字は、ASCII 以外の英字も含めて、完全に一致しなければ
  なりません。たとえば `Café` は `CAFé` と等しく、`CAFÉ` とは等しくありません。`É` と `é` は別のタグで、
  `Straße` と `STRASSE` も別のタグです。
- `Tag` は `HashSet` や `HashMap` のキーとして正しく使えなければなりません。テストでは 2 つの関数を
  呼ぶだけでなく、タグを `==` や `!=` で比べたり、集合やマップに入れて大文字・小文字の違うつづりで
  探したり、`DefaultHasher` でハッシュ値を計算したりします。
- `unique_tags` は、`words` の中の異なるタグを 1 つずつ、最初に現れた順に並べて返します。
  どのタグも、最初に現れたときのつづりにします。
- `tag_counts` は、各タグが `words` に何回現れるかを数えます。どのキーも、そのタグが最初に現れたときの
  つづりにします。
- `words` は空のこともあります。

### 例

```rust
unique_tags(&["Rust", "go", "RUST", "Go"])
// タグ Rust と go（この順）

tag_counts(&["Rust", "go", "rust", "RUST"])
// {Rust: 3, go: 1}

let mut set = HashSet::new();
set.insert(Tag::new("Rust"))                  // true
set.contains(&Tag::new("rUST"))               // true
set.insert(Tag::new("RUST"))                  // false：等しいタグがすでにある
set.get(&Tag::new("rust")).unwrap().as_str()  // "Rust"
```
