JSON に似た構文から `Json` の値を作る宣言的マクロ `json!` を書いてください。`Json` 型とその `From` の実装は
用意されています（エディタに入っています）。マクロは、どんな入力でも受け付ける仮のルールとしてエディタに入っており、
その後に、ほかのモジュールからパスで呼べるようにする `pub(crate) use` の行があります。
ルールを書き換え、`pub(crate) use` の行は残してください。

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl From<bool> for Json { /* Json::Bool */ }
impl From<i32> for Json { /* Json::Num */ }
impl From<f64> for Json { /* Json::Num */ }
impl From<&str> for Json { /* Json::Str */ }
impl From<String> for Json { /* Json::Str */ }

macro_rules! json { /* ... */ }
pub(crate) use json;
```

`json!(v)` は値 `v` を 1 つ受け取り、それに対応する `Json` になります。値は次のどれかです。

- `null`。`Json::Null` になります。
- `true` または `false`。`Json::Bool` になります。
- 数値リテラル。`3` のような整数リテラル（`i32`）か、`2.5` のような浮動小数点数リテラル（`f64`）です。
  `Json::Num` になるので、`json!(3)` は `Json::Num(3.0)` です。
- `"s"` のような文字列リテラル。`Json::Str` になります。
- 値の配列 `[v1, v2, ...]`。要素を書かれた順に並べた `Json::Arr` になります。
- オブジェクト `{ "k1": v1, "k2": v2, ... }`。ペアを書かれた順に並べた `Json::Obj` になります。
  キーはどれも文字列リテラルです。同じキーが複数回出てくるときは、すべてのペアを残します。
- かっこで囲んだ、それ以外の Rust の式。`(x - 7)`、`(-1)`、`(name.clone())` などです。
  式の型は `bool`、`i32`、`f64`、`&str`、`String`、`Json` のどれかで、値は `Json::from(式)` です
  （`Json` のときは、その値そのものです）。上のリテラルでない式は、負の数も含めて、かっこで囲む必要があります。

配列とオブジェクトは互いに入れ子にでき、空（`[]`、`{}`）でもよく、最後の要素の後に末尾のカンマがあってもかまいません
（`[1, 2,]`、`{ "a": 1, }`）。かっこで囲んだ式は、どれもちょうど 1 回、書かれた順に評価します。

テストは `crate::user::json!(null)` のようにパスでマクロを呼び出します。呼び出すのは別のモジュールで、
そこには `json` にも `Json` にも `use` 宣言がありません。

### 例

```rust
crate::user::json!({ "a": 1, "b": [true, null, "s"], "c": {} })
// Json::Obj(vec![
//     ("a".to_string(), Json::Num(1.0)),
//     ("b".to_string(), Json::Arr(vec![Json::Bool(true), Json::Null, Json::Str("s".to_string())])),
//     ("c".to_string(), Json::Obj(vec![])),
// ])

let x: i32 = 4;
crate::user::json!([(x - 7), "x"]) // Json::Arr(vec![Json::Num(-3.0), Json::Str("x".to_string())])

crate::user::json!(null) // Json::Null
```
