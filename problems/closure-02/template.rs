pub fn map_pair(f: impl Fn(i32) -> i32, pair: (i32, i32)) -> (i32, i32) {
    (f(pair.0), f(pair.1))
}
