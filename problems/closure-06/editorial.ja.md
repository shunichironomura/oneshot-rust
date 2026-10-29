`make_greeter` には問題が 2 つあり、コンパイラは両方を一度に報告します。

- 返したクロージャは `make_greeter` から戻った後に使われるので、関数が所有しているもの（引数も
  含みます）を借用してはいけません（E0373「closure may outlive the current function」）。
  `move` が必要です。
- ただし、`move` だけでは足りません。`let greeting = greeting.trim();` の後の `greeting` は、
  引数の `String` を借用する `&str` を指しています。この `String` は関数から戻るときに破棄されます。
  その参照をクロージャにムーブしても、クロージャは `String` を借用したままです
  （E0597「`greeting` does not live long enough」）。`to_string()` で、空白を取り除いた文字列を
  所有する `String` にしてから、それをムーブします。

### 別解

`String` をまるごとムーブして、呼ぶたびに空白を取り除くこともできます。

```rust
pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    move |name| format!("{}, {name}!", greeting.trim())
}
```
