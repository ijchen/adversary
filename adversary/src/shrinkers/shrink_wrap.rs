use crate::{report::Observation, shrinker::Shrinker, ValueGen};

/// A [`Shrinker`] for some outer [`ValueGen`] `Outer`, which just delegates to
/// an inner `Shrinker` for another `ValueGen` `Inner`, using the provided
/// function to convert from `Outer` to `Inner`.
pub struct ShrinkWrap<Outer: ValueGen + ?Sized, Inner: ValueGen<Seed = Outer::Seed> + ?Sized> {
    inner: Inner::Shrinker,
    gen_converter: fn(&Outer) -> &Inner,
}

impl<Outer: ValueGen + ?Sized, Inner: ValueGen<Seed = Outer::Seed> + ?Sized>
    ShrinkWrap<Outer, Inner>
{
    pub fn new(inner: Inner::Shrinker, gen_converter: fn(&Outer) -> &Inner) -> Self {
        Self {
            inner,
            gen_converter,
        }
    }
}

impl<Outer: ValueGen + ?Sized, Inner: ValueGen<Seed = Outer::Seed> + ?Sized> Shrinker<Outer>
    for ShrinkWrap<Outer, Inner>
{
    fn current_attempt(&self) -> Option<<Outer as ValueGen>::Seed> {
        self.inner.current_attempt()
    }

    fn update(&mut self, gen: &Outer, current_attempt_passed: bool) {
        let gen = (self.gen_converter)(gen);
        self.inner.update(gen, current_attempt_passed)
    }

    fn into_observations(self) -> Vec<Observation> {
        self.inner.into_observations()
    }
}
