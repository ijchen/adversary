use crate::{RangeAwareValueGen, ValueGen};

#[derive(Debug)]
pub struct WithoutAdversarial<G> {
    inner: G,
}

impl<G: ValueGen> WithoutAdversarial<G> {
    pub fn new(inner_gen: G) -> Self {
        Self { inner: inner_gen }
    }
}

impl<G: ValueGen> ValueGen for WithoutAdversarial<G> {
    type Value = G::Value;
    type Seed = G::Seed;
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = G::Shrinker<'a>
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        self.inner.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        self.inner.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(0)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        std::iter::empty()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        self.inner.sample(rng)
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        self.inner.new_shrinker(failing_value_seed)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        self.inner.create_value(seed)
    }
}

impl<G: RangeAwareValueGen> RangeAwareValueGen for WithoutAdversarial<G> {
    fn value_in_range(&self, value: &Self::Value) -> bool {
        self.inner.value_in_range(value)
    }
}
