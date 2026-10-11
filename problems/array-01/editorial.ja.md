配列は `Vec` と同じように要素を所有しています。添字は配列の中の場所を指すだけなので、`(a[i], b[i])` で
`a` から `A` をムーブすることはできません。配列に穴が開いてしまうからです。代わりにコピーするには
`Copy` か `Clone` が必要ですが、`A` と `B` は実装しているとは限りません。`[String::new(); N]` のような
「空の」結果を用意して埋めていく方法もうまくいきません。繰り返し式は値を `N` 回コピーするので、
要素の型が `Copy` でなければならないからです。

解決策は、配列を消費することです。`array.map(f)` は配列を値で受け取り、各要素を順に**値で** `f` に渡して、
結果を同じ長さの新しい配列にまとめます。`shout` はこれだけで書けます。配列は参照だけでなく値のままでも
`IntoIterator` を実装しているので、`for x in arr` や（エディション 2021 からは）`arr.into_iter()` は
所有権のある要素を返します。

`zip_arrays` はこの 2 つを組み合わせます。`a.map` が `a` の要素を 1 つずつ渡し、そのたびに `b` の
イテレータから対応する要素を 1 つ取り出します。どちらの配列も要素は `N` 個なので `unwrap` は失敗しません。
`N = 0` のときは、クロージャは一度も呼ばれません。

`transpose` で扱う `i32` は `Copy` なので、`[[0; R]; C]` で問題ありません。返り値の型では長さが
入れ替わっていることに注意してください。大きさは `m[0].len()` ではなく `R` と `C` から取ります
（または `m` をループで回します）。`R = 0` のとき、`m[0]` はパニックします。

### 別解

`std::array::from_fn(|i| ...)` は添字から配列を作ります。`transpose` なら、数は `m` からコピーして
取り出せるので、これで書けます。

```rust
pub fn transpose<const R: usize, const C: usize>(m: [[i32; C]; R]) -> [[i32; R]; C] {
    std::array::from_fn(|c| std::array::from_fn(|r| m[r][c]))
}
```

`zip_arrays` で `from_fn` を使うなら、クロージャの中で添字を使わず、イテレータから要素を取り出します。

```rust
pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
    let mut pairs = a.into_iter().zip(b);
    std::array::from_fn(|_| pairs.next().unwrap())
}
```

いったん `Vec` に集めてから配列に変換する方法もあります。ただし `.try_into().unwrap()` はコンパイルが
通りません。変換のエラーは `Vec` そのもので、`unwrap` はそれが `Debug` を実装していることを求めますが、
`A` と `B` が `Debug` を実装しているとは限らないからです。代わりに結果で `match` します。

```rust
pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
    let pairs: Vec<(A, B)> = a.into_iter().zip(b).collect();
    match pairs.try_into() {
        Ok(array) => array,
        Err(_) => unreachable!("both arrays have N elements"),
    }
}
```

`shout` では関数が配列を所有しているので、文字列をその場で書き換えることもできます。

```rust
pub fn shout<const N: usize>(mut names: [String; N]) -> [String; N] {
    for name in &mut names {
        *name = name.to_uppercase();
        name.push('!');
    }
    names
}
```

埋めていくための `String` の配列がどうしても欲しいなら、インラインの `const` ブロックを使います。
これはコピーではなく定数として繰り返されるので、`[const { String::new() }; N]` はコンパイルが通ります。
