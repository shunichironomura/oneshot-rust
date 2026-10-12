Rust Book 21 章のシングルスレッド Web サーバーから、ネットワークを除いたレスポンスの部分です。
レスポンスは、`TcpStream` や `Vec<u8>` のような任意の `io::Write` にバイト列として書き込みます。
構造体 `Request` が用意されています（エディタに入っています）。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn respond(req: &Request, out: &mut impl Write) -> io::Result<()>;
```

`respond` はメソッドとパスだけを見てレスポンスを作ります（ヘッダは関係ありません）。
メソッドもパスも完全一致で比べるので、大文字と小文字も区別します。

| メソッドとパス | ステータス | ボディ |
| --- | --- | --- |
| `GET /` | `200 OK` | `<h1>Hello!</h1>` |
| `GET /greet` | `200 OK` | `こんにちは` |
| それ以外 | `404 NOT FOUND` | `Oops!` |

- レスポンスは UTF-8 のテキスト `HTTP/1.1 {status}\r\nContent-Length: {n}\r\n\r\n{body}` です。
  `{status}` と `{body}` は表のとおりで、`{n}` はボディのバイト数です。
- レスポンスを `out` に書き込みます。`out` への書き込みがエラーを報告したら、そのエラーを返します。

### 例

```rust
let mut out = Vec::new();
respond(&Request { method: "GET".into(), path: "/".into(), headers: vec![] }, &mut out)
// Ok(())。out は b"HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n<h1>Hello!</h1>"

let mut out = Vec::new();
respond(&Request { method: "POST".into(), path: "/".into(), headers: vec![] }, &mut out)
// Ok(())。out は b"HTTP/1.1 404 NOT FOUND\r\nContent-Length: 5\r\n\r\nOops!"
```
