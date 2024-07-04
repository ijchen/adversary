use crate::{Adversarial, Exhaustive, Sample, Shrink};

pub trait InputGenerator<T>: Exhaustive<T> + Adversarial<T> + Sample<T> + Shrink<T> {}
