mod bool;
mod r#box;
mod ints;
mod option;
mod tuple;
mod unit;
mod vec;

use crate::ValueGen;

pub trait Canonical: Sized {
    fn canonical() -> impl ValueGen<Value = Self> + Send + Sync + Unpin;
}

pub fn any<T: Canonical>() -> impl ValueGen<Value = T> + Send + Sync + Unpin {
    T::canonical()
}
