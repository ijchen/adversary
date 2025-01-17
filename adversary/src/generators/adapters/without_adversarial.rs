use crate::ValueGen;

pub fn without_adversarial<G: ValueGen>(
    inner_generator: G,
) -> impl ValueGen<Value = G::Value, Seed = G::Seed> {
    WithoutAdversarial { inner_generator }
}

struct WithoutAdversarial<G> {
    inner_generator: G,
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
        self.inner_generator.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        self.inner_generator.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(0)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        std::iter::empty()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        self.inner_generator.sample(rng)
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        self.inner_generator.new_shrinker(failing_value_seed)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        self.inner_generator.create_value(seed)
    }
}
