use crate::judge::{case, check_eq};
use crate::user::{Request, parse_request, respond};

fn req(method: &str, path: &str, headers: &[(&str, &str)]) -> Request {
    Request {
        method: method.to_string(),
        path: path.to_string(),
        headers: headers.iter().map(|&(n, v)| (n.to_string(), v.to_string())).collect(),
    }
}

/// Compares without allocating (the interpreter is slow) and builds the expected `Request` only to
/// report a mismatch.
fn check_req(
    got: Option<Request>,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
) -> Result<(), String> {
    let same = got.as_ref().is_some_and(|r| {
        r.method == method
            && r.path == path
            && r.headers.len() == headers.len()
            && r.headers.iter().zip(headers).all(|((n, v), (en, ev))| n == en && v == ev)
    });
    if same { Ok(()) } else { check_eq(got, Some(req(method, path, headers))) }
}

/// Compares the bytes, and shows the response as text when they differ.
fn check_response(got: Vec<u8>, expected: &str) -> Result<(), String> {
    if got == expected.as_bytes() {
        Ok(())
    } else {
        check_eq(String::from_utf8_lossy(&got), expected)
    }
}

pub fn run() {
    case("sample-1", || {
        check_req(
            parse_request(b"GET / HTTP/1.1\r\nHost: localhost:7878\r\nAccept: */*\r\n\r\n"),
            "GET",
            "/",
            &[("Host", "localhost:7878"), ("Accept", "*/*")],
        )?;
        check_eq(parse_request(b"GET / HTTP/1.0\r\n\r\n"), None)
    });
    case("sample-2", || {
        check_response(
            respond(&req("GET", "/", &[])),
            "HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n<h1>Hello!</h1>",
        )?;
        check_response(
            respond(&req("POST", "/", &[])),
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 5\r\n\r\nOops!",
        )
    });
    case("hidden-1", || {
        // The request line: exactly three non-empty parts separated by single spaces.
        check_eq(parse_request(b""), None)?;
        check_eq(parse_request(b"GET /\r\n"), None)?;
        check_eq(parse_request(b"GET / HTTP/1.1 x\r\n"), None)?;
        check_eq(parse_request(b"GET  / HTTP/1.1\r\n"), None)?;
        check_eq(parse_request(b" / HTTP/1.1\r\n"), None)?;
        check_eq(parse_request(b"GET  HTTP/1.1\r\n"), None)
    });
    case("hidden-2", || {
        // Without an empty line, the headers run to the end of the input. The method and the path
        // are taken as they are (UTF-8 is fine), and a header is split at its first ": " only,
        // without trimming.
        check_req(parse_request(b"GET / HTTP/1.1"), "GET", "/", &[])?;
        check_req(
            parse_request("get /café HTTP/1.1\r\nX:  a: b \r\nE: \r\nH: フェリス".as_bytes()),
            "get",
            "/café",
            &[("X", " a: b "), ("E", ""), ("H", "フェリス")],
        )
    });
    case("hidden-3", || {
        // A header line without ": " makes the request invalid.
        check_eq(parse_request(b"GET / HTTP/1.1\r\nA:b\r\n\r\n"), None)
    });
    case("hidden-4", || {
        // The body is ignored, even if it is not a header or not UTF-8.
        check_req(
            parse_request(b"POST /f HTTP/1.1\r\nA: b\r\n\r\nno colon\r\n\xff"),
            "POST",
            "/f",
            &[("A", "b")],
        )
    });
    case("hidden-5", || {
        // Invalid UTF-8 before the body makes the request invalid.
        check_eq(parse_request(b"GET /caf\xe9 HTTP/1.1\r\n"), None)?;
        check_eq(parse_request(b"GET / HTTP/1.1\r\nX: \xff\r\nA: b\r\n"), None)
    });
    case("hidden-6", || {
        // Content-Length counts bytes: "こんにちは" is 5 characters but 15 bytes.
        check_response(
            respond(&req("GET", "/greet", &[("Accept", "*/*")])),
            "HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\nこんにちは",
        )
    });
    case("hidden-7", || {
        // Everything else is 404, compared exactly.
        let not_found = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 5\r\n\r\nOops!";
        check_response(respond(&req("GET", "/greet/", &[])), not_found)?;
        check_response(respond(&req("get", "/", &[])), not_found)?;
        check_response(respond(&req("HEAD", "/greet", &[])), not_found)
    });
}
