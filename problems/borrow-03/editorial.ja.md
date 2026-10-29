`transfer` は、1 つのマップへの `&mut` を同時に 2 つ持っているので、エラー E0499 になります。
借用チェッカーから見ると、`self.accounts.get_mut(from)` は `src` を最後に使うまでマップ全体を借用します。
借用チェッカーには `from != to` であることが証明できず、もし等しければ 2 つの参照が同じ値を指してしまいます。

`HashMap::get_disjoint_mut`（Rust 1.86 で安定化）は、これを実行時に確かめて複数の `&mut` を一度に返します。
`get_disjoint_mut([from, to])` は `[Option<&mut V>; 2]` を返し、存在しないキーは `None` になります。
2 つのキーが同じエントリを指すと**パニック**するので、`SameAccount` の確認はその前に置いたままにします。

借用を 1 つずつにして手で直すこともできますが、順番に注意が必要です。`transfer` では、何かを変更する前に
すべての確認を終えなければなりません。`from` から引いた後に `?` で `to` を探すと、`to` が存在しないときに
お金が消えます。`to` を探す前に残高を確認すると、`UnknownAccount` を返すべきところで
`InsufficientFunds` を返してしまいます。

### 別解

`get_disjoint_mut` を使わずに、エントリを 1 つずつ借用し、先にすべてを確認する方法です。

```rust
pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), TransferError> {
    if from == to {
        return Err(TransferError::SameAccount);
    }
    if !self.accounts.contains_key(to) {
        return Err(TransferError::UnknownAccount);
    }
    let src = self.accounts.get_mut(from).ok_or(TransferError::UnknownAccount)?;
    if *src < amount {
        return Err(TransferError::InsufficientFunds);
    }
    *src -= amount;
    *self.accounts.get_mut(to).unwrap() += amount;
    Ok(())
}
```
