pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    limit.is_none_or(|max| attempts < max)
}
