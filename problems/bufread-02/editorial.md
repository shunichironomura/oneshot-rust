`str::len` is the length in **bytes**, which is what `Content-Length` counts: `"こんにちは"` has 5
characters (`chars().count()`) but 15 bytes.

`write!` works with any `io::Write`, so the same `respond` can write to a `TcpStream` or, in the
tests, to a `Vec<u8>`. It needs the `Write` trait in scope, which the `use` line at the top
provides. `write!` returns an `io::Result<()>`, which is exactly what `respond` returns, so it can
be the final expression. Calling `unwrap` or `expect` on it instead would panic when the writer
fails, for example when the client has closed the connection.

Matching on the tuple `(req.method.as_str(), req.path.as_str())` compares both strings at once.
`as_str()` is needed because a `String` cannot be matched against string literals directly.

### Alternatives

`format!` builds the whole response as a `String` first, and `write_all` sends its bytes. The
response is then built in memory before anything is written.

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
