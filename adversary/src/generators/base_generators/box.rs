use crate::{RangeAwareValueGen, ValueGen};

struct BoxGen<G: ValueGen>(G);

impl<G: ValueGen> BoxGen<G> {
    pub fn new(value_gen: G) -> Self {
        Self(value_gen)
    }
}

impl<G: ValueGen> ValueGen for BoxGen<G> {
    type Value = Box<G::Value>;

    type Seed = G::Seed;

    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = G::Shrinker<'a>
    where
        Self: 'a;

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

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        self.0.new_shrinker(failing_value_seed)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        Box::new(self.0.create_value(seed))
    }
}

// TODO: probably want to publicly expose the actual ValueGen types instead of
// existential types everywhere, that way users get things like the compiler
// knowing that the BoxGen<G: RangeAwareValueGen> is RangeAwareValueGen (where
// currently, this impl is basically useless)
impl<G: RangeAwareValueGen> RangeAwareValueGen for BoxGen<G> {
    fn value_in_range(&self, value: &Self::Value) -> bool {
        self.0.value_in_range(value)
    }
}

/// A [`ValueGen`] that boxes the values from an inner [`ValueGen`].
pub fn boxed<G: ValueGen>(value_gen: G) -> impl ValueGen<Value = Box<G::Value>, Seed = G::Seed> {
    BoxGen::new(value_gen)
}
