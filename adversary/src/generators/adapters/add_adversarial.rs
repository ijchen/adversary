use crate::{shrinker::Shrinker, InputGenerator};

// TODO(ichen): consider implications of users incorrectly adding adversarial
// values outside the set of correct values for the generator (ex, adding `3` to
// a shrinker of integers in `10..20`)
pub fn add_adversarial<G: InputGenerator>(
    inner_generator: G,
    additional_adversarial_values: Box<[G::InputSource]>,
) -> impl InputGenerator<Input = G::Input, InputSource = G::InputSource> {
    AddAdversarial {
        inner_generator,
        additional_adversarial_values,
    }
}

struct AddAdversarial<G, U> {
    inner_generator: G,
    additional_adversarial_values: Box<[U]>,
}

impl<G: InputGenerator> InputGenerator for AddAdversarial<G, G::InputSource> {
    type Input = G::Input;

    type InputSource = G::InputSource;

    fn cardinality(&self) -> Option<usize> {
        self.inner_generator.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        self.inner_generator.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.inner_generator
            .adversarial_count()
            .and_then(|adversarial_count| {
                usize::checked_add(adversarial_count, self.additional_adversarial_values.len())
            })
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        // TODO(ichen): consider whether we want the added adversarial inputs to
        // be added before or after the inner generator's adversarial inputs
        self.additional_adversarial_values
            .iter()
            .cloned()
            .chain(self.inner_generator.adversarial())
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
