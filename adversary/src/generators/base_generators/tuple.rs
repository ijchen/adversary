use crate::{input_generator::NextAttempt, InputGenerator, IntoInputGenerator};

fn cartesian_product_2<
    A: Iterator,
    B: Iterator,
    AMaker: Copy + Fn() -> A,
    BMaker: Copy + Fn() -> B,
>(
    a_maker: AMaker,
    b_maker: BMaker,
) -> impl Iterator<Item = (A::Item, B::Item)>
where
    A::Item: Clone,
    B::Item: Clone,
{
    a_maker().flat_map(move |a| b_maker().map(move |b| (a.clone(), b)))
}

fn cartesian_product_3<
    A: Iterator,
    B: Iterator,
    C: Iterator,
    AMaker: Copy + Fn() -> A,
    BMaker: Copy + Fn() -> B,
    CMaker: Copy + Fn() -> C,
>(
    a_maker: AMaker,
    b_maker: BMaker,
    c_maker: CMaker,
) -> impl Iterator<Item = (A::Item, B::Item, C::Item)>
where
    A::Item: Clone,
    B::Item: Clone,
    C::Item: Clone,
{
    cartesian_product_2(a_maker, move || cartesian_product_2(b_maker, c_maker))
        .map(|(a, (b, c))| (a, b, c))
}

#[non_exhaustive]
#[doc(hidden)]
pub struct TupleGen<A, B, C>(A, B, C);

impl<
        A,
        B,
        C,
        IntoGenA: IntoInputGenerator<A>,
        IntoGenB: IntoInputGenerator<B>,
        IntoGenC: IntoInputGenerator<C>,
    > IntoInputGenerator<(A, B, C)> for (IntoGenA, IntoGenB, IntoGenC)
{
    type IntoInputGenerator = TupleGen<
        IntoGenA::IntoInputGenerator,
        IntoGenB::IntoInputGenerator,
        IntoGenC::IntoInputGenerator,
    >;

    fn into_input_generator(self) -> Self::IntoInputGenerator {
        TupleGen(
            self.0.into_input_generator(),
            self.1.into_input_generator(),
            self.2.into_input_generator(),
        )
    }
}

#[allow(unused)] // TODO: WIP
pub struct TupleHist3<A, B, C> {
    inner_histories: (A, B, C),
    phase: TupleHist3Phase,
}

#[allow(unused)] // TODO: WIP
enum TupleHist3Phase {
    ElementWiseFirstPass { curr_index: u8 },
    AllTogetherFirstPass,
    AllPairs { first_index: u8, second_index: u8 },
    ElementWiseSecondPass { curr_index: u8 },
    AllTogetherSecondPass,
}

impl<
        A,
        B,
        C,
        GenA: InputGenerator<Input = A>,
        GenB: InputGenerator<Input = B>,
        GenC: InputGenerator<Input = C>,
    > InputGenerator for TupleGen<GenA, GenB, GenC>
{
    type Input = (A, B, C);

    type InputSource = (GenA::InputSource, GenB::InputSource, GenC::InputSource);

    type History = TupleHist3<GenA::History, GenB::History, GenC::History>;

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

    fn new_history(&self, failing_input: Self::InputSource) -> Self::History {
        TupleHist3 {
            inner_histories: (
                self.0.new_history(failing_input.0),
                self.1.new_history(failing_input.1),
                self.2.new_history(failing_input.2),
            ),
            phase: TupleHist3Phase::ElementWiseFirstPass { curr_index: 0 },
        }
    }

    fn next_input(
        &self,
        _rng: &mut impl rand::Rng,
        _history: &Self::History,
    ) -> NextAttempt<Self::InputSource> {
        todo!()
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _shrinkable_input: Self::InputSource,
        _test_passed: bool,
    ) {
        todo!()
    }

    fn generate_observations(&self, _history: Self::History) -> Vec<crate::report::Observation> {
        todo!()
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        (
            self.0.create_input(input_source.0),
            self.1.create_input(input_source.1),
            self.2.create_input(input_source.2),
        )
    }
}
