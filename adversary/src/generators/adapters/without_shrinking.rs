use crate::{input_generator::NextAttempt, InputGenerator};

pub struct WithoutShrinking<G>(G);

impl<G: InputGenerator> InputGenerator for WithoutShrinking<G> {
    type Input = G::Input;
    type InputSource = G::InputSource;

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        self.0.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        self.0.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.0.adversarial_count()
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        self.0.adversarial()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
        self.0.sample(rng)
    }

    fn new_history(&self) -> Self::History {
        ()
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _shrinkable_input: Self::InputSource,
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
    ) -> NextAttempt<Self::InputSource> {
        NextAttempt::Done
    }

    fn create_input(&self, input_source: &Self::InputSource) -> Self::Input {
        self.0.create_input(input_source)
    }
}

impl<G: InputGenerator> WithoutShrinking<G> {
    pub fn new(inner_generator: G) -> Self {
        Self(inner_generator)
    }
}
