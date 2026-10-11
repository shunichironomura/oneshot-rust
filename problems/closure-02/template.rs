pub fn apply_n(f: impl Fn(), n: usize) {
    for _ in 0..n {
        f();
    }
}

pub fn consume(f: impl Fn() -> String) -> String {
    f()
}

pub fn map_pair(f: impl Fn(i32) -> i32, pair: (i32, i32)) -> (i32, i32) {
    (f(pair.0), f(pair.1))
}

pub fn call_shared(f: impl Fn() -> u32) -> u32 {
    let g = &f;
    g() + g()
}

pub fn ops_table() -> [(char, impl Fn(i32, i32) -> i32); 3] {
    [('+', |a, b| a + b), ('-', |a, b| a - b), ('*', |a, b| a * b)]
}
