use crate::{
    generators::adapters::{add_adversarial, flatten, map, without_adversarial, WithoutShrinking},
    IntoValueGen, ValueGen,
};

pub trait ValueGenExt: ValueGen + Sized {
    // TODO: docs
    fn adv_map<F: Fn(Self::Value) -> T, T>(
        self,
        map_function: F,
    ) -> impl ValueGen<Value = T, Seed = Self::Seed> {
        map(self, map_function)
    }

    // TODO: docs
    fn adv_flatten<T>(self) -> impl ValueGen<Value = T>
    where
        Self::Value: IntoValueGen<T>,
    {
        flatten(self)
    }

    // TODO: docs
    fn adv_flat_map<F: Fn(Self::Value) -> G, G: IntoValueGen<T>, T>(
        self,
        map_function: F,
    ) -> impl ValueGen<Value = T> {
        self.adv_map(map_function).adv_flatten()
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
