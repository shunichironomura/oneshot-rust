This is the response side of the single-threaded web server from chapter 21 of the Rust Book,
without the network: the response is written as bytes to any `io::Write`, such as a `TcpStream` or
a `Vec<u8>`. The `Request` struct is already in the editor.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn respond(req: &Request, out: &mut impl Write) -> io::Result<()>;
```

`respond` builds a response from the method and the path only; the headers do not matter. Both are
compared exactly, so case matters.

| Method and path | Status | Body |
| --- | --- | --- |
| `GET /` | `200 OK` | `<h1>Hello!</h1>` |
| `GET /greet` | `200 OK` | `こんにちは` |
| anything else | `404 NOT FOUND` | `Oops!` |

- The response is the UTF-8 text `HTTP/1.1 {status}\r\nContent-Length: {n}\r\n\r\n{body}`, where
  `{status}` and `{body}` come from the table and `{n}` is the length of the body in bytes.
- Write the response to `out`, and return any error that writing to `out` reports.

### Examples

```rust
let mut out = Vec::new();
respond(&Request { method: "GET".into(), path: "/".into(), headers: vec![] }, &mut out)
// Ok(()), and out is b"HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n<h1>Hello!</h1>"

let mut out = Vec::new();
respond(&Request { method: "POST".into(), path: "/".into(), headers: vec![] }, &mut out)
// Ok(()), and out is b"HTTP/1.1 404 NOT FOUND\r\nContent-Length: 5\r\n\r\nOops!"
```
