mod cartesian_product;
mod shrinker;

use cartesian_product::cartesian_product_3;
use shrinker::TupleShrinker3;

use crate::{shrinker::Shrinker, InputGenerator, IntoInputGenerator};

impl<
        A,
        B,
        C,
        IntoGenA: IntoInputGenerator<A>,
        IntoGenB: IntoInputGenerator<B>,
        IntoGenC: IntoInputGenerator<C>,
    > IntoInputGenerator<(A, B, C)> for (IntoGenA, IntoGenB, IntoGenC)
{
    fn into_input_generator(self) -> impl InputGenerator<Input = (A, B, C)> {
        TupleGen3(
            self.0.into_input_generator(),
            self.1.into_input_generator(),
            self.2.into_input_generator(),
        )
    }
}

struct TupleGen3<GenA, GenB, GenC>(GenA, GenB, GenC);

impl<GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator> InputGenerator
    for TupleGen3<GenA, GenB, GenC>
{
    type Input = (GenA::Input, GenB::Input, GenC::Input);

    type InputSource = (GenA::InputSource, GenB::InputSource, GenC::InputSource);

    #[expect(
        clippy::needless_question_mark,
        reason = "this code will eventually be the output of a macro - this pattern is easier w/ repetitions"
    )]
    fn cardinality(&self) -> Option<usize> {
        Some(
            1usize
                .checked_mul(self.0.cardinality()?)?
                .checked_mul(self.1.cardinality()?)?
                .checked_mul(self.2.cardinality()?)?,
        )
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        cartesian_product_3(
            || self.0.exhaustive(),
            || self.1.exhaustive(),
            || self.2.exhaustive(),
        )
    }

    #[expect(
        clippy::needless_question_mark,
        reason = "this code will eventually be the output of a macro - this pattern is easier w/ repetitions"
    )]
    fn adversarial_count(&self) -> Option<usize> {
        Some(
            1usize
                .checked_mul(self.0.adversarial_count()?)?
                .checked_mul(self.1.adversarial_count()?)?
                .checked_mul(self.2.adversarial_count()?)?,
        )
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        cartesian_product_3(
            || self.0.adversarial(),
            || self.1.adversarial(),
            || self.2.adversarial(),
        )
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
        (self.0.sample(rng), self.1.sample(rng), self.2.sample(rng))
    }

    fn new_shrinker(
        &self,
        failing_input: Self::InputSource,
    ) -> impl Shrinker<InputSource = Self::InputSource> {
        TupleShrinker3::new((&self.0, &self.1, &self.2), failing_input)
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        (
            self.0.create_input(input_source.0),
            self.1.create_input(input_source.1),
            self.2.create_input(input_source.2),
        )
    }
}
