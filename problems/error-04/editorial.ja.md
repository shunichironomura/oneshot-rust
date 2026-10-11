`Error::source` のシグネチャは `fn source(&self) -> Option<&(dyn Error + 'static)>` です。この `'static` は
参照についてのものではありません（参照が生きるのは `&self` と同じ間だけです）。原因のエラーの**型**が
借用したデータを持たないことを表しています。ダウンキャストできるのはそういう型だけです。
`downcast_ref` は `TypeId` を比べて型を確かめますが、`TypeId` は `'static` な型にしかないので、
`dyn Error + 'static` にしか定義されていません。つい書きたくなる `Option<&dyn Error>` はコンパイルが通りません。
この位置では省略されたオブジェクトのライフタイムが `&self` のライフタイムになり、トレイトより弱い約束に
なってしまうからです。コンパイラはわかりにくい E0803（"cannot infer an appropriate lifetime"）を報告します。
メソッドの中では、`match self` で `source` が `&ParseIntError` として束縛され、`Some(source)` で
`&(dyn Error + 'static)` に型強制されます。

`BadPort` の表示は自分のメッセージだけです。原因は `source()` からたどれるので、`report` のように
連鎖全体を表示するコードでは、メッセージに含めると 2 回表示されてしまいます。原因を `source()` で返すか、
メッセージに含めるかのどちらか一方にするのが、標準ライブラリの慣習です。

`load` の `?` は `From::from` でエラーを変換します。std には `impl<E: Error> From<E> for Box<dyn Error>`
があるので、`ConfigError`（`Debug` と `Display` と `Error` の実装がそろって初めて対象になります）も
`ParseIntError` も、そのままの形でボックスに入ります。ボックスは型を隠しますが、値は保たれます。
`downcast_ref` で取り出せるので、テストは `ConfigError` と素の `ParseIntError` を区別できます。
タイムアウトのエラーを `map_err(|e| e.to_string())?` でメッセージに変えると、文字列は残っても型は失われます。

ヘルパーの `get` には名前付きのライフタイムが必要です。引数に省略されたライフタイムがいくつもあるので、
戻り値がどれを借用しているのかコンパイラにはわかりません（E0106）。戻り値が借用しているのは値
（`&'a str`）で、`pairs` や `key` ではありません。また、`iter().find` はクロージャに
`&&(&str, &str)` を渡すので、パターン `&&(k, _)` でキーをコピーして取り出しています。
`|(k, _)|` だと `k` は `&&str` になり、`k == key` はコンパイルが通りません（`&str` と `str` は比べられません）。

`root_cause` でも `'a` は省略できません。引数の型に `'a` と `'static` の 2 つのライフタイムがあるからです。
ループでは `cur` を連鎖に沿って進めます。`cur.source()` が返す参照は `cur` と同じだけ生きるので、
`cur = next` で問題ありません。`report` は `'static` なしの `&dyn Error` を受け取ります。
連鎖をたどって表示するだけなら、ダウンキャストは要らないからです。

### 別解

`std::iter::successors` を使うと、連鎖をイテレータにできます。

```rust
pub fn root_cause<'a>(e: &'a (dyn Error + 'static)) -> &'a (dyn Error + 'static) {
    std::iter::successors(Some(e), |&e| e.source()).last().unwrap()
}

pub fn report(e: &dyn Error) -> String {
    std::iter::successors(Some(e), |&e| e.source())
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(": ")
}
```

`|&e|` の `&` が重要です。クロージャが受け取るのは `&&dyn Error` で、std は参照にも `Error` を実装しています
（`impl<T: Error + ?Sized> Error for &T`）。`|e| e.source()` と書くと、メソッドの探索で外側の参照に対する
この実装が選ばれ、戻り値はクロージャの引数を借用することになるので、返せません
（"lifetime may not live long enough"）。`|&e|` なら、先に内側の `&dyn Error` をコピーして取り出せます。

再帰でも連鎖をたどれます。

```rust
pub fn root_cause<'a>(e: &'a (dyn Error + 'static)) -> &'a (dyn Error + 'static) {
    match e.source() {
        Some(cause) => root_cause(cause),
        None => e,
    }
}

pub fn report(e: &dyn Error) -> String {
    match e.source() {
        Some(cause) => format!("{e}: {}", report(cause)),
        None => e.to_string(),
    }
}
```

`load` は、1 回のループで各キーの最初の値を拾う書き方もできます。また、`Result` の別名を作ると
シグネチャが短くなります。別名には下のようにエラー型の既定値を付けてください。
`type Result<T> = std::result::Result<T, Box<dyn Error>>` だけだと、`parse_port` の
`Result<u16, ConfigError>` がコンパイルできなくなります（E0107）。`transpose` はタイムアウトの
`Option<Result<u64, _>>` を `Result<Option<u64>, _>` に変えて、`?` を使えるようにします。

```rust
type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub fn parse_port(s: &str) -> Result<u16, ConfigError> {
    let n: u32 = s.parse().map_err(|source| ConfigError::BadPort {
        value: s.to_string(),
        source,
    })?;
    u16::try_from(n)
        .ok()
        .filter(|&port| port != 0)
        .ok_or(ConfigError::OutOfRange(n))
}

pub fn load(pairs: &[(&str, &str)]) -> Result<(String, u16, u64)> {
    let (mut host, mut port, mut timeout) = (None, None, None);
    for &(key, value) in pairs {
        let slot = match key {
            "host" => &mut host,
            "port" => &mut port,
            "timeout" => &mut timeout,
            _ => continue,
        };
        slot.get_or_insert(value);
    }
    let host = host.ok_or(ConfigError::Missing("host"))?;
    let port = parse_port(port.ok_or(ConfigError::Missing("port"))?)?;
    let timeout = timeout.map(str::parse::<u64>).transpose()?.unwrap_or(30);
    Ok((host.to_string(), port, timeout))
}
```
