ジャッジはデバッグビルドと同じく、オーバーフロー検査を有効にしてコードをビルドします。演算の結果が型に
収まらないと、リリースビルドのように黙って一周するのではなく、パニック（`attempt to add with overflow`）します。
そのため、素直な `xs.iter().sum::<u8>()` は、合計が 255 を超えたところでパニックします。

`checked_add` はオーバーフローすると `None` を返し、`try_fold` はクロージャが最初に `None` を返したところで
止まります。これで関数は 1 行で書けます。

惜しいけれどパニックする書き方があります。`xs.iter().map(|&x| Some(x)).sum::<Option<u8>>()` が
`None` を返すのは要素に `None` があるときだけで、足し算自体は普通の足し算なので、オーバーフローすると
パニックします。

### 別解

ループでも書けます。`checked_add` が `None` を返したら、`?` がすぐに `None` を返します。

```rust
pub fn sum_checked(xs: &[u8]) -> Option<u8> {
    let mut total: u8 = 0;
    for &x in xs {
        total = total.checked_add(x)?;
    }
    Some(total)
}
```

より広い型で合計してから `u8::try_from(total).ok()` で変換してもかまいませんが、広い型は十分に広くなければ
なりません。長いスライスのバイトを足すと、`u16` でもオーバーフローします。
