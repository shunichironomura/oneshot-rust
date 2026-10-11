pub fn make_counter(step: u32) -> impl FnMut() -> u32 {
    let mut current = 0;
    move || {
        current += step;
        current
    }
}

pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}
