pub fn byte_size(x: &u64) -> usize {
    std::mem::size_of_val(x)
}
