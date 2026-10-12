3 つの関数にはそれぞれパターンの間違いがあり、コンパイラは 3 つすべてを一度に報告します。

- `first_even`：ふつうの `let` には**論駁不可能な**（irrefutable、どんな値にも必ずマッチする）
  パターンが必要です。`find` が返すのは `Option<&i32>` で、`Some(x)` は `None` にマッチしません（E0005）。
  マッチしなかったときにどうするかをコンパイラは推測しないので、let-else で
  `let Some(x) = ... else { return None };` と書きます。この文より後では `x` に値が入っていなければ
  ならないので、`else` ブロックは `return`、`break`、`continue` やパニックで、必ず外へ抜け出さなければ
  なりません。
- `title_or_default`：先頭のアンダースコアは「未使用の変数」の警告を消すだけです。`_t` も変数なので、
  `Some(_t)` は `title` から `String` を**ムーブ**し、その後の `title.unwrap_or(...)` は一部がムーブされた値を
  使うことになります（E0382）。`_` だけのパターンは違います。束縛ではないので、`Some(_)` は何もムーブせずに
  バリアントだけを確かめ、`title` はそのまま残ります。
- `filled_slots`：`for` ループのパターンも、`let` と同じく論駁不可能でなければなりません（E0005）。
  `for` ループは要素を読み飛ばさないので、`None` が来たときにどうするかを決められません。枠全体を
  変数に束縛し、本体の `if let` で調べます。`slot` は `&Option<u8>` なので、`Some(x)` は `x` を `&u8` として
  束縛し、`*x` で値をコピーして取り出します。

関数やクロージャの引数も `let` や `for` と同じ規則で、パターンは論駁不可能でなければなりません。
論駁可能なパターンを書く場所は、`match`、`if let`、`while let`、let-else です。

### 別解

同じ関数を、論駁可能なパターンの代わりに `Option` のメソッドで書いたものです。
`title_or_default` では、タプルの要素は左から順に評価されるので、`is_some` が `title` を借用し終えてから
`unwrap_or_else` がムーブします。

```rust
pub fn first_even(v: &[i32]) -> Option<i32> {
    v.iter().copied().find(|x| x % 2 == 0)
}

pub fn title_or_default(title: Option<String>) -> (bool, String) {
    (title.is_some(), title.unwrap_or_else(|| String::from("Untitled")))
}

pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    slots
        .iter()
        .enumerate()
        .filter_map(|(i, slot)| slot.map(|x| (i, x)))
        .collect()
}
```

テンプレートの形を残した、より小さな修正でもかまいません。`?` は let-else と同じように早めに `None` を
返します。`&title` に対してマッチする（または `Some(ref _t)` と書く）と、タイトルはムーブされずに借用されます。
ループでは、let-else と `continue` で空の枠を読み飛ばせます。

```rust
pub fn first_even(v: &[i32]) -> Option<i32> {
    let x = v.iter().find(|x| *x % 2 == 0)?;
    Some(*x)
}

pub fn title_or_default(title: Option<String>) -> (bool, String) {
    let mut given = false;
    if let Some(_t) = &title {
        given = true;
    }
    (given, title.unwrap_or(String::from("Untitled")))
}

pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    let mut filled = Vec::new();
    for (i, slot) in slots.iter().enumerate() {
        let Some(x) = slot else { continue };
        filled.push((i, *x));
    }
    filled
}
```
