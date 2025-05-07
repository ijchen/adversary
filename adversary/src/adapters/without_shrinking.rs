use crate::{RangeAwareValueGen, ValueGen, shrinker::NeverShrink};

#[derive(Debug)]
pub struct WithoutShrinking<G> {
    inner: G,
}

impl<G: ValueGen> WithoutShrinking<G> {
    pub fn new(inner_gen: G) -> Self {
        Self { inner: inner_gen }
    }
}

impl<G: ValueGen> ValueGen for WithoutShrinking<G> {
    type Value = G::Value;
    type Seed = G::Seed;
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = NeverShrink
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        self.inner.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        self.inner.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.inner.adversarial_count()
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        self.inner.adversarial()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        self.inner.sample(rng)
    }

    fn new_shrinker(&self, _seed: Self::Seed) -> Self::Shrinker<'_> {
        NeverShrink::new()
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        self.inner.create_value(seed)
    }
}

impl<G: RangeAwareValueGen> RangeAwareValueGen for WithoutShrinking<G> {
    fn value_in_range(&self, value: &Self::Value) -> bool {
        self.inner.value_in_range(value)
    }
}
