`Option` の順序では、`None` はどの `Some` よりも**前**に来ます。ここでの `None` は
「プレリリースなし」で、同じバージョンのどのプレリリースよりも**後**に来なければなりません。そのため、
`#[derive(PartialOrd, Ord)]` も、`self.pre.cmp(&other.pre)` も、最後に `&self.pre` を入れたタプルの比較も、
`1.0.0` を `1.0.0-beta` より前に置いてしまいます。まず数を比べ、次に 2 つの `Option` を `match` で
場合分けします。`Ordering::then` と `then_with` は比較をつなげます。後ろの比較が意味を持つのは、
それより前がすべて `Equal` のときだけです。`then_with` はクロージャを受け取るので、`match` もそのときに
しか実行されません。

`<`、`>`、`partial_cmp` が使うのは `PartialOrd`、`sort`、`max`、`min`、`BTreeSet` が使うのは `Ord` です。
`partial_cmp` を `Some(self.cmp(other))` と書けば、2 つが食い違うことはありません。順序は、derive した
`PartialEq`、`Eq`、`Hash` とも一致していなければなりません。つまり `cmp` が `Equal` を返すのは、
4 つのフィールドがすべて等しいときだけです。すべてのフィールドを見て、ラベルもそのまま比べているので、
この解答はそうなっています。たとえば大文字と小文字を区別せずにラベルを比べると、`1.2.0-RC1` と
`1.2.0-rc1` は `BTreeSet` にとっては「等しい」のに、`HashSet` にとっては等しくなくなります。

解析では、`u64::from_str`（`parse::<u64>()` が呼ぶもの）は数字だけかどうかを確かめてはくれません。
先頭の `+` を受け付けるので、`"+1"` や `"1.+2"` が通ってしまいます。まず各部分が ASCII 数字だけで
できていることを確かめ、オーバーフローは `parse` にエラーとして報告させます。同じように、
`char::is_alphanumeric` は `β`、`é`、`٣` などあらゆる Unicode の文字や数字を受け付けます。ASCII だけを
認めるのは `is_ascii_alphanumeric` と `is_ascii_digit` です。ラベルには `.` が含まれることがあるので
（`3-rc.1`）、`.` で分ける**前に**最初の `-` でラベルを切り離します。`Result<Vec<_>, _>` に `collect`
すると最初の不正な部分で止まり、そのあとスライスパターンで数が 1 ～ 3 個の場合だけを受け付けます。

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

`parse` を使わない場合は、`checked_mul` と `checked_add` で数字を足し上げます。オーバーフローは
パニックではなく `None` になります。`to_digit(10)` は `0` ～ `9` しか受け付けないので、符号も
はじかれます。

```rust
fn parse_number(s: &str) -> Result<u64, ParseVersionError> {
    if s.is_empty() {
        return Err(ParseVersionError);
    }
    s.chars()
        .try_fold(0u64, |n, c| {
            let digit = c.to_digit(10)?;
            n.checked_mul(10)?.checked_add(u64::from(digit))
        })
        .ok_or(ParseVersionError)
}
```
