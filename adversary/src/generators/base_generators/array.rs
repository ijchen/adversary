use crate::{IntoValueGen, ValueGen, report::Observation, shrinker::Shrinker};

impl<T: Clone, const N: usize> IntoValueGen<T> for [T; N] {
    // TODO: use ATPIT once stabilized
    type Gen = ArrayValueGen<T, N>;

    fn into_value_gen(self) -> Self::Gen {
        ArrayValueGen(self)
    }
}

// TODO: this should not be pub, make private once ATPIT allows IntoValueGen
// impls to hide the concrete type of IntoValueGen::Gen
pub struct ArrayValueGen<T, const N: usize>([T; N]);

impl<T: Clone, const N: usize> ValueGen for ArrayValueGen<T, N> {
    type Value = T;
    type Seed = usize;
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = ArrayShrinker
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        Some(N)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        0..N
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(0)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        std::iter::empty()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        rng.gen_range(0..N)
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        ArrayShrinker {
            next_index_to_try: 0,
            lowest_known_failing_index: failing_value_seed,
        }
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        self.0[seed].clone()
    }
}

// TODO(ichen): Combine this with SliceShrinker
pub struct ArrayShrinker {
    next_index_to_try: usize,
    lowest_known_failing_index: usize,
}

impl Shrinker<usize> for ArrayShrinker {
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
        report::{ShrinkStep, TestOutcome},
    };

    #[test]
    fn test_array() {
        let report = run_test_bool(
            |n| n < 5,
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            &mut crate::rand::thread_rng(),
            Config::default(),
        )
        .unwrap_report();
        assert_eq!(report.passing_runs, 5);
        let expected = vec![
            ShrinkStep::new(
                5,
                false,
                TestOutcome::Failed {
                    cause: FailureCause::NormalFailure,
                },
            ),
            ShrinkStep::new(0, false, TestOutcome::Passed),
            ShrinkStep::new(1, false, TestOutcome::Passed),
            ShrinkStep::new(2, false, TestOutcome::Passed),
            ShrinkStep::new(3, false, TestOutcome::Passed),
            ShrinkStep::new(4, false, TestOutcome::Passed),
        ];
        assert_eq!(report.shrink_steps.len(), expected.len());
        for (actual, expected) in report.shrink_steps.iter().zip(expected.iter()) {
            assert_eq!(actual.try_eq(expected), Some(true));
        }
        assert_eq!(report.simplest_failing_value(), &5);
    }
}
