use crate::{generators::adapters::Map, InputGenerator};

pub trait InputGeneratorExt: InputGenerator {
    // TODO: docs
    fn adv_map<U, F: Fn(Self::Input) -> U>(
        self,
        map_function: F,
    ) -> impl InputGenerator<Input = U, ShrinkableInput = Self::ShrinkableInput, History = Self::History>;
}

impl<G: InputGenerator> InputGeneratorExt for G {
    fn adv_map<U, F: Fn(Self::Input) -> U>(
        self,
        map_function: F,
    ) -> impl InputGenerator<Input = U, ShrinkableInput = Self::ShrinkableInput, History = Self::History>
    {
        Map::new(self, map_function)
    }
}
