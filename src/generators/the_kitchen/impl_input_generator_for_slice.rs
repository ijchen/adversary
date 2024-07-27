use crate::{input_generator::NextAttempt, InputGenerator};

/// Okay so hear me out - what if we implemented [`InputGenerator`] for slices?
///
/// TODO: consider long-term API/orphan rules/specialization consequences

impl<'a, T> InputGenerator for &'a [T] {
    type Input = &'a T;
    type InputIdentifier = Self::Input;

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        Some(self.len())
    }

    fn exhaustive(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        self.into_iter().map(|v| (v, v))
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(0)
    }

    fn adversarial(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        std::iter::empty()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> (Self::Input, Self::InputIdentifier) {
        let v = crate::rand::seq::SliceRandom::choose(*self, rng).unwrap();

        (v, v)
    }

    fn new_history(&self) -> Self::History {
        ()
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _input_identifier: Self::InputIdentifier,
        _test_passed: bool,
    ) {
    }

    fn generate_observations(&self, _history: Self::History) -> Vec<crate::report::Observation> {
        vec![]
    }

    fn next_input(
        &self,
        _rng: &mut impl crate::rand::Rng,
        _history: &Self::History,
    ) -> NextAttempt<(Self::Input, Self::InputIdentifier)> {
        NextAttempt::Done
    }
}

impl<'a, T: 'a, const N: usize> InputGenerator for &'a [T; N] {
    type Input = &'a T;
    type InputIdentifier = Self::Input;

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        Some(N)
    }

    fn exhaustive(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        self.into_iter().map(|v| (v, v))
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(0)
    }

    fn adversarial(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        std::iter::empty()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> (Self::Input, Self::InputIdentifier) {
        let v = crate::rand::seq::SliceRandom::choose(self.as_slice(), rng).unwrap();

        (v, v)
    }

    fn new_history(&self) -> Self::History {
        ()
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _input_identifier: Self::InputIdentifier,
        _test_passed: bool,
    ) {
    }

    fn generate_observations(&self, _history: Self::History) -> Vec<crate::report::Observation> {
        vec![]
    }

    fn next_input(
        &self,
        _rng: &mut impl crate::rand::Rng,
        _history: &Self::History,
    ) -> NextAttempt<(Self::Input, Self::InputIdentifier)> {
        NextAttempt::Done
    }
}

#[cfg(test)]
mod tests {
    use crate::run_test;

    #[test]
    fn test_slice() {
        let report = run_test(
            |&&n| n < 5,
            [0, 1, 2, 3, 4, 5, 6, 7, 8].as_slice(),
            &mut crate::rand::thread_rng(),
        )
        .unwrap_err();
        assert_eq!(report.passing_runs, 5);
        assert_eq!(report.shrink_steps, vec![]);
        assert_eq!(report.simplest_failing_input, &5);
    }

    #[test]
    fn test_ref_array() {
        let report = run_test(
            |&&n| n < 5,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8],
            &mut crate::rand::thread_rng(),
        )
        .unwrap_err();
        assert_eq!(report.passing_runs, 5);
        assert_eq!(report.shrink_steps, vec![]);
        assert_eq!(report.simplest_failing_input, &5);
    }
}
