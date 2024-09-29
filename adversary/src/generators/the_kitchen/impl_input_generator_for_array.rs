use crate::{input_generator::NextAttempt, report::Observation, InputGenerator};

/// Okay so hear me out - what if we implemented [`InputGenerator`] for arrays?
///
/// TODO: consider long-term API/orphan rules/specialization consequences

impl<T: Clone, const N: usize> InputGenerator for [T; N] {
    type Input = T;
    type InputSource = usize;

    type History = Self::InputSource;

    fn cardinality(&self) -> Option<usize> {
        assert!(!self.is_empty());

        Some(N)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        assert!(!self.is_empty());

        0..self.len()
    }

    fn adversarial_count(&self) -> Option<usize> {
        assert!(!self.is_empty());

        Some(0)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        assert!(!self.is_empty());

        std::iter::empty()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
        assert!(!self.is_empty());

        rng.gen_range(0..self.len())
    }

    fn new_history(&self, failing_input: Self::InputSource) -> Self::History {
        assert!(!self.is_empty());

        failing_input
    }

    fn current_simplest_failing(&self, history: &Self::History) -> Self::InputSource {
        *history
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _shrinkable_input: Self::InputSource,
        _test_passed: bool,
    ) {
        assert!(!self.is_empty());

        // TODO(ichen): implement for real
    }

    fn generate_observations(&self, _history: Self::History) -> Vec<Observation> {
        assert!(!self.is_empty());

        // TODO(ichen): implement for real
        vec![]
    }

    fn next_input(
        &self,
        _rng: &mut impl crate::rand::Rng,
        _history: &Self::History,
    ) -> NextAttempt<Self::InputSource> {
        assert!(!self.is_empty());

        // TODO(ichen): implement for real
        NextAttempt::Done
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        self[input_source].clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::run_test;

    #[test]
    fn test_array() {
        let report = run_test(
            |n| n < 5,
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            &mut crate::rand::thread_rng(),
        )
        .unwrap_err();
        assert_eq!(report.passing_runs, 5);
        assert_eq!(report.shrink_steps, vec![]);
        assert_eq!(report.simplest_failing_input, 5);
    }
}
