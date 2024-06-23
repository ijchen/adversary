pub trait Exhaustive<T> {
    /// TODO: better docs
    /// The total number of possible values. None if too large to fit in a usize
    fn cardinality(&self) -> Option<usize>;

    /// TODO: better docs
    /// Returns an iterator yielding all values once.
    fn exhaustive(&self) -> impl Iterator<Item = T>;
}
