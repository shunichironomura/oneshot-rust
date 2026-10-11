pub fn make_counter(step: u32) -> impl FnMut() -> u32 {
    || todo!()
}

pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    |_: A| todo!()
}
