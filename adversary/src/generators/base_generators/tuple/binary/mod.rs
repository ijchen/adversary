mod shrinker;

use crate::{IntoValueGen, RangeAwareValueGen, ValueGen};

use super::cartesian_product;

impl<A, B, IntoGenA: IntoValueGen<A>, IntoGenB: IntoValueGen<B>> IntoValueGen<(A, B)>
    for (IntoGenA, IntoGenB)
{
    // TODO: use ATPIT once stabilized
    type Gen = TupleGen2<IntoGenA::Gen, IntoGenB::Gen>;

    fn into_value_gen(self) -> Self::Gen {
        TupleGen2(self.0.into_value_gen(), self.1.into_value_gen())
    }
}

// TODO: this should not be pub, make private once ATPIT allows IntoValueGen
// impls to hide the concrete type of IntoValueGen::Gen
pub struct TupleGen2<GenA, GenB>(GenA, GenB);

impl<GenA: ValueGen, GenB: ValueGen> ValueGen for TupleGen2<GenA, GenB> {
    type Value = (GenA::Value, GenB::Value);
    type Seed = (GenA::Seed, GenB::Seed);
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = shrinker::TupleShrinker2<'a, GenA, GenB>
    where
        Self: 'a;

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

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        shrinker::TupleShrinker2::new((&self.0, &self.1), failing_value_seed)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        (self.0.create_value(seed.0), self.1.create_value(seed.1))
    }
}

impl<GenA: RangeAwareValueGen, GenB: RangeAwareValueGen> RangeAwareValueGen
    for TupleGen2<GenA, GenB>
{
    fn value_in_range(&self, value: &Self::Value) -> bool {
        self.0.value_in_range(&value.0) && self.1.value_in_range(&value.1)
    }
}
