use crate::{report::Observation, shrinker::Shrinker, InputGenerator};

/// Okay so hear me out - what if we implemented [`InputGenerator`] for arrays?
///
/// TODO: consider long-term API/orphan rules/specialization consequences

impl<T: Clone, const N: usize> InputGenerator for [T; N] {
    type Input = T;
    type InputSource = usize;

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

    fn new_shrinker(
        &self,
        failing_input: Self::InputSource,
    ) -> impl Shrinker<InputSource = Self::InputSource> {
        assert!(!self.is_empty());

        ArrayShrinker {
            next_index_to_try: 0,
            lowest_known_failing_index: failing_input,
        }
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        self[input_source].clone()
    }
}

// TODO: combine with SliceShrinker
pub struct ArrayShrinker {
    next_index_to_try: usize,
    lowest_known_failing_index: usize,
}

impl Shrinker for ArrayShrinker {
    type InputSource = usize;

    fn current_attempt(&self) -> Option<Self::InputSource> {
        (self.next_index_to_try < self.lowest_known_failing_index).then_some(self.next_index_to_try)
    }

    fn update(&mut self, current_attempt_passed: bool) {
        if !current_attempt_passed {
            self.lowest_known_failing_index = self.next_index_to_try;
        } else {
            self.next_index_to_try += 1;
        }
    }

    fn into_observations(self) -> Vec<Observation> {
        // TODO: consider providing some more useful observations
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{run_test, ShrinkStep};

    #[test]
    fn test_array() {
        let report = run_test(
            |n| n < 5,
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            &mut crate::rand::thread_rng(),
        )
        .unwrap_err();
        assert_eq!(report.passing_runs, 5);
        assert_eq!(
            report.shrink_steps,
            vec![
                ShrinkStep::new(0, false, true),
                ShrinkStep::new(1, false, true),
                ShrinkStep::new(2, false, true),
                ShrinkStep::new(3, false, true),
                ShrinkStep::new(4, false, true),
            ]
        );
        assert_eq!(report.simplest_failing_input, 5);
    }
}
