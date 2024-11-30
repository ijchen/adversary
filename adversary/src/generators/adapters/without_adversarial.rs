use crate::{shrinker::Shrinker, InputGenerator};

pub fn without_adversarial<G: InputGenerator>(
    inner_generator: G,
) -> impl InputGenerator<Input = G::Input, InputSource = G::InputSource> {
    WithoutAdversarial { inner_generator }
}

struct WithoutAdversarial<G> {
    inner_generator: G,
}

impl<G: InputGenerator> InputGenerator for WithoutAdversarial<G> {
    type Input = G::Input;

    type InputSource = G::InputSource;

    fn cardinality(&self) -> Option<usize> {
        self.inner_generator.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        self.inner_generator.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(0)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        std::iter::empty()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
        self.inner_generator.sample(rng)
    }

    fn new_shrinker(
        &self,
        failing_input: Self::InputSource,
    ) -> impl Shrinker<InputSource = Self::InputSource> {
        self.inner_generator.new_shrinker(failing_input)
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        self.inner_generator.create_input(input_source)
    }
}
