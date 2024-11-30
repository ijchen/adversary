use crate::{
    generators::adapters::{add_adversarial, map, without_adversarial, WithoutShrinking},
    InputGenerator,
};

pub trait InputGeneratorExt: InputGenerator + Sized {
    // TODO: docs
    fn adv_map<U, F: Fn(Self::Input) -> U>(
        self,
        map_function: F,
    ) -> impl InputGenerator<Input = U, InputSource = Self::InputSource> {
        map(self, map_function)
    }

    // TODO: docs
    fn adv_without_shrinking(
        self,
    ) -> impl InputGenerator<Input = Self::Input, InputSource = Self::InputSource> {
        WithoutShrinking::new(self)
    }

    // TODO: docs
    fn adv_without_adversarial(
        self,
    ) -> impl InputGenerator<Input = Self::Input, InputSource = Self::InputSource> {
        without_adversarial(self)
    }

    // TODO: docs
    fn adv_add_adversarial(
        self,
        additional_adversarial_values: impl Into<Box<[Self::InputSource]>>,
    ) -> impl InputGenerator<Input = Self::Input, InputSource = Self::InputSource> {
        add_adversarial(self, additional_adversarial_values.into())
    }
}

impl<G: InputGenerator> InputGeneratorExt for G {}
