use crate::{input_generator::NextAttempt, InputGenerator};

struct AddAdversarial<G: InputGenerator> {
    generator: G,
    adversarial: Box<[G::Input]>,
}

impl<G: InputGenerator> InputGenerator for AddAdversarial<G>
where
    G::Input: Clone,
{
    type Input = G::Input;

    type History = G::History;

    fn cardinality(&self) -> Option<usize> {
        self.generator.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Input> {
        self.generator.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.generator
            .adversarial_count()?
            .checked_add(self.adversarial.len())
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Input> {
        self.generator
            .adversarial()
            .chain(self.adversarial.iter().cloned())
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Input {
        self.generator.sample(rng)
    }

    fn new_history(&self) -> Self::History {
        self.generator.new_history()
    }

    fn update_history(&self, history: &mut Self::History, input: &Self::Input, test_passed: bool) {
        self.generator.update_history(history, input, test_passed)
    }

    fn generate_observations(&self, history: Self::History) -> Vec<crate::report::Observation> {
        self.generator.generate_observations(history)
    }

    fn next_input(
        &self,
        rng: &mut impl rand::Rng,
        history: &Self::History,
    ) -> NextAttempt<Self::Input> {
        self.generator.next_input(rng, history)
    }
}
