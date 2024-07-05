use crate::{Adversarial, Exhaustive, Sample, Shrink};

use super::Canonical;

impl<T0: Clone, T1: Clone, T2: Clone> Exhaustive<(T0, T1, T2)> for Canonical
where
    Canonical: Exhaustive<T0>,
    Canonical: Exhaustive<T1>,
    Canonical: Exhaustive<T2>,
{
    fn cardinality(&self) -> Option<usize> {
        <(Self, Self, Self) as Exhaustive<(T0, T1, T2)>>::cardinality(&(Self, Self, Self))
    }

    fn exhaustive(&self) -> impl Iterator<Item = (T0, T1, T2)> {
        <(Self, Self, Self) as Exhaustive<(T0, T1, T2)>>::exhaustive(&(Self, Self, Self))
    }
}

impl<T0: Clone, T1: Clone, T2: Clone> Adversarial<(T0, T1, T2)> for Canonical
where
    Canonical: Adversarial<T0>,
    Canonical: Adversarial<T1>,
    Canonical: Adversarial<T2>,
{
    fn adversarial_count(&self) -> usize {
        <(Self, Self, Self) as Adversarial<(T0, T1, T2)>>::adversarial_count(&(Self, Self, Self))
    }

    fn adversarial(&self) -> impl Iterator<Item = (T0, T1, T2)> {
        <(Self, Self, Self) as Adversarial<(T0, T1, T2)>>::adversarial(&(Self, Self, Self))
    }
}

impl<T0, T1, T2> Sample<(T0, T1, T2)> for Canonical
where
    Canonical: Sample<T0>,
    Canonical: Sample<T1>,
    Canonical: Sample<T2>,
{
    fn sample(&self, rng: &mut impl rand::Rng) -> (T0, T1, T2) {
        <(Self, Self, Self) as Sample<(T0, T1, T2)>>::sample(&(Self, Self, Self), rng)
    }
}

impl<T0, T1, T2> Shrink<(T0, T1, T2)> for Canonical {
    type History = ();

    fn history_from_failure(&self, _failing_input: &(T0, T1, T2)) -> Self::History {
        todo!()
    }

    fn generate_report_details(&self, _history: Self::History) -> String {
        todo!()
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _input: &(T0, T1, T2),
        _test_passed: bool,
    ) {
        todo!()
    }

    fn next_input(
        &self,
        _rng: &mut impl rand::Rng,
        _history: &Self::History,
    ) -> Option<(T0, T1, T2)> {
        todo!()
    }
}
