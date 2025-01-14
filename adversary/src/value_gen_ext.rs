use crate::{
    generators::adapters::{add_adversarial, map, without_adversarial, WithoutShrinking},
    ValueGen,
};

pub trait ValueGenExt: ValueGen + Sized {
    // TODO: docs
    fn adv_map<U, F: Fn(Self::Value) -> U>(
        self,
        map_function: F,
    ) -> impl ValueGen<Value = U, Seed = Self::Seed> {
        map(self, map_function)
    }

    // TODO: docs
    fn adv_without_shrinking(self) -> impl ValueGen<Value = Self::Value, Seed = Self::Seed> {
        WithoutShrinking::new(self)
    }

    // TODO: docs
    fn adv_without_adversarial(self) -> impl ValueGen<Value = Self::Value, Seed = Self::Seed> {
        without_adversarial(self)
    }

    // TODO: docs
    fn adv_add_adversarial(
        self,
        additional_adversarial_values: impl Into<Box<[Self::Seed]>>,
    ) -> impl ValueGen<Value = Self::Value, Seed = Self::Seed> {
        add_adversarial(self, additional_adversarial_values.into())
    }
}

impl<G: ValueGen> ValueGenExt for G {}
