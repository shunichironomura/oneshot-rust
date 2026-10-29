次のコードはコンパイルが通りません。直してください。

登録フォームには、ユーザー名の欄と年齢の欄という 2 つのウィジェットがあります。ウィジェットはそれぞれトレイトで、
どちらのトレイトにも、ウィジェットの種類を表す `kind` という名前の関連関数があります。既定の `kind` は型ごとに
上書きできます。`Form` はこの 2 つのトレイトを実装しています（どれもエディタに入っています）。

```rust
pub struct Form;

pub trait UsernameWidget {
    fn kind() -> &'static str { "username" }
}

pub trait AgeWidget {
    fn kind() -> &'static str { "age" }
}

pub fn kinds<W: UsernameWidget + AgeWidget>() -> (&'static str, &'static str)
```

`kinds::<W>()` は、`W` の `UsernameWidget` の実装の `kind` と、`W` の `AgeWidget` の実装の `kind` を、
この順に返します。

構造体、2 つのトレイト、それらの `Form` への実装、`kinds` のシグネチャは変えないでください。
テストハーネスは独自の型にも 2 つのトレイトを実装し（そこでは `kind` を上書きしていることがあります）、
その型で `kinds` を呼びます。

### 例

- `kinds::<Form>()` は `("username", "age")` です。
