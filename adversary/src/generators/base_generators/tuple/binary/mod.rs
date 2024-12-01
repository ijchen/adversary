mod shrinker;

use crate::{shrinker::Shrinker, InputGenerator, IntoInputGenerator};

use super::cartesian_product;

impl<A, B, IntoGenA: IntoInputGenerator<A>, IntoGenB: IntoInputGenerator<B>>
    IntoInputGenerator<(A, B)> for (IntoGenA, IntoGenB)
{
    fn into_input_generator(self) -> impl InputGenerator<Input = (A, B)> {
        TupleGen2(self.0.into_input_generator(), self.1.into_input_generator())
    }
}

struct TupleGen2<GenA, GenB>(GenA, GenB);

impl<GenA: InputGenerator, GenB: InputGenerator> InputGenerator for TupleGen2<GenA, GenB> {
    type Input = (GenA::Input, GenB::Input);

    type InputSource = (GenA::InputSource, GenB::InputSource);

    fn cardinality(&self) -> Option<usize> {
        usize::checked_mul(self.0.cardinality()?, self.1.cardinality()?)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        cartesian_product::cartesian_product_2(|| self.0.exhaustive(), || self.1.exhaustive())
    }

    fn adversarial_count(&self) -> Option<usize> {
        usize::checked_mul(self.0.adversarial_count()?, self.1.adversarial_count()?)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        cartesian_product::cartesian_product_2(|| self.0.adversarial(), || self.1.adversarial())
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
        (self.0.sample(rng), self.1.sample(rng))
    }

    fn new_shrinker(
        &self,
        failing_input: Self::InputSource,
    ) -> impl Shrinker<InputSource = Self::InputSource> {
        shrinker::TupleShrinker2::new((&self.0, &self.1), failing_input)
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        (
            self.0.create_input(input_source.0),
            self.1.create_input(input_source.1),
        )
    }
}
