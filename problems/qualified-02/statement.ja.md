次のコードはコンパイルが通りません。直してください。

登録フォームには、ユーザー名の欄と年齢の欄という 2 つのウィジェットがあります。ウィジェットはそれぞれトレイトで、
どちらのトレイトもメソッドに `get` という名前を使っています。`Form` はこの 2 つを実装しています
（どれもエディタに入っています）。

```rust
pub struct Form {
    pub username: String,
    pub age: u8,
}

pub trait UsernameWidget {
    fn get(&self) -> String;
}

pub trait AgeWidget {
    fn get(&self) -> u8;
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String
```

`summary(w)` は、`w` の 2 つのウィジェットの値を返します。`UsernameWidget` の `get` の結果、空白 1 つ、
括弧で囲んだ `AgeWidget` の `get` の結果をこの順につなげます。

構造体、2 つのトレイト、それらの `Form` への実装、`summary` のシグネチャは変えないでください。
テストハーネスは独自の型にも 2 つのトレイトを実装し（`get` はその型独自のものです）、その型で `summary` を
呼びます。

### 例

- `summary(&Form { username: "rustacean".to_string(), age: 28 })` は `"rustacean (28)"` です。
