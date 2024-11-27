// use crate::{input_generator::NextAttempt, InputGenerator};

// struct ReplaceAdversarial<G: InputGenerator> {
//     generator: G,
//     adversarial: Box<[G::InputSource]>,
// }

// impl<G: InputGenerator> InputGenerator for ReplaceAdversarial<G>
// where
//     G::InputSource: Clone,
// {
//     type Input = G::Input;
//     type InputSource = G::InputSource;

//     type Shrinker = G::Shrinker;

//     fn cardinality(&self) -> Option<usize> {
//         self.generator.cardinality()
//     }

//     fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
//         self.generator.exhaustive()
//     }

//     fn adversarial_count(&self) -> Option<usize> {
//         Some(self.adversarial.len())
//     }

//     fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
//         self.adversarial.iter().cloned()
//     }

//     fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
//         self.generator.sample(rng)
//     }

//     fn new_shrinker(&self, failing_input: Self::InputSource) -> Self::Shrinker {
//         self.generator.new_shrinker(failing_input)
//     }

//     fn current_simplest_failing(&self, history: &Self::Shrinker) -> Self::InputSource {
//         self.generator.current_simplest_failing(history)
//     }

//     fn update_history(
//         &self,
//         history: &mut Self::Shrinker,
//         shrinkable_input: Self::InputSource,
//         test_passed: bool,
//     ) {
//         self.generator
//             .update_history(history, shrinkable_input, test_passed)
//     }

//     fn generate_observations(&self, history: Self::Shrinker) -> Vec<Observation> {
//         self.generator.generate_observations(history)
//     }

//     fn next_input(
//         &self,
//         rng: &mut impl rand::Rng,
//         history: &Self::Shrinker,
//     ) -> NextAttempt<Self::InputSource> {
//         self.generator.next_input(rng, history)
//     }

//     fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
//         self.generator.create_input(input_source)
//     }
// }
