Rust Book 21 章のシングルスレッド Web サーバーから、ネットワークを除いたリクエスト処理の部分です。
リクエストはバイト列で届き、レスポンスもバイト列で返します。構造体 `Request` が用意されています
（エディタに入っています）。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn parse_request(raw: &[u8]) -> Option<Request>;
pub fn respond(req: &Request) -> Vec<u8>;
```

`parse_request` は HTTP リクエストを読み取ります。各行は `\r\n` で終わり、`\r\n` は行に含めません。
最後の行は、`\r\n` がなく `raw` の終わりで終わっていてもかまいません。ボディより前では、`raw` の
`\n` はすべて `\r\n` の一部だと仮定してかまいません。

- 最初の行はリクエスト行 `METHOD PATH VERSION` です。ちょうど 3 つの空でない部分が、
  1 つずつの空白（`' '`）で区切られていなければなりません。`VERSION` はちょうど `HTTP/1.1` で
  なければなりません。`METHOD` と `PATH` はそれ以上チェックしません。
- その後の行のうち、最初の空行か `raw` の終わりまでがヘッダーで、`Name: value` の形をしています。
  各行を**最初の** `": "`（コロンと空白）で名前と値に分け、どちらもそのまま（前後の空白も取り除かずに）
  使います。`headers` には出てきた順に並べます。
- その空行より後はすべてボディです。`parse_request` は、ボディがどんなバイト列でも無視します。
- `raw` にリクエスト行がないとき、リクエスト行が上の規則に合わないとき、ヘッダーの行に `": "` が
  ないとき、リクエスト行かヘッダーの行が正しい UTF-8 でないときは `None` を返します。

`respond` はメソッドとパスだけを見てレスポンスを作ります（ヘッダーは関係ありません）。
メソッドもパスも完全一致で比べるので、大文字と小文字も区別します。

| メソッドとパス | ステータス | ボディ |
| --- | --- | --- |
| `GET /` | `200 OK` | `<h1>Hello!</h1>` |
| `GET /greet` | `200 OK` | `こんにちは` |
| それ以外 | `404 NOT FOUND` | `Oops!` |

レスポンスは UTF-8 のテキスト `HTTP/1.1 {status}\r\nContent-Length: {n}\r\n\r\n{body}` です。
`{status}` と `{body}` は表のとおりで、`{n}` はボディのバイト数です。

### 例

```rust
parse_request(b"GET / HTTP/1.1\r\nHost: localhost:7878\r\nAccept: */*\r\n\r\n")
// Some(Request { method: "GET", path: "/", headers: [("Host", "localhost:7878"), ("Accept", "*/*")] })

parse_request(b"GET / HTTP/1.0\r\n\r\n")
// None（バージョンが HTTP/1.1 ではない）

respond(&Request { method: "GET".into(), path: "/".into(), headers: vec![] })
// b"HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n<h1>Hello!</h1>"

respond(&Request { method: "POST".into(), path: "/".into(), headers: vec![] })
// b"HTTP/1.1 404 NOT FOUND\r\nContent-Length: 5\r\n\r\nOops!"
```
