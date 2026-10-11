2 つの単語のリストを集合として比べてください。両方にある単語や、片方にしかない単語などを求めます。
また、1 つのリストの中で最初に繰り返される単語も求めてください。

```rust
#[derive(Debug, PartialEq)]
pub struct Overlap {
    pub both: Vec<String>,
    pub only_left: Vec<String>,
    pub only_right: Vec<String>,
    pub exactly_one: Vec<String>,
    pub all: Vec<String>,
}

pub fn overlap(left: &[&str], right: &[&str]) -> Overlap
pub fn first_repeat<'a>(words: &[&'a str]) -> Option<&'a str>
```

構造体 `Overlap` はエディタに入っています。

- 単語はそのままの形で比べます。`"A"` と `"a"` は別の単語です。
- `overlap` は、単語がリストに出てくるかどうかだけを見ます。何回出てくるかは関係ありません。
  返す `Overlap` の各フィールドは次のとおりです。
  - `both`：`left` と `right` の両方にある単語。
  - `only_left`：`left` にあって `right` にない単語。
  - `only_right`：`right` にあって `left` にない単語。
  - `exactly_one`：2 つのリストのうち、ちょうど一方にだけある単語。
  - `all`：少なくとも一方のリストにある単語。
- 5 つのベクタはどれも同じ単語を 2 回以上含まず、単語は文字列を `<` で比べた昇順に並びます。
  この比較はバイト単位なので、大文字は小文字より前になり（`"Z" < "a"`）、ある単語はそれで始まる
  もっと長い単語より前になり（`"app" < "apple"`）、`"z" < "é"` です。
- どちらのリストも空のことがあります。
- `first_repeat` は `words` を先頭から読んでいき、それより前にすでに出てきた単語に初めて出会ったら、
  その単語を返します。2 回以上出てくる単語がなければ `None` を返します。

### 例

```rust
overlap(&["b", "a", "c", "a"], &["d", "c", "b"])
// Overlap {
//     both: ["b", "c"],
//     only_left: ["a"],
//     only_right: ["d"],
//     exactly_one: ["a", "d"],
//     all: ["a", "b", "c", "d"],
// }

first_repeat(&["a", "b", "c", "b", "a"])
// Some("b")（2 つ目の "b" は 2 つ目の "a" より前にある）
```
