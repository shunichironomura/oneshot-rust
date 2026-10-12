ほとんどの仕事は `BTreeSet` がしてくれます。リストを集合に集めると重複がなくなり、`BTreeSet` は
いつも昇順に要素を返すので、`intersection`、`difference`、`symmetric_difference`、`union` が返す単語は
最初から重複がなく、昇順に並んでいます。文字列の大小はバイト単位で決まるので、問題文が求める順序そのものです。

最初の提出では、次の 2 つの型の違いでつまずきがちです。

- `left.iter()` が返す要素は `&&str`（スライスの各要素への参照）なので、
  `left.iter().collect::<BTreeSet<&str>>()` はコンパイルが通りません。`.copied()` で `&str` にします。
- 集合演算が返す要素も集合の要素への参照で、ここでは `&&str` です。`Vec<String>` に集めるには
  `.map(|w| w.to_string())` のような変換が必要です。`.cloned()` では `&str` に戻るだけです。

`HashSet` でも解けますが、要素を返す順序は決まっていないので、どのベクタも最後に並べ替える必要があります。
並べ替えを忘れても、小さな例ではたまたま昇順になることがあり、間違いに気づきにくくなります。

### 別解

`HashSet` を使い、最後に各ベクタを並べ替えます。

```rust
use std::collections::HashSet;

pub fn overlap(left: &[&str], right: &[&str]) -> Overlap {
    let left: HashSet<&str> = left.iter().copied().collect();
    let right: HashSet<&str> = right.iter().copied().collect();
    Overlap {
        both: sorted(left.intersection(&right)),
        only_left: sorted(left.difference(&right)),
        only_right: sorted(right.difference(&left)),
        exactly_one: sorted(left.symmetric_difference(&right)),
        all: sorted(left.union(&right)),
    }
}

fn sorted<'a>(words: impl Iterator<Item = &'a &'a str>) -> Vec<String> {
    let mut words: Vec<String> = words.map(|w| w.to_string()).collect();
    words.sort();
    words
}
```

2 つの `&BTreeSet` に演算子 `&`、`-`、`^`、`|` を使っても同じ集合が求まります。結果は新しい `BTreeSet`
（ここでは `BTreeSet<&str>`）なので、簡単にベクタにできます。

```rust
use std::collections::BTreeSet;

pub fn overlap(left: &[&str], right: &[&str]) -> Overlap {
    let left: BTreeSet<&str> = left.iter().copied().collect();
    let right: BTreeSet<&str> = right.iter().copied().collect();
    Overlap {
        both: owned(&left & &right),
        only_left: owned(&left - &right),
        only_right: owned(&right - &left),
        exactly_one: owned(&left ^ &right),
        all: owned(&left | &right),
    }
}

fn owned(words: BTreeSet<&str>) -> Vec<String> {
    words.into_iter().map(String::from).collect()
}
```
