pub enum IterTwo<A, B> {
    A(A),
    B(B),
}

// TODO(ijchen): implement important default methods (or just pull in a dependency for this)
impl<A, B> Iterator for IterTwo<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterTwo::A(iter) => iter.next(),
            IterTwo::B(iter) => iter.next(),
        }
    }
}

// TODO(ijchen): implement important default methods (or just pull in a dependency for this)
pub enum IterThree<A, B, C> {
    A(A),
    B(B),
    C(C),
}

impl<A, B, C> Iterator for IterThree<A, B, C>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
    C: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterThree::A(iter) => iter.next(),
            IterThree::B(iter) => iter.next(),
            IterThree::C(iter) => iter.next(),
        }
    }
}
