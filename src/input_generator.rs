use crate::{Adversarial, Exhaustive, Sample, Shrink};

pub trait InputGenerator<T: Clone>: Exhaustive<T> + Adversarial<T> + Sample<T> + Shrink<T> {}
