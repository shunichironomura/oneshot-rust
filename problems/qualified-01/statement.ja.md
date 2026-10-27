次のコードはコンパイルが通りません。直してください。

登録フォームには、ユーザー名の欄と年齢の欄という 2 つのウィジェットがあります。ウィジェットはそれぞれトレイトで、
どちらのトレイトもメソッドに `get` と `kind` という名前を使っています。`Form` はこの 2 つと 3 つ目のトレイト
`Labeled` を実装していて、さらに `label` という固有メソッドも持っています（どれもエディタに入っています）。

```rust
pub struct Form {
    pub username: String,
    pub age: u8,
}

pub trait UsernameWidget {
    fn get(&self) -> String;
    fn kind() -> &'static str { "username" }
}

pub trait AgeWidget {
    fn get(&self) -> u8;
    fn kind() -> &'static str { "age" }
}

pub trait Labeled {
    fn label(&self) -> String; // Form では "user:<username>"
}

impl Form {
    pub fn label(&self) -> String; // "form of <username>"
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String
pub fn kinds<W: UsernameWidget + AgeWidget>() -> (&'static str, &'static str)
pub fn next_age(form: &Form) -> u8
pub fn tag(form: &Form) -> String
```

- `summary(w)` は、`w` の 2 つのウィジェットの値を返します。`UsernameWidget` の `get` の結果、空白 1 つ、
  括弧で囲んだ `AgeWidget` の `get` の結果をこの順につなげます（例：`"rustacean (28)"`）。
- `kinds::<W>()` は、`W` の `UsernameWidget` の実装の `kind` と、`W` の `AgeWidget` の実装の `kind` を、
  この順に返します。`Form` なら `("username", "age")` です。
- `next_age(form)` は、フォームの年齢に 1 を足した値を返します。年齢がすでに `255` なら `255` のままです。
- `tag(form)` は、`Labeled` の実装がフォームに付けるラベル `"user:<username>"` を返します。

構造体、3 つのトレイト、それらの `Form` への実装、固有メソッド `Form::label`、4 つの関数のシグネチャは
変えないでください。テストハーネスは独自の型にも 2 つのウィジェットトレイトを実装し（`get` はその型
独自のもので、`kind` も上書きしていることがあります）、その型で `summary` と `kinds` を呼びます。

### 例

`f = Form { username: "rustacean".to_string(), age: 28 }` とすると、次のようになります。

- `summary(&f)` は `"rustacean (28)"` です。
- `kinds::<Form>()` は `("username", "age")` です。
- `next_age(&f)` は `29`、`tag(&f)` は `"user:rustacean"` です。
