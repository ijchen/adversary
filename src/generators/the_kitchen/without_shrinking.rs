use crate::{input_generator::NextAttempt, InputGenerator};

struct WithoutShrinking<G>(G);

impl<G: InputGenerator> InputGenerator for WithoutShrinking<G> {
    type Input = G::Input;
    type InputIdentifier = ();

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        self.0.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        self.0.exhaustive().map(|(v, _)| (v, ()))
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.0.adversarial_count()
    }

    fn adversarial(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        self.0.adversarial().map(|(v, _)| (v, ()))
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> (Self::Input, Self::InputIdentifier) {
        (self.0.sample(rng).0, ())
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

    fn generate_observations(&self, _history: Self::History) -> Vec<crate::report::Observation> {
        vec![]
    }

    fn next_input(
        &self,
        _rng: &mut impl rand::Rng,
        _history: &Self::History,
    ) -> NextAttempt<(Self::Input, Self::InputIdentifier)> {
        NextAttempt::Done
    }
}
