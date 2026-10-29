次のコードはコンパイルが通りません。直してください。

口座の残高が、添字を口座番号としてスライスに並んでいます。`merge_accounts` は、口座 `from` のお金を
すべて口座 `to` に移します。

```rust
pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize);
```

- `balances[from]` を `balances[to]` に足し、`balances[from]` を 0 にします。
- どちらの添字も `balances.len()` 未満です。
- `from == to` なら、スライスは変わりません。
- 残高の合計は `u64` に収まるので、足し算がオーバーフローすることはありません。

シグネチャは変えないでください。

### 例

- `[5, 0, 7]` に `from = 0`、`to = 2` で `merge_accounts` を呼ぶと `[0, 0, 12]` になります。
