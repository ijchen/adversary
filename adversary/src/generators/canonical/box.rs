use crate::{Canonical, ValueGen, generators::boxed};

impl<T: Canonical> Canonical for Box<T> {
    fn canonical() -> impl ValueGen<Value = Self> {
        boxed(T::canonical())
    }
}
