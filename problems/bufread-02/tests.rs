use std::io;

use crate::judge::{case, check, check_eq};
use crate::user::{Request, respond};

fn req(method: &str, path: &str, headers: &[(&str, &str)]) -> Request {
    Request {
        method: method.to_string(),
        path: path.to_string(),
        headers: headers.iter().map(|&(n, v)| (n.to_string(), v.to_string())).collect(),
    }
}

/// Writes the response into a `Vec<u8>` and compares the bytes, showing them as text when they
/// differ.
fn check_response(req: &Request, expected: &str) -> Result<(), String> {
    let mut out = Vec::new();
    respond(req, &mut out).map_err(|e| format!("respond returned Err({e})"))?;
    if out == expected.as_bytes() {
        Ok(())
    } else {
        check_eq(String::from_utf8_lossy(&out), expected)
    }
}

/// A writer that always fails.
struct Broken;

impl io::Write for Broken {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("broken pipe"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn run() {
    case("sample-1", || {
        check_response(
            &req("GET", "/", &[]),
            "HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n<h1>Hello!</h1>",
        )?;
        check_response(
            &req("POST", "/", &[]),
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 5\r\n\r\nOops!",
        )
    });
    case("hidden-1", || {
        // Content-Length counts bytes: "こんにちは" is 5 characters but 15 bytes.
        check_response(
            &req("GET", "/greet", &[("Accept", "*/*")]),
            "HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\nこんにちは",
        )
    });
    case("hidden-2", || {
        // Everything else is 404, compared exactly.
        let not_found = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 5\r\n\r\nOops!";
        check_response(&req("GET", "/greet/", &[]), not_found)?;
        check_response(&req("get", "/", &[]), not_found)?;
        check_response(&req("HEAD", "/greet", &[]), not_found)
    });
    case("hidden-3", || {
        // An error from the writer is returned, not unwrapped.
        check(respond(&req("GET", "/", &[]), &mut Broken).is_err(), "expected Err from a broken writer")
    });
}
