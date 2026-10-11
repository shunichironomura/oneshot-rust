`?` はエラーを返す前に `From::from` で変換します。core には反射的な impl
`impl<T> From<T> for T` があり、`ok_or(AppError::MissingComma)?` が `AppError` をそのまま返せるのは
この impl のおかげです。テンプレートはライブラリのエラー向けに
`impl<E: Error> From<E> for AppError` を追加していますが、この impl は `Error` を実装する
**すべての**型が対象で、`AppError` 自身もその 1 つです。
`E = AppError` のとき、2 つの impl がどちらも `From<AppError> for AppError` を与えることになります。
Rust は impl の重なりを決して認めない（stable には特殊化がない）ので、この変換を実際に使うコードが
なくても、汎用の impl そのものが E0119（conflicting implementations）で拒否されます。

`impl Error for AppError` を消せば重なりはなくなりますが、問題文で禁止されているだけでなく、
行き止まりでもあります。`sum_pairs` の `?` は std の `impl<E: Error> From<E> for Box<dyn Error>` を
通してエラーをボックス化しますし、`downcast_ref::<AppError>()` も `AppError: Error` を必要とします。
`Error` を実装しつつ、あらゆる `Error` から変換できる型は作れないのです。

そこで、実際に出てくるエラー型を名指しします。`parse` は `i32` なら `ParseIntError` を、`f64` なら
`ParseFloatError` を返します。それぞれに `From` を実装すれば、ほかのコードはそのままで動きます。

`Box<dyn Error>` 自身が `Error` を実装していないのも、あらゆるエラー型から変換できる
`anyhow::Error` が `Error` を実装していないのも、同じ衝突が理由です。

### 別解

`From` を実装せず、呼び出し側で `map_err` を使って変換する方法です。この場合は
ターボフィッシュが必要です。`::<i32>` がないと、コンパイラは `map_err` の呼び出しを検査する中で
クロージャの本体 `e.to_string()` を検査しますが、その時点では `parse` の変換先の型がまだ
決まっていません。`e` の型がわからないので、type annotations needed（E0282）で止まります。
`let a: i32 = ...?;` のように変数に型を書いても解決しません。その型が `parse` に届くのは、
後で `?` を通してだからです。

```rust
pub fn parse_pair(s: &str) -> Result<(i32, f64), AppError> {
    let (a, b) = s.split_once(',').ok_or(AppError::MissingComma)?;
    let a = a.trim().parse::<i32>().map_err(|e| AppError::Parse(e.to_string()))?;
    let b = b.trim().parse::<f64>().map_err(|e| AppError::Parse(e.to_string()))?;
    Ok((a, b))
}
```

テンプレートの「どんなエラー型でも」という考えを残したいなら、`From` の impl ではなく
ジェネリックな関数にします。コヒーレンス（impl が重ならないことの検査）は impl だけが対象なので、
関数なら問題ありません。

```rust
use std::str::FromStr;

fn parse_part<T>(s: &str) -> Result<T, AppError>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    s.trim().parse::<T>().map_err(|e| AppError::Parse(e.to_string()))
}

pub fn parse_pair(s: &str) -> Result<(i32, f64), AppError> {
    let (a, b) = s.split_once(',').ok_or(AppError::MissingComma)?;
    Ok((parse_part(a)?, parse_part(b)?))
}
```

最後に、`AppError` が満たさない境界を加えても重なりはなくなります。
`impl<E: Error + Clone> From<E> for AppError` はコンパイルできます。`AppError` は `Clone` ではなく、
`Clone` にできるのはこのクレートだけだからです（2 つの解析エラーはどちらも `Clone` です）。
ただし壊れやすい方法です。後で `AppError` に `Clone` を derive すると E0119 が戻ってきますし、
`Clone` でないエラーは変換できなくなります。
