pub fn apply_n(mut f: impl FnMut(), n: usize) {
    for _ in 0..n {
        f();
    }
}

pub fn consume<T>(f: impl FnOnce() -> T) -> T {
    f()
}

pub fn map_pair<A, B>(mut f: impl FnMut(A) -> B, pair: (A, A)) -> (B, B) {
    (f(pair.0), f(pair.1))
}

pub fn call_shared(f: impl Fn() -> u32) -> u32 {
    let g = &f;
    g() + g()
}

pub fn ops_table() -> [(char, fn(i32, i32) -> i32); 3] {
    [('+', |a, b| a + b), ('-', |a, b| a - b), ('*', |a, b| a * b)]
}
