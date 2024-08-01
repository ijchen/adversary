use crate::{
    input_generator::{InputWithShrinkable, NextAttempt},
    report::Observation,
    Canonical, InputGenerator,
};

struct CanonicalUnitGenerator;

impl InputGenerator for CanonicalUnitGenerator {
    type Input = ();
    type ShrinkableInput = Self::Input;

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        Some(1)
    }

    fn exhaustive(
        &self,
    ) -> impl Iterator<Item = InputWithShrinkable<Self::Input, Self::ShrinkableInput>> {
        std::iter::once(InputWithShrinkable((), ()))
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(1)
    }

    fn adversarial(
        &self,
    ) -> impl Iterator<Item = InputWithShrinkable<Self::Input, Self::ShrinkableInput>> {
        std::iter::once(InputWithShrinkable((), ()))
    }

    fn sample(
        &self,
        _rng: &mut (impl crate::rand::Rng + ?Sized),
    ) -> InputWithShrinkable<Self::Input, Self::ShrinkableInput> {
        InputWithShrinkable((), ())
    }

    fn new_history(&self) -> Self::History {
        ()
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _shrinkable_input: Self::ShrinkableInput,
        _test_passed: bool,
    ) {
    }

    fn generate_observations(&self, _history: Self::History) -> Vec<Observation> {
        Vec::new()
    }

    fn next_input(
        &self,
        _rng: &mut impl crate::rand::Rng,
        _history: &Self::History,
    ) -> NextAttempt<Self::Input, Self::ShrinkableInput> {
        NextAttempt::Done
    }
}

impl Canonical for () {
    fn canonical() -> impl InputGenerator<Input = Self> {
        CanonicalUnitGenerator
    }
}
