use crate::ValueGen;

// TODO(ichen): consider implications of users incorrectly adding adversarial
// values outside the set of correct values for the generator (ex, adding `3` to
// a shrinker of integers in `10..20`)
pub fn add_adversarial<G: ValueGen>(
    inner_generator: G,
    additional_adversarial_values: Box<[G::Seed]>,
) -> impl ValueGen<Value = G::Value, Seed = G::Seed> {
    AddAdversarial {
        inner_generator,
        additional_adversarial_values,
    }
}

struct AddAdversarial<G, U> {
    inner_generator: G,
    additional_adversarial_values: Box<[U]>,
}

impl<G: ValueGen> ValueGen for AddAdversarial<G, G::Seed> {
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
        self.inner_generator
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
            .chain(self.inner_generator.adversarial())
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
