どちらの関数も、1 つのコレクションへの `&mut` を同時に 2 つ持っているので、エラー E0499 になります。
借用チェッカーから見ると、`self.accounts.get_mut(from)` は `src` を最後に使うまでマップ全体を借用し、
`&mut balances[from]` はどの要素かを区別せずに「スライスのどれかの要素」を借用します。
借用チェッカーには `from != to` であることが証明できず、もし等しければ 2 つの参照が同じ値を指してしまいます。

`get_disjoint_mut`（Rust 1.86 で安定化）は、これを実行時に確かめて複数の `&mut` を一度に返します。
ただし、同じキーが重なったときの扱いはマップ版とスライス版で異なります。

- `HashMap::get_disjoint_mut([from, to])` は `[Option<&mut V>; 2]` を返し、存在しないキーは `None` になります。
  2 つのキーが同じエントリを指すと**パニック**するので、`SameAccount` の確認はその前に置いたままにします。
- `<[T]>::get_disjoint_mut([from, to])` は `Result` を返し、添字が重なる（または範囲外の）ときは `Err` になります。
  ここでは `from == to` がちょうど「何もしない」場合なので、`if let Ok(..)` で済みます。

借用を 1 つずつにして手で直すこともできますが、順番に注意が必要です。

- `transfer` では、何かを変更する前にすべての確認を終えなければなりません。`from` から引いた後に
  `?` で `to` を探すと、`to` が存在しないときにお金が消えます。`to` を探す前に残高を確認すると、
  `UnknownAccount` を返すべきところで `InsufficientFunds` を返してしまいます。
- `merge_accounts` を 1 行ずつ書き換えた `balances[to] += balances[from]; balances[from] = 0;` は
  コンパイルが通りますが、`from == to` のときに残高を 2 倍にしてから 0 にしてしまいます。
  借用チェッカーが防いでいた「2 つの参照が同じ要素を指す」場合こそが、壊れる場合なのです。

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

pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize) {
    let amount = std::mem::take(&mut balances[from]);
    balances[to] += amount;
}
```

`std::mem::take` は 0 を残して元の残高を返すので、`from == to` のときは残高がそのまま戻されます。

コンパイラが提案する `split_at_mut` でもスライスへの `&mut` を 2 つ得られますが、
`from < to`、`from > to`、`from == to` を別々に扱う必要があります。
