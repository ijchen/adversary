use crate::ValueGen;

#[derive(Debug)]
pub struct AddAdversarial<G: ValueGen> {
    inner: G,
    additional_adversarial_values: Box<[G::Seed]>,
}

impl<G: ValueGen> AddAdversarial<G> {
    // TODO(ichen): consider implications of users incorrectly adding
    // adversarial values outside the set of correct values for the generator
    // (ex, adding `3` to a ValueGen of integers in `10..20`)
    pub fn new(inner_gen: G, additional_adversarial_values: Box<[G::Seed]>) -> Self {
        Self {
            inner: inner_gen,
            additional_adversarial_values,
        }
    }
}

impl<G: ValueGen> ValueGen for AddAdversarial<G> {
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
        self.inner
            .adversarial_count()
            .and_then(|adversarial_count| {
                usize::checked_add(adversarial_count, self.additional_adversarial_values.len())
            })
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        // TODO(ichen): consider whether we want the added adversarial values to
        // be added before or after the inner generator's adversarial values
        self.additional_adversarial_values
            .iter()
            .cloned()
            .chain(self.inner.adversarial())
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
