`Closed` と `Open` はデータを持たず、型引数として使うためだけに存在します。構造体の型引数はどこかのフィールドで
使わなければならないので、`PhantomData<State>` で、何も保持せずに `State` を使ったことにします。
`PhantomData` のサイズは 0 なので、`Connection<Open>` と `Connection<Closed>` のメモリ上の配置は同じです。

`open` と `close` は `self` を**値で**受け取ります。ホストとログを型引数の違う新しい構造体にムーブし、元の値は
なくなるので、`open` したあとに閉じた接続を使うことはできません。`send` は `Connection<Open>` にしかないので、
閉じた接続での送信は実行時のチェックではなくコンパイルエラーになります。2 つの遷移は、任意の状態の接続を作れる
非公開のジェネリックな補助関数 `transition<Next>` を共有しています。

構造体更新構文（`Connection { _state: PhantomData, ..self }`）はここでは使えません。元の値と結果が同じ型である
必要があり、`Connection<Closed>` と `Connection<Open>` は別の型だからです。

### 別解

`Closed` と `Open` はそれ自体サイズが 0 なので、`PhantomData` の代わりに状態をそのまま持たせることもできます。
この場合もメモリは増えません。

```rust
pub struct Connection<State> {
    host: String,
    log: Vec<String>,
    state: State,
}

impl Connection<Closed> {
    pub fn open(mut self) -> Connection<Open> {
        self.log.push("open".to_string());
        Connection {
            host: self.host,
            log: self.log,
            state: Open,
        }
    }
}
```
