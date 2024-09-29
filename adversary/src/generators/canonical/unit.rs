use crate::{input_generator::NextAttempt, report::Observation, Canonical, InputGenerator};

struct CanonicalUnitGenerator;

impl InputGenerator for CanonicalUnitGenerator {
    type Input = ();
    type InputSource = ();

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        Some(1)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        std::iter::once(())
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(1)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        std::iter::once(())
    }

    fn sample(&self, _rng: &mut (impl crate::rand::Rng + ?Sized)) -> Self::InputSource {
        ()
    }

    fn new_history(&self, _failing_input: Self::InputSource) -> Self::History {
        ()
    }

    fn current_simplest_failing(&self, _history: &Self::History) -> Self::InputSource {
        ()
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _shrinkable_input: Self::InputSource,
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
    ) -> NextAttempt<Self::InputSource> {
        NextAttempt::Done
    }

    fn create_input(&self, _input_source: Self::InputSource) -> Self::Input {
        ()
    }
}

impl Canonical for () {
    fn canonical() -> impl InputGenerator<Input = Self> {
        CanonicalUnitGenerator
    }
}
