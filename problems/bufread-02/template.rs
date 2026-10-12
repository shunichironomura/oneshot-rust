use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn respond(req: &Request, out: &mut impl Write) -> io::Result<()> {
    todo!()
}
