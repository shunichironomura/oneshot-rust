pub fn map_pair<A, B>(mut f: impl FnMut(A) -> B, pair: (A, A)) -> (B, B) {
    (f(pair.0), f(pair.1))
}
