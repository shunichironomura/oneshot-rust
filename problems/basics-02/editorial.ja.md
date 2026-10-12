Rust では `if` と `loop` は式です。`1 + 2` と同じように値を持ちます。ここでのうっかりは、どちらも
その値が何かを忘れたことから起きています。

1. **`if` の分岐はすべて同じ型でなければなりません。** 3 つの分岐は `&str` で、最後の 1 つだけが
   `String` です。リテラルを `.to_string()`（または `String::from`）で `String` にします。そうすれば
   `if` をそのまま関数の最後の式にできます。コンパイラは最後の分岐を `&n.to_string()` と借用する
   よう提案しますが、そうすると `if` 全体が `&str` になり、エラーは `String` として返すべき
   `label` に移るだけです。
2. **`break` は値を持てます。** `loop` は式で、その値は `break value` で決まります。値のない `break;`
   の値は `()` なので、`break x;` と書く必要があります。

### 別解

書き方を変えれば、どちらのうっかりもそもそも起きません。2 つの余りの組で `match` すれば FizzBuzz の
規則をそのまま書けますし、次の倍数はループを使わずに計算できます。`limit / step * step` は `limit`
以下で最大の `step` の倍数なので、答えはそれより `step` だけ大きい数です。

```rust
pub fn fizz_label(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

pub fn first_multiple_above(step: u64, limit: u64) -> u64 {
    (limit / step + 1) * step
}
```
