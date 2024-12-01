use crate::{InputGenerator, InputGeneratorExt, IntoInputGenerator};

impl<T, IntoGen: IntoInputGenerator<T>> IntoInputGenerator<(T,)> for (IntoGen,) {
    fn into_input_generator(self) -> impl InputGenerator<Input = (T,)> {
        self.0.into_input_generator().adv_map(|value| (value,))
    }
}
