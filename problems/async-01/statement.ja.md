次のコードはコンパイルが通りません。直してください。

テストハーネスは、次の関数を自前の小さなエグゼキュータで実行します
（使えるのは `std` だけで、非同期ランタイムはありません）。

```rust
pub fn greet(name: String) -> impl Future<Output = String>;
```

- `greet(name)` は、結果が `Hello, {name}!`（`{name}` は `name` に置き換えたもの）になる
  Future を返します。`name` は空文字列を含め、どんな文字列でもかまいません。
- この Future は、`greet` から戻った後のいつ `await` してもかまいません。

シグネチャは変えないでください。

### 例

- `greet("Ann".to_string())` の Future の結果は `"Hello, Ann!"` です。
