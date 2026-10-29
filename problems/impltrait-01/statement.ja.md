次のコードはコンパイルが通りません。直してください。

関数 `select` は、スライスから要素を選び、`impl Iterator` として返します。列挙型 `Keep` はすでにエディタに
入っています。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keep {
    Evens,
    Odds,
    All,
}

pub fn select(v: &[i32], keep: Keep) -> impl Iterator<Item = i32> + '_
```

- `select(v, keep)` は、`v` の要素を元の順に返します。`Keep::Evens` なら偶数だけ、
  `Keep::Odds` なら奇数だけ、`Keep::All` ならすべての要素です。
- 負の数も同じように扱います。`-4` と `0` は偶数、`-3` は奇数です。

`Keep` と `select` のシグネチャは変えないでください。

### 例

- `select(&[1, -3, 4], Keep::Odds)` は `1`、`-3` の順に返します。
