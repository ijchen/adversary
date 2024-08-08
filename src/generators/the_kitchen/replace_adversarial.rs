use crate::{
    input_generator::{InputWithShrinkable, NextAttempt},
    InputGenerator,
};

struct ReplaceAdversarial<G: InputGenerator> {
    generator: G,
    adversarial: Box<[InputWithShrinkable<G::Input, G::ShrinkableInput>]>,
}

impl<G: InputGenerator> InputGenerator for ReplaceAdversarial<G>
where
    InputWithShrinkable<G::Input, G::ShrinkableInput>: Clone,
{
    type Input = G::Input;
    type ShrinkableInput = G::ShrinkableInput;

    type History = G::History;

    fn cardinality(&self) -> Option<usize> {
        self.generator.cardinality()
    }

    fn exhaustive(
        &self,
    ) -> impl Iterator<Item = InputWithShrinkable<Self::Input, Self::ShrinkableInput>> {
        self.generator.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(self.adversarial.len())
    }

    fn adversarial(
        &self,
    ) -> impl Iterator<Item = InputWithShrinkable<Self::Input, Self::ShrinkableInput>> {
        self.adversarial.iter().cloned()
    }

    fn sample(
        &self,
        rng: &mut (impl rand::Rng + ?Sized),
    ) -> InputWithShrinkable<Self::Input, Self::ShrinkableInput> {
        self.generator.sample(rng)
    }

    fn new_history(&self) -> Self::History {
        self.generator.new_history()
    }

    fn update_history(
        &self,
        history: &mut Self::History,
        shrinkable_input: Self::ShrinkableInput,
        test_passed: bool,
    ) {
        self.generator
            .update_history(history, shrinkable_input, test_passed)
    }

    fn generate_observations(&self, history: Self::History) -> Vec<crate::report::Observation> {
        self.generator.generate_observations(history)
    }

    fn next_input(
        &self,
        rng: &mut impl rand::Rng,
        history: &Self::History,
    ) -> NextAttempt<Self::Input, Self::ShrinkableInput> {
        self.generator.next_input(rng, history)
    }
}
