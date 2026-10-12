#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub user: String,
    /// In seconds. The session is valid while `now < expires_at`.
    pub expires_at: u64,
    pub admin: bool,
}

pub fn is_admin(s: Option<&Session>) -> bool {
    s.is_some_and(|session| session.admin)
}

pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    limit.is_none_or(|max| attempts < max)
}

pub fn time_left(s: &Session, now: u64) -> Option<u64> {
    (now < s.expires_at).then(|| s.expires_at - now)
}

pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session> {
    slot.take_if(|session| session.expires_at <= now)
}
