Rust では `if` は式です。`1 + 2` と同じように値を持ち、**分岐はすべて同じ型でなければなりません。**
3 つの分岐は `&str` で、最後の 1 つだけが `String` です。リテラルを `.to_string()`（または
`String::from`）で `String` にします。そうすれば `if` をそのまま関数の最後の式にできます。

コンパイラは最後の分岐を `&n.to_string()` と借用するよう提案しますが、そうすると `if` 全体が `&str`
になり、エラーは `String` として返すべき `label` に移るだけです。

### 別解

2 つの余りの組で `match` すれば、FizzBuzz の規則をそのまま書けます。

```rust
pub fn fizz_label(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}
```
