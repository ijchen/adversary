use crate::{Adversarial, Exhaustive, Sample, Shrink};

use super::Canonical;

impl<T, const N: usize> Exhaustive<[T; N]> for Canonical
where
    Canonical: Exhaustive<T>,
{
    fn cardinality(&self) -> Option<usize> {
        usize::checked_mul(Self::cardinality(&self)?, N)
    }

    fn exhaustive(&self) -> impl Iterator<Item = [T; N]> {
        todo!();
        #[allow(unreachable_code)]
        std::iter::empty()
    }
}

impl<T, const N: usize> Adversarial<[T; N]> for Canonical
where
    Canonical: Adversarial<T>,
{
    fn adversarial_count(&self) -> Option<usize> {
        usize::checked_mul(Self::adversarial_count(&self)?, N)
    }

    fn adversarial(&self) -> impl Iterator<Item = [T; N]> {
        todo!();
        #[allow(unreachable_code)]
        std::iter::empty()
    }
}

impl<T, const N: usize> Sample<[T; N]> for Canonical
where
    Canonical: Sample<T>,
{
    fn sample(&self, rng: &mut impl rand::Rng) -> [T; N] {
        std::array::from_fn(|_| Self::sample(&Self, rng))
    }
}

// TODO: do a much more thoughtful and improved implementation - this is just a
// basic impl to get started with.
impl<T, const N: usize> Shrink<[T; N]> for Canonical
where
    Canonical: Shrink<T>,
{
    type History = ();

    fn history_from_failure(&self, _failing_input: &[T; N]) -> Self::History {
        ()
    }

    fn generate_report_details(&self, _history: Self::History) -> String {
        String::from("Array shrinking is not yet supported")
    }

    fn update_history(&self, _history: &mut Self::History, _input: &[T; N], _test_passed: bool) {
        ()
    }

    fn next_input(&self, _rng: &mut impl rand::Rng, _history: &Self::History) -> Option<[T; N]> {
        // Shrinking not yet implemented
        None
    }
}
