次のコードはコンパイルが通りません。直してください。

`make_greeter` はあいさつをする関数を作ります。

```rust
pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String
```

- `make_greeter(greeting)` は、`name` を `"<greeting>, <name>!"` に変える関数を返します。`<greeting>` は
  `greeting` の両端の空白を取り除いたもの（`str::trim` と同じ）です。
- `name` はそのまま使います。
- 返した関数は何回でも呼べます。

シグネチャは変えないでください。関数の本体の中は自由に変えて構いません。

### 例

```rust
let hi = make_greeter(String::from("  Hi "));
hi("Ann")  // "Hi, Ann!"
```
