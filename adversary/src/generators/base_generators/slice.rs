use crate::{IntoValueGen, RangeAwareValueGen, ValueGen, report::Observation, shrinker::Shrinker};

impl<'a, T> IntoValueGen<&'a T> for &'a [T] {
    // TODO: use ATPIT once stabilized
    type Gen = SliceValueGen<'a, T>;

    fn into_value_gen(self) -> Self::Gen {
        SliceValueGen(self)
    }
}

impl<'a, T, const N: usize> IntoValueGen<&'a T> for &'a [T; N] {
    // TODO: use ATPIT once stabilized
    type Gen = SliceValueGen<'a, T>;

    fn into_value_gen(self) -> Self::Gen {
        SliceValueGen(self)
    }
}

// TODO: this should not be pub, make private once ATPIT allows IntoValueGen
// impls to hide the concrete type of IntoValueGen::Gen
pub struct SliceValueGen<'a, T>(&'a [T]);

impl<'a, T> ValueGen for SliceValueGen<'a, T> {
    type Value = &'a T;

    type Seed = usize;

    // TODO: use ATPIT once stabilized
    type Shrinker<'b>
        = SliceShrinker
    where
        Self: 'b;

    fn cardinality(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        0..self.0.len()
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(0)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        std::iter::empty()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        rng.gen_range(0..self.0.len())
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        SliceShrinker {
            next_index_to_try: 0,
            lowest_known_failing_index: failing_value_seed,
        }
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        &self.0[seed]
    }
}

impl<'a, T: PartialEq> RangeAwareValueGen for SliceValueGen<'a, T> {
    fn value_in_range(&self, value: &Self::Value) -> bool {
        self.0.contains(value)
    }
}

// TODO: this should not be pub, make private once ATPIT allows ValueGen impls
// to hide the concrete type of ValueGen::Shrinker
pub struct SliceShrinker {
    next_index_to_try: usize,
    lowest_known_failing_index: usize,
}

impl Shrinker<usize> for SliceShrinker {
    fn current_attempt(&self) -> Option<usize> {
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
        // TODO(ichen): useful observations
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        report::{FailureCause, ShrinkStep, TestOutcome},
    };

    #[test]
    fn test_slice() {
        let report = run_test_bool(
            |&n| n < 5,
            [0, 1, 2, 3, 4, 5, 6, 7, 8].as_slice(),
            &mut crate::rand::thread_rng(),
            TestConfig::default(),
        )
        .unwrap_report();
        assert_eq!(report.passing_runs, 5);
        let expected = vec![
            ShrinkStep::new(
                &5,
                false,
                TestOutcome::Failed {
                    cause: FailureCause::NormalFailure,
                },
            ),
            ShrinkStep::new(&0, false, TestOutcome::Passed),
            ShrinkStep::new(&1, false, TestOutcome::Passed),
            ShrinkStep::new(&2, false, TestOutcome::Passed),
            ShrinkStep::new(&3, false, TestOutcome::Passed),
            ShrinkStep::new(&4, false, TestOutcome::Passed),
        ];
        assert_eq!(report.shrink_steps.len(), expected.len());
        for (actual, expected) in report.shrink_steps.iter().zip(expected.iter()) {
            assert_eq!(actual.try_eq(expected), Some(true));
        }
        assert_eq!(report.simplest_failing_value(), &&5);
    }

    #[test]
    fn test_ref_array() {
        let report = run_test_bool(
            |&n| n < 5,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8],
            &mut crate::rand::thread_rng(),
            TestConfig::default(),
        )
        .unwrap_report();
        assert_eq!(report.passing_runs, 5);
        let expected = vec![
            ShrinkStep::new(
                &5,
                false,
                TestOutcome::Failed {
                    cause: FailureCause::NormalFailure,
                },
            ),
            ShrinkStep::new(&0, false, TestOutcome::Passed),
            ShrinkStep::new(&1, false, TestOutcome::Passed),
            ShrinkStep::new(&2, false, TestOutcome::Passed),
            ShrinkStep::new(&3, false, TestOutcome::Passed),
            ShrinkStep::new(&4, false, TestOutcome::Passed),
        ];
        assert_eq!(report.shrink_steps.len(), expected.len());
        for (actual, expected) in report.shrink_steps.iter().zip(expected.iter()) {
            assert_eq!(actual.try_eq(expected), Some(true));
        }
        assert_eq!(report.simplest_failing_value(), &&5);
    }
}
