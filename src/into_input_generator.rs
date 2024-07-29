use crate::InputGenerator;

pub trait IntoInputGenerator {
    type Input;
    type IntoInputGenerator: InputGenerator<Input = Self::Input>;

    fn into_input_generator(self) -> Self::IntoInputGenerator;
}

/// Generic impl of `IntoInputGenerator` for any `G: InputGenerator`
impl<G: InputGenerator> IntoInputGenerator for G {
    type Input = <Self as InputGenerator>::Input;

    type IntoInputGenerator = Self;

    #[inline]
    fn into_input_generator(self) -> Self::IntoInputGenerator {
        self
    }
}
