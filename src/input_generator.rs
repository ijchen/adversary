use crate::{generators, Adversarial, Exhaustive, Sample, Shrink};

pub trait InputGenerator<T>: Exhaustive<T> + Adversarial<T> + Sample<T> + Shrink<T> {
    fn adv_map<F: Fn(T) -> U, U>(self, func: F) -> impl InputGenerator<U>
    where
        Self: Sized,
    {
        generators::Map {
            generator: self,
            func,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, G: Exhaustive<T> + Adversarial<T> + Sample<T> + Shrink<T>> InputGenerator<T> for G {}
