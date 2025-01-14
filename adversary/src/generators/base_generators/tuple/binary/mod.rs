mod shrinker;

use crate::{shrinker::Shrinker, IntoValueGen, ValueGen};

use super::cartesian_product;

impl<A, B, IntoGenA: IntoValueGen<A>, IntoGenB: IntoValueGen<B>> IntoValueGen<(A, B)>
    for (IntoGenA, IntoGenB)
{
    fn into_value_gen(self) -> impl ValueGen<Value = (A, B)> {
        TupleGen2(self.0.into_value_gen(), self.1.into_value_gen())
    }
}

struct TupleGen2<GenA, GenB>(GenA, GenB);

impl<GenA: ValueGen, GenB: ValueGen> ValueGen for TupleGen2<GenA, GenB> {
    type Value = (GenA::Value, GenB::Value);

    type Seed = (GenA::Seed, GenB::Seed);

    fn cardinality(&self) -> Option<usize> {
        usize::checked_mul(self.0.cardinality()?, self.1.cardinality()?)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        cartesian_product::cartesian_product_2(|| self.0.exhaustive(), || self.1.exhaustive())
    }

    fn adversarial_count(&self) -> Option<usize> {
        usize::checked_mul(self.0.adversarial_count()?, self.1.adversarial_count()?)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        cartesian_product::cartesian_product_2(|| self.0.adversarial(), || self.1.adversarial())
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        (self.0.sample(rng), self.1.sample(rng))
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> impl Shrinker<Seed = Self::Seed> {
        shrinker::TupleShrinker2::new((&self.0, &self.1), failing_value_seed)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        (self.0.create_value(seed.0), self.1.create_value(seed.1))
    }
}
