use crate::{Adversarial, Exhaustive, Sample, Shrink};

pub trait InputGenerator<T>: Exhaustive<T> + Adversarial<T> + Sample<T> + Shrink<T> {}

impl<T, G: Exhaustive<T> + Adversarial<T> + Sample<T> + Shrink<T>> InputGenerator<T> for G {}
