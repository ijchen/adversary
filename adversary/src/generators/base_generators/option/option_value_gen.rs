use crate::{
    Chance, RangeAwareValueGen, ValueGen,
    generators::base_generators::option::shrinker::OptionShrinker, n_iters::IterThree,
};

/// A [`ValueGen`] that produces `Option`s from an inner [`ValueGen`].
pub fn option<G: ValueGen>(
    value_gen: G,
    some_chance: Chance,
) -> impl ValueGen<Value = Option<G::Value>, Seed = Option<G::Seed>> {
    OptionValueGen::new(value_gen, some_chance)
}

struct OptionValueGen<G: ValueGen> {
    value_gen: G,
    some_chance: Chance,
}

impl<G: ValueGen> OptionValueGen<G> {
    pub fn new(value_gen: G, some_chance: Chance) -> Self {
        Self {
            value_gen,
            some_chance,
        }
    }
}

impl<G: ValueGen> ValueGen for OptionValueGen<G> {
    type Value = Option<G::Value>;

    type Seed = Option<G::Seed>;

    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = OptionShrinker<'a, G>
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        match self.some_chance {
            Chance::IMPOSSIBLE => Some(1),
            Chance::GUARANTEED => Some(self.value_gen.cardinality()?),
            _ => self
                .value_gen
                .cardinality()
                .and_then(|cardinality| cardinality.checked_add(1)),
        }
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        match self.some_chance {
            Chance::IMPOSSIBLE => IterThree::A(std::iter::once(None)),
            Chance::GUARANTEED => IterThree::B(self.value_gen.exhaustive().map(Some)),
            _ => IterThree::C(std::iter::once(None).chain(self.value_gen.exhaustive().map(Some))),
        }
    }

    fn adversarial_count(&self) -> Option<usize> {
        match self.some_chance {
            Chance::IMPOSSIBLE => Some(1),
            Chance::GUARANTEED => Some(self.value_gen.adversarial_count()?),
            _ => self
                .value_gen
                .adversarial_count()
                .and_then(|cardinality| cardinality.checked_add(1)),
        }
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        match self.some_chance {
            Chance::IMPOSSIBLE => IterThree::A(std::iter::once(None)),
            Chance::GUARANTEED => IterThree::B(self.value_gen.adversarial().map(Some)),
            _ => IterThree::C(std::iter::once(None).chain(self.value_gen.adversarial().map(Some))),
        }
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        match self.some_chance.gen_bool(rng) {
            true => Some(self.value_gen.sample(rng)),
            false => None,
        }
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        OptionShrinker::new(&self.value_gen, self.some_chance, failing_value_seed)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        seed.map(|seed| self.value_gen.create_value(seed))
    }
}

impl<G: RangeAwareValueGen> RangeAwareValueGen for OptionValueGen<G> {
    fn value_in_range(&self, value: &Self::Value) -> bool {
        match value {
            Some(value) => self.some_chance.is_possible() && self.value_gen.value_in_range(value),
            None => !self.some_chance.is_guaranteed(),
        }
    }
}
