use crate::{
    IntoValueGen, ValueGen,
    adapters::{AddAdversarial, Flatten, Map, WithoutAdversarial, WithoutShrinking},
};

pub trait ValueGenExt: ValueGen + Sized {
    // TODO: docs
    fn adv_map<T, F: Fn(Self::Value) -> T>(self, map_function: F) -> Map<Self, F> {
        Map::new(self, map_function)
    }

    // TODO: docs
    fn adv_flatten<T>(self) -> Flatten<Self, <Self::Value as IntoValueGen<T>>::Gen>
    where
        Self::Value: IntoValueGen<T>,
    {
        Flatten::new(self)
    }

    // TODO: docs
    fn adv_flat_map<F: Fn(Self::Value) -> G, G: IntoValueGen<T>, T>(
        self,
        map_function: F,
    ) -> Flatten<Map<Self, F>, G::Gen> {
        self.adv_map(map_function).adv_flatten()
    }

    // TODO: docs
    fn adv_without_shrinking(self) -> WithoutShrinking<Self> {
        WithoutShrinking::new(self)
    }

    // TODO: docs
    fn adv_without_adversarial(self) -> WithoutAdversarial<Self> {
        WithoutAdversarial::new(self)
    }

    // TODO: docs
    fn adv_add_adversarial<I: Into<Box<[Self::Seed]>>>(
        self,
        additional_adversarial_values: I,
    ) -> AddAdversarial<Self> {
        AddAdversarial::new(self, additional_adversarial_values.into())
    }
}

impl<G: ValueGen> ValueGenExt for G {}
