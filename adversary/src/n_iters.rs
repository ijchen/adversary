pub enum IterTwo<A, B> {
    A(A),
    B(B),
}

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            IterTwo::A(iter) => iter.size_hint(),
            IterTwo::B(iter) => iter.size_hint(),
        }
    }

    fn fold<B2, F>(self, init: B2, f: F) -> B2
    where
        Self: Sized,
        F: FnMut(B2, Self::Item) -> B2,
    {
        match self {
            IterTwo::A(iter) => iter.fold(init, f),
            IterTwo::B(iter) => iter.fold(init, f),
        }
    }
}

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            IterThree::A(iter) => iter.size_hint(),
            IterThree::B(iter) => iter.size_hint(),
            IterThree::C(iter) => iter.size_hint(),
        }
    }

    fn fold<B2, F>(self, init: B2, f: F) -> B2
    where
        Self: Sized,
        F: FnMut(B2, Self::Item) -> B2,
    {
        match self {
            IterThree::A(iter) => iter.fold(init, f),
            IterThree::B(iter) => iter.fold(init, f),
            IterThree::C(iter) => iter.fold(init, f),
        }
    }
}
