pub struct RunsMut<'a, T> {
    rest: &'a mut [T],
}

pub fn runs_mut<T: PartialEq>(v: &mut [T]) -> RunsMut<'_, T> {
    todo!()
}

impl<'a, T: PartialEq> Iterator for RunsMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<&'a mut [T]> {
        todo!()
    }
}

impl<'a, T: PartialEq> DoubleEndedIterator for RunsMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        todo!()
    }
}
