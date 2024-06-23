use rand::Rng;

pub trait Sample<T> {
    /// TODO: better docs
    /// Randomly samples an element from all possible values (not necessarily
    /// with equal distribution)
    fn sample(&self, rng: &mut impl Rng) -> T;
}
