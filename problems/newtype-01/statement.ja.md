登録フォームでは `alice_1` のようなユーザー名を受け付けます。`String` を包む newtype の `Username` を作ってください。
有効な名前しか持てず、それでいて `&str` を使える場所ならほとんどどこでも使える型にします。型と `impl` ブロックは
エディタに入っているので、中身を実装してください。

```rust
#[derive(Debug, Clone)]
pub struct Username(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameError {
    TooShort,
    TooLong,
    BadStart,
    BadChar(char),
}

impl Username {
    pub fn parse(s: &str) -> Result<Username, NameError>;
}

impl Deref for Username { type Target = str; /* 実装する */ }
impl AsRef<str> for Username { /* 実装する */ }
impl Borrow<str> for Username { /* 実装する */ }
impl fmt::Display for Username { /* 実装する */ }
impl PartialEq<&str> for Username { /* 実装する */ }
impl PartialEq for Username { /* 実装する */ }
impl Eq for Username {}
impl PartialOrd for Username { /* 実装する */ }
impl Ord for Username { /* 実装する */ }
impl Hash for Username { /* 実装する */ }
```

`Username::parse(s)` は、まず `str::trim` と同じように `s` の両端の空白を取り除きます。残った部分が名前です。
名前に対して次のチェックをこの順に行い、最初に失敗したチェックのエラーを返します。

1. 名前が 3 文字（`char`）以上であること。満たさなければ `TooShort`。
2. 名前が 16 文字以下であること。満たさなければ `TooLong`。
3. 最初の文字が ASCII の英小文字（`a` から `z`）であること。満たさなければ `BadStart`。
4. 残りの文字がどれも ASCII の英小文字、ASCII の数字（`0` から `9`）、`_` のいずれかであること。満たさなければ
   `BadChar(c)`。`c` は、このどれでもない最初の文字です。

すべてのチェックに通れば、`parse` はその名前を持つ `Username` を `Ok` で返します。

`Username` はどこでも名前の代わりになります。

- 名前（`str`）へ参照外しできます。そのため `str` のメソッドがそのまま使え（`u.len()`、`u.starts_with("al")`、
  `&u[1..3]` など）、`&str` を受け取る場所に `&u` を渡せます。
- `as_ref()` と `borrow()` は名前を返し、`Display` は名前を表示します（テストでは、ユーザー名のスライスを
  `join(", ")` でつなぐこともします）。
- `u == "alice"`（`Username` と `&str` の比較）は、名前がちょうど `alice` のときに限り真です。
- 2 つのユーザー名は、名前が等しいときに等しくなります。ユーザー名の順序は名前（`str`）の順序と同じです。
  たとえば `al_x` < `alice` < `bob` です。
- テストはユーザー名を `HashSet`、`HashMap`、`BTreeMap` に入れ、`&str` で問い合わせます。
  `set.contains("alice")`、`map.get("bob")`、`map.remove("carol")` などは、その名前のユーザー名を
  見つけなければなりません。

### 例

```rust
let u = Username::parse("  alice_1 ").unwrap();
u.len()              // 7
u.starts_with("al")  // true
u == "alice_1"       // true
format!("<{u}>")     // "<alice_1>"

Username::parse("Al")        // Err(NameError::TooShort)
Username::parse("9lives")    // Err(NameError::BadStart)
Username::parse("bad-name")  // Err(NameError::BadChar('-'))

let mut set = HashSet::new();
set.insert(Username::parse("alice").unwrap());
set.insert(Username::parse("bob").unwrap());
set.contains("bob")          // true
```
