use std::io::{BufRead, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn parse_request(raw: &[u8]) -> Option<Request> {
    // `&[u8]` implements `BufRead`, so it can be read line by line like a socket.
    let mut lines = raw.lines();

    let request_line = lines.next()?.ok()?;
    let parts: Vec<&str> = request_line.split(' ').collect();
    let [method, path, version] = parts[..] else {
        return None;
    };
    if method.is_empty() || path.is_empty() || version != "HTTP/1.1" {
        return None;
    }

    let mut headers = Vec::new();
    for line in lines {
        let line = line.ok()?; // invalid UTF-8
        if line.is_empty() {
            break; // the body follows and is never read
        }
        let (name, value) = line.split_once(": ")?;
        headers.push((name.to_string(), value.to_string()));
    }

    Some(Request { method: method.to_string(), path: path.to_string(), headers })
}

pub fn respond(req: &Request) -> Vec<u8> {
    let (status, body) = match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => ("200 OK", "<h1>Hello!</h1>"),
        ("GET", "/greet") => ("200 OK", "こんにちは"),
        _ => ("404 NOT FOUND", "Oops!"),
    };
    let mut response = Vec::new();
    // `len` counts bytes, which is what Content-Length needs.
    write!(response, "HTTP/1.1 {status}\r\nContent-Length: {}\r\n\r\n{body}", body.len())
        .expect("writing to a Vec<u8> cannot fail");
    response
}
