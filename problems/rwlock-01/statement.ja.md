アプリの設定は、あちこちから、しばしば同時に、複数のスレッドから読まれます。変更はたまにしかありません。
設定を保持する `Store` を実装してください。`Settings` は用意されています（エディタに入っています）。
`Store` のフィールドは自由に決めてください。

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub theme: String,
    pub font_size: u32,
}

pub struct Store {
    // フィールドは自由に決める
}

impl Store {
    pub fn new(initial: Settings) -> Store;
    pub fn read(&self) -> impl Deref<Target = Settings> + '_;
    pub fn update(&self, f: impl FnOnce(&mut Settings)) -> u64;
    pub fn try_update(&self, f: impl FnOnce(&mut Settings)) -> Option<u64>;
    pub fn version(&self) -> u64;
    pub fn snapshot(&self) -> Settings;
}
```

- ハーネスは 1 つの `&Store` を複数のスレッドで共有します（`std::thread::scope` を使います）。
- ストアはバージョン番号を持ち、最初は `1` です。`version()` はこの番号を返します。
- `read()` は現在の設定の*ビュー*を返します。`*view` が設定そのもので、`view.theme` のようにフィールドを
  読めます。ビューが生きている間、設定は変わりません。ビューはいくつでも同時に生きていられます。
  同じスレッドでも、別々のスレッドでも構いません。
- `update(f)` は、生きているビューがすべてなくなり、実行中のほかの更新も終わるまで待ちます。
  それから設定を渡して `f` を 1 回呼び、バージョンに 1 を足して、新しいバージョンを返します。
  複数のスレッドが同時に更新しても、失われる更新はなく、それぞれが異なるバージョンを返します。
- `try_update(f)` は、すぐに始められるなら `update(f)` と同じことをして `Some(新しいバージョン)` を返します。
  ビューが生きているか、ほかの更新が実行中なら、すぐに `None` を返します。このとき `f` は呼ばず、
  何も（バージョンも）変えません。
- `snapshot()` は現在の設定のコピーを返します。このコピーは後の更新で変わりません。
- `version()` と `snapshot()` は、ビューが生きている間にも呼ばれます。
- ハーネスは、ビューを持っているスレッドで `update` を呼ぶことはありません（永遠に待つことになるため）。
  ビューを持っているスレッドがストアのメソッドを呼ぶのは、ほかのスレッドがストアを使っていないときだけです。
  `f` がストアを使うことはなく、`f` がパニックすることもありません。

### 例

```rust
let store = Store::new(Settings { theme: "dark".to_string(), font_size: 12 });
let a = store.read();
let b = store.read();                            // 2 つのビューが同時に生きている
// a.theme == b.theme、b.font_size == 12
store.try_update(|s| s.font_size = 14)           // None（ビューが生きているので f は呼ばれない）
drop(a);
drop(b);
store.try_update(|s| s.font_size = 14)           // Some(2)
store.update(|s| s.theme = "light".to_string())  // 3
store.version()                                  // 3
store.snapshot()                                 // Settings { theme: "light", font_size: 14 }
```
