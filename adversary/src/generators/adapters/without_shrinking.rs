use crate::{shrinker::Shrinker, shrinkers::NeverShrink, ValueGen};

pub struct WithoutShrinking<G>(G);

impl<G: ValueGen> ValueGen for WithoutShrinking<G> {
    type Value = G::Value;
    type Seed = G::Seed;

    fn cardinality(&self) -> Option<usize> {
        self.0.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.0.adversarial_count()
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.adversarial()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        self.0.sample(rng)
    }

    fn new_shrinker(&self, _seed: Self::Seed) -> impl Shrinker<Seed = Self::Seed> {
        NeverShrink::new()
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        self.0.create_value(seed)
    }
}

impl<G: ValueGen> WithoutShrinking<G> {
    pub fn new(inner_generator: G) -> Self {
        Self(inner_generator)
    }
}
