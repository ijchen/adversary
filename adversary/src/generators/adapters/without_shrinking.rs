use std::marker::PhantomData;

use crate::{report::Observation, shrinker::Shrinker, InputGenerator};

pub struct WithoutShrinking<G>(G);
pub struct WithoutShrinkingShrinker<T>(PhantomData<T>); // lol

impl<G: InputGenerator> InputGenerator for WithoutShrinking<G> {
    type Input = G::Input;
    type InputSource = G::InputSource;

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

    fn new_shrinker(
        &self,
        _failing_input: Self::InputSource,
    ) -> impl Shrinker<InputSource = Self::InputSource> {
        WithoutShrinkingShrinker(PhantomData)
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        self.0.create_input(input_source)
    }
}

impl<T: Clone> Shrinker for WithoutShrinkingShrinker<T> {
    type InputSource = T;

    fn current_attempt(&self) -> Option<Self::InputSource> {
        None
    }

    fn update(&mut self, _current_attempt_passed: bool) {}

    fn into_observations(self) -> Vec<Observation> {
        Vec::new()
    }
}

impl<G: InputGenerator> WithoutShrinking<G> {
    pub fn new(inner_generator: G) -> Self {
        Self(inner_generator)
    }
}
