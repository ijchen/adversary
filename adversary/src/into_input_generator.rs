use crate::InputGenerator;

pub trait IntoInputGenerator<Input> {
    type IntoInputGenerator: InputGenerator<Input = Input>;

    fn into_input_generator(self) -> Self::IntoInputGenerator;
}

/// Generic impl of `IntoInputGenerator` for any `G: InputGenerator`
impl<G: InputGenerator> IntoInputGenerator<G::Input> for G {
    type IntoInputGenerator = Self;

    #[inline]
    fn into_input_generator(self) -> Self::IntoInputGenerator {
        self
    }
}
