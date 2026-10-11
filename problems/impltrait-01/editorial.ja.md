`impl Trait` を返す関数も、返すのは**ひとつの**具体的な型です。その型は関数の本体が決め、呼び出し側からは
「トレイトを実装している何かの型」としか見えません。「トレイトを実装している値なら何でもよい」という意味では
ないので、どの分岐も同じ型を返す必要があります。

- `select` では、クロージャはそれぞれ固有の無名の型を持ちます。見た目が同じでも同じ型にはなりません。
  そのため `Filter<Copied<Iter<i32>>, {closure#1}>` と `Filter<Copied<Iter<i32>>, {closure#2}>` は
  別の型で、`Copied<Iter<i32>>` はさらに別の型です。
- `describe` では、`String` と `i32` はどちらも `Display` を実装していますが、やはり 2 つの別の型です。
- `op_for` では、4 つのクロージャが 4 つの型になります。どれも `scale` をキャプチャしているので、
  関数ポインタ型 `fn(i32, i32) -> i32` に型強制することもできません。

模範解答は、分岐を 1 つの値の**中に**移しています。`select` は `keep` で `match` するクロージャを持つ
`filter` を 1 つだけ作り、`describe` はどちらの分岐でも `String` を作り、`op_for` は呼ばれるたびに
`symbol` で `match` するクロージャを 1 つ返します。これらのクロージャには `move` が必要です。
`keep`、`symbol`、`scale` は引数なので、借用するだけのクロージャは関数の呼び出しより長く生きられません
（E0373、"closure may outlive the current function"）。どれも `Copy` なので、`move` で単にコピーされます。

型エラーの陰に、ロジックのバグが 2 つ隠れています。型だけを直しても（たとえば `else` の分岐に
コンパイラが提案する `n.to_string()` を使っても）、これらは残ります。

- `%` は剰余で、結果の符号は左側のオペランドに従います。`-3 % 2` は `-1` です。そのため `x % 2 == 1` では
  負の奇数を取りこぼします。代わりに `x % 2 != 0`（または `x.rem_euclid(2) == 1`）で判定します。
- `-n` は `i32::MIN` でオーバーフローします。`2147483648` は `i32` に収まらないからです。
  ジャッジのようにオーバーフロー検査が有効だとパニックになります。`n.abs()` も同じです。
  `n.unsigned_abs()` は絶対値を `u32` で返すので、必ず収まります。

### 別解

分岐はそのままにして、型を消す方法もあります。`Box<dyn Iterator<Item = i32> + '_>` は、中にどんな
イテレータが入っていても 1 つの型で、それ自身も `Iterator` を実装しているので、`impl Iterator` として
返せます。各アームをトレイトオブジェクトに変えるのは `boxed` の型注釈です。これがないと、
それぞれの `Box::new(...)` が固有の具体的な型のままになり、アームの型は食い違ったままです。
`+ '_` は、イテレータが `v` を借用していることを表します。`op_for` は返り値の型を変えてよいので、
`Box<dyn Fn(i32, i32) -> i32>` を直接返せます。その場合は返り値の型が各アームを変換します。

```rust
pub fn select(v: &[i32], keep: Keep) -> impl Iterator<Item = i32> + '_ {
    let iter = v.iter().copied();
    let boxed: Box<dyn Iterator<Item = i32> + '_> = match keep {
        Keep::Evens => Box::new(iter.filter(|x| x % 2 == 0)),
        Keep::Odds => Box::new(iter.filter(|x| x % 2 != 0)),
        Keep::All => Box::new(iter),
    };
    boxed
}

pub fn op_for(symbol: char, scale: i32) -> Box<dyn Fn(i32, i32) -> i32> {
    match symbol {
        '+' => Box::new(move |a, b| (a + b) * scale),
        '-' => Box::new(move |a, b| (a - b) * scale),
        '*' => Box::new(move |a, b| a * b * scale),
        _ => Box::new(move |a, _| a * scale),
    }
}
```

ボックス化すると、ヒープ確保と、要素ごと・呼び出しごとの間接呼び出しのコストがかかります。
模範解答ではこれを避けています。

何もキャプチャしないクロージャなら、関数ポインタに型強制できます。そこで `op_for` は、まず演算を
`fn(i32, i32) -> i32` として選び、`scale` を掛けるのは 1 つのキャプチャするクロージャに任せることもできます。

```rust
pub fn op_for(symbol: char, scale: i32) -> impl Fn(i32, i32) -> i32 {
    let base: fn(i32, i32) -> i32 = match symbol {
        '+' => |a, b| a + b,
        '-' => |a, b| a - b,
        '*' => |a, b| a * b,
        _ => |a, _| a,
    };
    move |a, b| base(a, b) * scale
}
```
