This is the request parsing of the single-threaded web server from chapter 21 of the Rust Book,
without the network: the request arrives as raw bytes. The `Request` struct is already in the
editor.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn parse_request(raw: &[u8]) -> Option<Request>;
```

`parse_request` reads an HTTP request. Its lines end with `\r\n`, which is not part of the line.
The last line may also end at the end of `raw` without `\r\n`. You may assume that before the body,
`\r` and `\n` appear in `raw` only together as `\r\n`.

- The first line is the request line `METHOD PATH VERSION`: exactly three parts separated by
  spaces. `VERSION` must be exactly `HTTP/1.1`. `METHOD` and `PATH` are not checked further.
- The lines after it, up to the first empty line or the end of `raw`, are headers of the form
  `Name: value`. Each one is split at its **first** `": "` (a colon and a space) into the name and
  the value, which are kept exactly as they are (no trimming). `headers` lists them in the order
  they appear.
- Everything after that empty line is the body. `parse_request` ignores it, whatever bytes it
  contains.
- Return `None` if `raw` has no request line, the request line breaks the rules above, a header line
  has no `": "`, or the request line or a header line is not valid UTF-8.

### Examples

```rust
parse_request(b"GET / HTTP/1.1\r\nHost: localhost:7878\r\nAccept: */*\r\n\r\n")
// Some(Request { method: "GET", path: "/", headers: [("Host", "localhost:7878"), ("Accept", "*/*")] })

parse_request(b"GET / HTTP/1.0\r\n\r\n")
// None (the version is not HTTP/1.1)
```
