次のコードはコンパイルが通りません。直してください。

`parallel_sum` は `values` を最大 `workers` 個の塊に分け、塊ごとに別々のスレッドで合計を求めて、その総和を返します。

```rust
pub fn parallel_sum(values: &[u64], workers: usize) -> u64
```

シグネチャは変えないでください。`workers` は 1 以上です。複数のスレッドで分担する作りは残してください。

### 例

`parallel_sum(&[1, 2, 3, 4, 5], 2)` は `15` を返します。
