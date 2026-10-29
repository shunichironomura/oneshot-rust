use std::borrow::Cow;

pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32) {
    // Look first: `to_mut` would copy a borrowed slice even if nothing changes.
    if v.iter().any(|&x| x > max) {
        for x in v.to_mut().iter_mut() {
            *x = (*x).min(max);
        }
    }
}
