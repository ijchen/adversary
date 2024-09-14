mod bool;
mod unit;

use crate::InputGenerator;

pub trait Canonical: Sized {
    fn canonical() -> impl InputGenerator<Input = Self> + Send + Sync + Unpin;
}

pub fn any<T: Canonical>() -> impl InputGenerator<Input = T> + Send + Sync + Unpin {
    T::canonical()
}
