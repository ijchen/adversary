use crate::{Canonical, Chance, ValueGen, generators::option};

const SOME_CHANCE: Chance = Chance::from_percent(90.0).unwrap();

impl<T: Canonical> Canonical for Option<T> {
    fn canonical() -> impl ValueGen<Value = Self> {
        option(T::canonical(), SOME_CHANCE)
    }
}
