use std::io::BufRead;

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
    let [method, path, "HTTP/1.1"] = parts[..] else {
        return None;
    };

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
