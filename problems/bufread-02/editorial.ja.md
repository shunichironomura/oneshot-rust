`str::len` は**バイト数**を返し、これが `Content-Length` に必要な値です。`"こんにちは"` は 5 文字
（`chars().count()`）ですが 15 バイトです。

`write!` はどんな `io::Write` にも使えるので、同じ `respond` で `TcpStream` にも、テストでは
`Vec<u8>` にも書き込めます。そのためには `Write` トレイトがスコープに入っている必要があり、先頭の
`use` がそれを担っています。`write!` は `io::Result<()>` を返し、これは `respond` の返り値の型と
同じなので、そのまま最後の式にできます。代わりに `unwrap` や `expect` を呼ぶと、クライアントが
接続を閉じたときなど、書き込みが失敗したときにパニックします。

タプル `(req.method.as_str(), req.path.as_str())` で `match` すると、2 つの文字列を一度に比べられます。
`String` は文字列リテラルと直接パターンマッチできないので、`as_str()` が必要です。

### 別解

`format!` でレスポンス全体を `String` として作ってから、`write_all` でそのバイト列を書き込む方法です。
書き込む前にレスポンス全体がメモリ上に作られます。

```rust
pub fn respond(req: &Request, out: &mut impl Write) -> io::Result<()> {
    let (status, body) = match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => ("200 OK", "<h1>Hello!</h1>"),
        ("GET", "/greet") => ("200 OK", "こんにちは"),
        _ => ("404 NOT FOUND", "Oops!"),
    };
    let response = format!("HTTP/1.1 {status}\r\nContent-Length: {}\r\n\r\n{body}", body.len());
    out.write_all(response.as_bytes())
}
```
