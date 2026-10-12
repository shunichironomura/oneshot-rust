`Option` の順序では、`None` はどの `Some` よりも**前**に来ます。ここでの `None` は
「プレリリースなし」で、同じバージョンのどのプレリリースよりも**後**に来なければなりません。そのため、
`#[derive(PartialOrd, Ord)]` も、`self.pre.cmp(&other.pre)` も、最後に `&self.pre` を入れたタプルの比較も、
`1.0.0` を `1.0.0-beta` より前に置いてしまいます。まず数を比べ、次に 2 つの `Option` を `match` で
場合分けします。`Ordering::then` と `then_with` は比較をつなげます。後ろの比較が意味を持つのは、
それより前がすべて `Equal` のときだけです。`then_with` はクロージャを受け取るので、`match` もそのときに
しか実行されません。

`<`、`>`、`partial_cmp` は `PartialOrd` を呼びます。`sort`、`max`、`min`、`BTreeSet` が要求するのは
`Ord` ですが、内部ではどちらのトレイトのメソッドを呼ぶこともあります（たとえば `sort` は `<` で
比べます）。そのため 2 つは一致していなければなりません。`partial_cmp` を `Some(self.cmp(other))` と
書けば、2 つが食い違うことはありません。順序は、derive した `PartialEq`、`Eq`、`Hash` とも
一致していなければなりません。つまり `cmp` が `Equal` を返すのは、
4 つのフィールドがすべて等しいときだけです。すべてのフィールドを見て、ラベルもそのまま比べているので、
この解答はそうなっています。たとえば大文字と小文字を区別せずにラベルを比べると、`1.2.0-RC1` と
`1.2.0-rc1` は `BTreeSet` にとっては「等しい」のに、`HashSet` にとっては等しくなくなります。

`latest_stable` は `filter` のあとに `max` を呼ぶだけです。`&Version` は `Version` を通じて `Ord` を
実装しているので、`max` の結果の `Option<&Version>` をそのまま返せます。

### 別解

キーのタプルで比べることもできます。ただし、ラベルより前に `pre.is_none()` を入れます。
`false < true` なのでプレリリースのないほうが大きくなり、ラベルが効くのは両方にプレリリースがあるときだけに
なります（`None` どうしは等しくなります）。

```rust
impl Version {
    fn key(&self) -> (u64, u64, u64, bool, Option<&str>) {
        (self.major, self.minor, self.patch, self.pre.is_none(), self.pre.as_deref())
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}
```

`std::cmp::Reverse` で `Option` の順序をひっくり返すこともできます。`Reverse(pre.map(Reverse))` は
`None` を最後に置き、ラベルどうしは 2 回反転するので元の順序のままです。

```rust
use std::cmp::Reverse;

fn key(v: &Version) -> (u64, u64, u64, Reverse<Option<Reverse<&str>>>) {
    (v.major, v.minor, v.patch, Reverse(v.pre.as_deref().map(Reverse)))
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        key(self).cmp(&key(other))
    }
}
```
