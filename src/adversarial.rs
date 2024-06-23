pub trait Adversarial<T> {
    /// TODO: better docs
    /// Returns the number of adversarial values
    fn adversarial_count(&self) -> usize;

    /// TODO: better docs
    /// Returns an iterator of all the adversarial values
    fn adversarial(&self) -> impl Iterator<Item = T>;
}
