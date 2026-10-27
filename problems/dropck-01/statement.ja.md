次のコードはコンパイルが通りません。直してください。

`Tracer` は、作られたときと破棄（ドロップ）されたときに、共有の記録先に 1 行ずつ書き込みます。
`trace` はトレーサーを使って、名前の付いた手順の実行を記録します。

```rust
pub struct Tracer<'a> {
    sink: &'a RefCell<Vec<String>>,
    name: String,
}

impl<'a> Tracer<'a> {
    pub fn new(sink: &'a RefCell<Vec<String>>, name: &str) -> Tracer<'a>;
}

impl Drop for Tracer<'_> { /* ... */ }

pub fn trace(names: &[&str]) -> Vec<String>;
```

- `Tracer::new(sink, name)` は記録先 `sink` に `"begin <name>"` を追加し、トレーサーを返します。
- トレーサーをドロップすると、その記録先に `"end <name>"` が追加されます。
- `trace(names)` は、`names` の順に名前ごとに 1 つずつトレーサーを作ります。どれも同じ記録先に
  書き込みます。次に記録先に `"run"` を追加し、すべてのトレーサーを**作った順に**ドロップして、
  記録先に集まった行を古い順にすべて返します。
- `names` は空のこともあり、同じ名前が何度か出てくることもあります。

`Tracer`、`Tracer::new`、`Drop` の実装は正しいので、変更しないでください。`"end"` の行は、
トレーサーをドロップしたときに書き込まれるものでなければなりません。`trace` のシグネチャも
変えないでください。

### 例

`trace(&["a", "b"])` は `["begin a", "begin b", "run", "end a", "end b"]` を返します。
