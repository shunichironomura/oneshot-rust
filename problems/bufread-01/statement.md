This is the request handling of the single-threaded web server from chapter 21 of the Rust Book,
without the network: the request arrives as raw bytes, and the response goes back as bytes. The
`Request` struct is already in the editor.

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

`parse_request` reads an HTTP request. Its lines end with `\r\n`, which is not part of the line.
The last line may also end at the end of `raw` without `\r\n`.

- The first line is the request line `METHOD PATH VERSION`: exactly three non-empty parts separated
  by single spaces (`' '`). `VERSION` must be exactly `HTTP/1.1`. `METHOD` and `PATH` are not
  checked further.
- The lines after it, up to the first empty line or the end of `raw`, are headers of the form
  `Name: value`. Each one is split at its **first** `": "` (a colon and a space) into the name and
  the value, which are kept exactly as they are (no trimming). `headers` lists them in the order
  they appear.
- Everything after that empty line is the body. `parse_request` ignores it, whatever bytes it
  contains.
- Return `None` if `raw` has no request line, the request line breaks the rules above, a header line
  has no `": "`, or the request line or a header line is not valid UTF-8.

`respond` builds a response from the method and the path only; the headers do not matter. Both are
compared exactly, so case matters.

| Method and path | Status | Body |
| --- | --- | --- |
| `GET /` | `200 OK` | `<h1>Hello!</h1>` |
| `GET /greet` | `200 OK` | `こんにちは` |
| anything else | `404 NOT FOUND` | `Oops!` |

The response is the UTF-8 text `HTTP/1.1 {status}\r\nContent-Length: {n}\r\n\r\n{body}`, where
`{status}` and `{body}` come from the table and `{n}` is the length of the body in bytes.

### Examples

```rust
parse_request(b"GET / HTTP/1.1\r\nHost: localhost:7878\r\nAccept: */*\r\n\r\n")
// Some(Request { method: "GET", path: "/", headers: [("Host", "localhost:7878"), ("Accept", "*/*")] })

parse_request(b"GET / HTTP/1.0\r\n\r\n")
// None (the version is not HTTP/1.1)

respond(&Request { method: "GET".into(), path: "/".into(), headers: vec![] })
// b"HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n<h1>Hello!</h1>"

respond(&Request { method: "POST".into(), path: "/".into(), headers: vec![] })
// b"HTTP/1.1 404 NOT FOUND\r\nContent-Length: 5\r\n\r\nOops!"
```
