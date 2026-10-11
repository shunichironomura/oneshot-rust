次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`Bank` は口座ごとの残高を名前で管理します（`new` に渡す名前はすべて異なります）。
`balance` は口座の残高を返し、存在しない名前なら `None` を返します。
`transfer` は口座 `from` から口座 `to` へ `amount` を移します。次の順に確認します。

1. `from == to` なら `Err(TransferError::SameAccount)` を返します。
2. `from` か `to` が存在しない口座なら `Err(TransferError::UnknownAccount)` を返します。
3. `from` の残高が `amount` より少なければ `Err(TransferError::InsufficientFunds)` を返します。
4. それ以外なら、`from` から `amount` を引いて `to` に足し、`Ok(())` を返します。

`transfer` がエラーを返したときは、どの残高も変わりません。

`merge_accounts` は、添字を口座番号としてスライスに並べた残高を扱います。
`balances[from]` を `balances[to]` に足し、`balances[from]` を 0 にします。
どちらの添字も `balances.len()` 未満です。`from == to` なら、スライスは変わりません。

```rust
#[derive(Debug, PartialEq)]
pub enum TransferError {
    UnknownAccount,
    SameAccount,
    InsufficientFunds,
}

impl Bank {
    pub fn new(accounts: &[(&str, u64)]) -> Self;
    pub fn balance(&self, name: &str) -> Option<u64>;
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), TransferError>;
}

pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize);
```

これらのシグネチャは変えないでください。

### 例

- `let mut bank = Bank::new(&[("a", 10), ("b", 0)]);` のとき、`bank.transfer("a", "b", 7)` は `Ok(())` を返し、
  その後の `bank.balance("a")` は `Some(3)`、`bank.balance("b")` は `Some(7)` です。
  `bank.transfer("a", "a", 1)` は `Err(TransferError::SameAccount)` を返します。
- `[5, 0, 7]` に `from = 0`、`to = 2` で `merge_accounts` を呼ぶと `[0, 0, 12]` になります。
