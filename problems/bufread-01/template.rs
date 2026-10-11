#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn parse_request(raw: &[u8]) -> Option<Request> {
    todo!()
}

pub fn respond(req: &Request) -> Vec<u8> {
    todo!()
}
