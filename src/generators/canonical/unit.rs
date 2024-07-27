use crate::{input_generator::NextAttempt, report::Observation, Canonical, InputGenerator};

struct CanonicalUnitGenerator;

impl InputGenerator for CanonicalUnitGenerator {
    type Input = ();
    type InputIdentifier = Self::Input;

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        Some(1)
    }

    fn exhaustive(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        std::iter::once(((), ()))
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(1)
    }

    fn adversarial(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        std::iter::once(((), ()))
    }

    fn sample(
        &self,
        _rng: &mut (impl crate::rand::Rng + ?Sized),
    ) -> (Self::Input, Self::InputIdentifier) {
        ((), ())
    }

    fn new_history(&self) -> Self::History {
        ()
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _input_identifier: Self::InputIdentifier,
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
    ) -> NextAttempt<(Self::Input, Self::InputIdentifier)> {
        NextAttempt::Done
    }
}

impl Canonical for () {
    fn canonical() -> impl InputGenerator<Input = Self> {
        CanonicalUnitGenerator
    }
}
