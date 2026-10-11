`&[u8]` implements `std::io::Read` (reading takes bytes off the front of the slice) and also
`BufRead`, so a byte slice can be read line by line just like the `TcpStream` in the Book. `lines`
is a method of the `BufRead` trait, so the call compiles only with `use std::io::BufRead;` in scope.
`BufReader::new(raw)`, as in the Book, works too, but it only adds a second buffer.

Each item of `lines()` is an `io::Result<String>`, not a `String`: reading a line fails when it is
not valid UTF-8. The Book unwraps it, which would panic on such a request; `.ok()?` turns the error
into `None` as required. `flatten()` (skip the errors) and `map_while(Result::ok)` (stop at the first
error) compile just as easily, but they can return a request for invalid input. `lines()` also
removes the line ending (it splits at `\n` and drops a `\r` just before it), so the version can be
compared with `"HTTP/1.1"` directly.

The loop stops at the first empty line, so the body is never read, let alone decoded. Converting
all of `raw` with `str::from_utf8` first would reject a request whose body is binary.

`split(' ')` keeps empty parts, so a double space produces an extra, empty part, while
`split_whitespace` would accept `"GET  / HTTP/1.1"`. The slice pattern
`let [method, path, version] = parts[..] else { ... }` checks the count and names the parts in one
step. `split_once(": ")` splits at the first separator and returns `None` when there is none, and
`?` passes that on.

In `respond`, `str::len` is the length in **bytes**, which is what `Content-Length` counts:
`"こんにちは"` has 5 characters (`chars().count()`) but 15 bytes. `Vec<u8>` implements `io::Write`,
so `write!` can format straight into it once `std::io::Write` is in scope. It returns an
`io::Result`, which must be used or the compiler warns; writing to a `Vec` never fails, hence the
`expect`.

### Alternatives

The Book's style collects the lines up to the empty line first. `take_while` stops at the first
`Ok("")` but lets errors through, and collecting into `Result<Vec<_>, _>` turns the first error into
`Err`. Collecting into `Option<_>` does the same for the headers.

```rust
use std::io::{BufRead, BufReader};

pub fn parse_request(raw: &[u8]) -> Option<Request> {
    let lines: Vec<String> = BufReader::new(raw)
        .lines()
        .take_while(|line| !matches!(line, Ok(l) if l.is_empty()))
        .collect::<Result<_, _>>()
        .ok()?;
    let (request_line, header_lines) = lines.split_first()?;

    let parts: Vec<&str> = request_line.split(' ').collect();
    let [method, path, version] = parts[..] else {
        return None;
    };
    if method.is_empty() || path.is_empty() || version != "HTTP/1.1" {
        return None;
    }

    let headers = header_lines
        .iter()
        .map(|line| {
            let (name, value) = line.split_once(": ")?;
            Some((name.to_string(), value.to_string()))
        })
        .collect::<Option<_>>()?;

    Some(Request { method: method.to_string(), path: path.to_string(), headers })
}
```

`read_line` reads one line into a `String` that you pass in. It keeps the line ending and returns
`Ok(0)` at the end of the input. The reader is `raw` itself, so `raw` must be `mut`: reading
advances the slice.

```rust
use std::io::BufRead;

pub fn parse_request(mut raw: &[u8]) -> Option<Request> {
    let mut line = String::new();
    raw.read_line(&mut line).ok()?;
    let parts: Vec<&str> = line.strip_suffix("\r\n").unwrap_or(&line).split(' ').collect();
    let [method, path, version] = parts[..] else {
        return None;
    };
    if method.is_empty() || path.is_empty() || version != "HTTP/1.1" {
        return None;
    }
    let (method, path) = (method.to_string(), path.to_string());

    let mut headers = Vec::new();
    loop {
        line.clear();
        if raw.read_line(&mut line).ok()? == 0 {
            break;
        }
        let text = line.strip_suffix("\r\n").unwrap_or(&line);
        if text.is_empty() {
            break;
        }
        let (name, value) = text.split_once(": ")?;
        headers.push((name.to_string(), value.to_string()));
    }

    Some(Request { method, path, headers })
}
```

Parsing also works without `std::io`: split the bytes at `b'\n'` and decode each line on its own.
Each item is then an `Option<&str>` (`None` for invalid UTF-8), hence the `??` on the first one.
Iterators are lazy, so nothing after the empty line, including the body, is ever decoded.

```rust
pub fn parse_request(raw: &[u8]) -> Option<Request> {
    let mut lines = raw.split(|&b| b == b'\n').map(|line| {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        std::str::from_utf8(line).ok()
    });

    let request_line = lines.next()??;
    let parts: Vec<&str> = request_line.split(' ').collect();
    let [method, path, version] = parts[..] else {
        return None;
    };
    if method.is_empty() || path.is_empty() || version != "HTTP/1.1" {
        return None;
    }

    let mut headers = Vec::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (name, value) = line.split_once(": ")?;
        headers.push((name.to_string(), value.to_string()));
    }

    Some(Request { method: method.to_string(), path: path.to_string(), headers })
}
```

`format!` builds a `String`, and `into_bytes` turns it into a `Vec<u8>` without copying. This needs
no `Write` import.

```rust
pub fn respond(req: &Request) -> Vec<u8> {
    let (status, body) = match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => ("200 OK", "<h1>Hello!</h1>"),
        ("GET", "/greet") => ("200 OK", "こんにちは"),
        _ => ("404 NOT FOUND", "Oops!"),
    };
    format!("HTTP/1.1 {status}\r\nContent-Length: {}\r\n\r\n{body}", body.len()).into_bytes()
}
```
