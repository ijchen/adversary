mod bool;
mod unit;

use crate::InputGenerator;

pub trait Canonical: Sized {
    fn canonical() -> impl InputGenerator<Input = Self>;
}

pub fn any<T: Canonical>() -> impl InputGenerator<Input = T> {
    T::canonical()
}
