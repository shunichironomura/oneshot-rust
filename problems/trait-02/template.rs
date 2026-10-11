pub trait Pairs: Iterator + Sized {
    fn pairs(self) -> PairsIter<Self>;
}

pub struct PairsIter<I: Iterator> {
    iter: I,
    // Add more fields if you need them.
}

impl<I> Pairs for I
where
    I: Iterator,
    I::Item: Clone,
{
    fn pairs(self) -> PairsIter<I> {
        todo!()
    }
}

impl<I> Iterator for PairsIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
