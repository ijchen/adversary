use crate::InputGenerator;

// TODO(ichen): should this be fallible, or maybe have both a fallible and
// infallible version? Consider integer ranges - nothing stops 5..3 from being
// created or attempted to be turned into an input generator. The current impl
// panics in this case - is that really the best we can do?
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
