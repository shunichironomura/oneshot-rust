Rust Book 21 章のシングルスレッド Web サーバーから、ネットワークを除いたリクエスト解析の部分です。
リクエストはバイト列で届きます。構造体 `Request` が用意されています（エディタに入っています）。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn parse_request(raw: &[u8]) -> Option<Request>;
```

`parse_request` は HTTP リクエストを読み取ります。各行は `\r\n` で終わり、`\r\n` は行に含めません。
最後の行は、`\r\n` がなく `raw` の終わりで終わっていてもかまいません。ボディより前では、
`\r` と `\n` は `\r\n` としてしか `raw` に現れないと仮定してかまいません。

- 最初の行はリクエスト行 `METHOD PATH VERSION` です。ちょうど 3 つの部分が空白で区切られて
  いなければなりません。`VERSION` はちょうど `HTTP/1.1` でなければなりません。`METHOD` と `PATH` は
  それ以上チェックしません。
- その後の行のうち、最初の空行か `raw` の終わりまでがヘッダで、`Name: value` の形をしています。
  各行を**最初の** `": "`（コロンと空白）で名前と値に分け、どちらもそのまま（前後の空白も取り除かずに）
  使います。`headers` には出てきた順に並べます。
- その空行より後はすべてボディです。`parse_request` は、ボディがどんなバイト列でも無視します。
- `raw` にリクエスト行がないとき、リクエスト行が上の規則に合わないとき、ヘッダの行に `": "` が
  ないとき、リクエスト行かヘッダの行が正しい UTF-8 でないときは `None` を返します。

### 例

```rust
parse_request(b"GET / HTTP/1.1\r\nHost: localhost:7878\r\nAccept: */*\r\n\r\n")
// Some(Request { method: "GET", path: "/", headers: [("Host", "localhost:7878"), ("Accept", "*/*")] })

parse_request(b"GET / HTTP/1.0\r\n\r\n")
// None（バージョンが HTTP/1.1 ではない）
```
