#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub user: String,
    /// In seconds. The session is valid while `now < expires_at`.
    pub expires_at: u64,
    pub admin: bool,
}

pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session> {
    todo!()
}
