use crate::{Adversarial, Exhaustive, Sample, Shrink};

impl<
        T0: Clone,
        T1: Clone,
        T2: Clone,
        G0: Exhaustive<T0>,
        G1: Exhaustive<T1>,
        G2: Exhaustive<T2>,
    > Exhaustive<(T0, T1, T2)> for (G0, G1, G2)
{
    fn cardinality(&self) -> Option<usize> {
        Some(
            1usize
                .checked_mul(G0::cardinality(&self.0)?)?
                .checked_mul(G1::cardinality(&self.1)?)?
                .checked_mul(G2::cardinality(&self.2)?)?,
        )
    }

    fn exhaustive(&self) -> impl Iterator<Item = (T0, T1, T2)> {
        // TODO: this is gross and I'm not even 100% sure it's correct
        G0::exhaustive(&self.0)
            .flat_map(|t1| {
                std::iter::repeat(t1).zip(
                    G1::exhaustive(&self.1)
                        .flat_map(|t2| std::iter::repeat(t2).zip(G2::exhaustive(&self.2))),
                )
            })
            .map(|(t1, (t2, t3))| (t1, t2, t3))
    }
}

impl<
        T0: Clone,
        T1: Clone,
        T2: Clone,
        G0: Adversarial<T0>,
        G1: Adversarial<T1>,
        G2: Adversarial<T2>,
    > Adversarial<(T0, T1, T2)> for (G0, G1, G2)
{
    fn adversarial_count(&self) -> usize {
        1usize
            .checked_mul(G0::adversarial_count(&self.0))
            .unwrap() // TODO: these can actually panic
            .checked_mul(G1::adversarial_count(&self.1))
            .unwrap() // TODO: these can actually panic
            .checked_mul(G2::adversarial_count(&self.2))
            .unwrap() // TODO: these can actually panic
    }

    fn adversarial(&self) -> impl Iterator<Item = (T0, T1, T2)> {
        // TODO: this is gross and I'm not even 100% sure it's correct
        G0::adversarial(&self.0)
            .flat_map(|t1| {
                std::iter::repeat(t1).zip(
                    G1::adversarial(&self.1)
                        .flat_map(|t2| std::iter::repeat(t2).zip(G2::adversarial(&self.2))),
                )
            })
            .map(|(t1, (t2, t3))| (t1, t2, t3))
    }
}

impl<T0, T1, T2, G0: Sample<T0>, G1: Sample<T1>, G2: Sample<T2>> Sample<(T0, T1, T2)>
    for (G0, G1, G2)
{
    fn sample(&self, rng: &mut impl rand::Rng) -> (T0, T1, T2) {
        (self.0.sample(rng), self.1.sample(rng), self.2.sample(rng))
    }
}

impl<T0, T1, T2, G0: Shrink<T0>, G1: Shrink<T1>, G2: Shrink<T2>> Shrink<(T0, T1, T2)>
    for (G0, G1, G2)
{
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
