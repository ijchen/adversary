use std::marker::PhantomData;

use crate::{Adversarial, Exhaustive, InputGenerator, Sample, Shrink};

pub(crate) struct Map<T, U, G, F>
where
    G: InputGenerator<T>,
    F: Fn(T) -> U,
{
    pub(crate) generator: G,
    pub(crate) func: F,
    // TODO(ichen): is this necessary? How does this affect dropck?
    pub(crate) _phantom: PhantomData<T>,
}

// TODO(ichen): is this logically sound? It sort of isn't - we may not iterate
// over all possible values of type U
impl<T, U, G, F> Exhaustive<U> for Map<T, U, G, F>
where
    G: InputGenerator<T>,
    F: Fn(T) -> U,
{
    fn cardinality(&self) -> Option<usize> {
        G::cardinality(&self.generator)
    }

    fn exhaustive(&self) -> impl Iterator<Item = U> {
        G::exhaustive(&self.generator).map(|item| (self.func)(item))
    }
}

// TODO(ichen): I feel like these outputs are no longer necessarily the
// best adversarial ones... maybe we should consider this yeilding nothing?
impl<T, U, G, F> Adversarial<U> for Map<T, U, G, F>
where
    G: InputGenerator<T>,
    F: Fn(T) -> U,
{
    fn adversarial_count(&self) -> Option<usize> {
        G::adversarial_count(&self.generator)
    }

    fn adversarial(&self) -> impl Iterator<Item = U> {
        G::adversarial(&self.generator).map(|item| (self.func)(item))
    }
}

impl<T, U, G, F> Sample<U> for Map<T, U, G, F>
where
    G: InputGenerator<T>,
    F: Fn(T) -> U,
{
    fn sample(&self, rng: &mut impl rand::Rng) -> U {
        (self.func)(self.generator.sample(rng))
    }
}

impl<T, U, G, F> Shrink<U> for Map<T, U, G, F>
where
    G: InputGenerator<T>,
    F: Fn(T) -> U,
{
    type History = ();

    fn history_from_failure(&self, _failing_input: &U) -> Self::History {
        ()
    }

    fn update_history(&self, _history: &mut Self::History, _input: &U, _test_passed: bool) {
        ()
    }

    fn generate_report_details(&self, _history: Self::History) -> String {
        String::from("Array shrinking is not yet supported")
    }

    fn next_input(&self, _rng: &mut impl rand::Rng, _history: &Self::History) -> Option<U> {
        // Shrinking not yet implemented
        None
    }
}
