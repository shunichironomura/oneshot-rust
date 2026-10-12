use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn respond(req: &Request, out: &mut impl Write) -> io::Result<()> {
    let (status, body) = match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => ("200 OK", "<h1>Hello!</h1>"),
        ("GET", "/greet") => ("200 OK", "こんにちは"),
        _ => ("404 NOT FOUND", "Oops!"),
    };
    // `len` counts bytes, which is what Content-Length needs.
    write!(out, "HTTP/1.1 {status}\r\nContent-Length: {}\r\n\r\n{body}", body.len())
}
