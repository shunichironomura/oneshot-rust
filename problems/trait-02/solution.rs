pub trait Pairs: Iterator + Sized {
    fn pairs(self) -> PairsIter<Self>;
}

pub struct PairsIter<I: Iterator> {
    iter: I,
    prev: Option<I::Item>,
}

impl<I> Pairs for I
where
    I: Iterator,
    I::Item: Clone,
{
    fn pairs(self) -> PairsIter<I> {
        PairsIter {
            iter: self,
            prev: None,
        }
    }
}

impl<I> Iterator for PairsIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first = match self.prev.take() {
            Some(item) => item,
            None => self.iter.next()?,
        };
        let second = self.iter.next()?;
        self.prev = Some(second.clone());
        Some((first, second))
    }
}
