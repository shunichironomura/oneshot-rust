#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub user: String,
    /// In seconds. The session is valid while `now < expires_at`.
    pub expires_at: u64,
    pub admin: bool,
}

#[derive(Debug, PartialEq)]
pub enum AuthError {
    NoSession,
    Expired,
    NotAdmin,
}

pub fn is_admin(s: Option<&Session>) -> bool {
    todo!()
}

pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    todo!()
}

pub fn time_left(s: &Session, now: u64) -> Option<u64> {
    todo!()
}

pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session> {
    todo!()
}

pub fn authorize(s: Option<&Session>, now: u64) -> Result<(), AuthError> {
    todo!()
}
