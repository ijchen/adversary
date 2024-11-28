use crate::InputGenerator;

pub trait IntoInputGenerator<Input> {
    fn into_input_generator(self) -> impl InputGenerator<Input = Input>;
}

/// Generic impl of `IntoInputGenerator` for any `G: InputGenerator`
impl<G: InputGenerator> IntoInputGenerator<G::Input> for G {
    #[inline]
    fn into_input_generator(self) -> impl InputGenerator<Input = G::Input> {
        self
    }
}
