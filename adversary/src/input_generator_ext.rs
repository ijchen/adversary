use crate::{
    generators::adapters::{Map, WithoutShrinking},
    InputGenerator,
};

pub trait InputGeneratorExt: InputGenerator + Sized {
    // TODO: docs
    fn adv_map<U, F: Fn(Self::Input) -> U>(
        self,
        map_function: F,
    ) -> impl InputGenerator<Input = U, InputSource = Self::InputSource, History = Self::History>
    {
        Map::new(self, map_function)
    }

    // TODO: docs
    fn adv_without_shrinking(self) -> impl InputGenerator<Input = Self::Input> {
        WithoutShrinking::new(self)
    }
}

impl<G: InputGenerator> InputGeneratorExt for G {}
