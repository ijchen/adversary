use crate::{Canonical, ValueGen, length_gen};

impl<T: Canonical> Canonical for Vec<T> {
    fn canonical() -> impl ValueGen<Value = Self> {
        crate::vec::VecValueGen::new(T::canonical(), length_gen::<T>())
    }
}
