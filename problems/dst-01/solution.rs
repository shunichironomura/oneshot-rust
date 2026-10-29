pub fn byte_size<T: ?Sized>(x: &T) -> usize {
    std::mem::size_of_val(x)
}
