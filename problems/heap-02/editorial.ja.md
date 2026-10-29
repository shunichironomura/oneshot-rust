`BinaryHeap` は**最大ヒープ**です。`pop` は最も大きい要素を返します。各値を `Reverse` で包むと順序が逆になり、
最大ヒープが最小ヒープになって、最も小さい値から取り出せます。
代わりに値の符号を反転する（`-x`）と、`i32::MIN` でオーバーフローします（デバッグビルドではパニック）。

`BinaryHeap<Reverse<i32>>` では、`pop` を繰り返すと整数を昇順に取り出せますが、`into_sorted_vec` は
整数を降順に並べて返します（各整数は `Reverse` に包まれています）。`iter`、`into_iter`、`into_vec` は
ヒープ内部の配列をそのまま走査し、その配列はソートされていないので、先頭の `k` 個を取ると
誤った答えになります。ヒープが空になったら止めれば `k > v.len()` にも対応できます。
一方、`Vec::with_capacity(k)` は `k = usize::MAX` のときにパニックします。

### 別解

最大ヒープに `k` 個の値だけを持たせる方法でも書けます。それまでに見た値のうち小さいほうから
`k` 個を持ち、その中で最大の値が先頭に来ます。それより小さい値が来たら入れ替えます。
この `Reverse` を使わない `BinaryHeap<i32>` では、`into_sorted_vec` は値を昇順に並べて返します。

```rust
pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32> {
    // それまでに見た値のうち小さいほうから k 個を持つ（その中で最大の値が先頭）
    let mut heap = BinaryHeap::new();
    for &x in v {
        if heap.len() < k {
            heap.push(x);
        } else if heap.peek().is_some_and(|&top| x < top) {
            heap.pop();
            heap.push(x);
        }
    }
    heap.into_sorted_vec()
}
```

1 回だけ求めるなら、コピーをソートするのがいちばん簡単です。

```rust
pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32> {
    let mut sorted = v.to_vec();
    sorted.sort();
    sorted.truncate(k);
    sorted
}
```
