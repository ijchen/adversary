use crate::{input_generator::NextAttempt, InputGenerator};

struct WithoutShrinking<G>(G);

impl<G: InputGenerator> InputGenerator for WithoutShrinking<G> {
    type Input = G::Input;

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        self.0.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Input> {
        self.0.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.0.adversarial_count()
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Input> {
        self.0.adversarial()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Input {
        self.0.sample(rng)
    }

    fn new_history(&self) -> Self::History {
        ()
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _input: &Self::Input,
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
    ) -> NextAttempt<Self::Input> {
        NextAttempt::Done
    }
}
