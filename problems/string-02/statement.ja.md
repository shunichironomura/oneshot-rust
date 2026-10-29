次のコードはコンパイルが通りません。直してください。

`join_path` は、ディレクトリとファイル名をつないで 1 つのパスにします。

```rust
pub fn join_path(dir: String, file: String) -> String
```

- `dir`、`/`、`file` をこの順につなげて返します。
- `dir` が空なら `file` をそのまま返します。
- それ以外の特別扱いはありません。`file` が空でも `/` は付きます（`"usr"` と `""` なら `"usr/"`）。

シグネチャは変えないでください。

### 例

- `join_path("usr".into(), "bin".into())` は `"usr/bin"` です。
