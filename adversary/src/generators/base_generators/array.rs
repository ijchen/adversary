use crate::{report::Observation, shrinker::Shrinker, IntoValueGen, ValueGen};

impl<T: Clone, const N: usize> IntoValueGen<T> for [T; N] {
    fn into_value_gen(self) -> impl ValueGen<Value = T> {
        ArrayValueGen(self)
    }
}

struct ArrayValueGen<T, const N: usize>([T; N]);

impl<T: Clone, const N: usize> ValueGen for ArrayValueGen<T, N> {
    type Value = T;

    type Seed = usize;

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

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> impl Shrinker<Seed = Self::Seed> {
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

impl Shrinker for ArrayShrinker {
    type Seed = usize;

    fn current_attempt(&self) -> Option<Self::Seed> {
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
    use crate::{report::ShrinkStep, run_test};

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
                ShrinkStep::new(5, false, false),
                ShrinkStep::new(0, false, true),
                ShrinkStep::new(1, false, true),
                ShrinkStep::new(2, false, true),
                ShrinkStep::new(3, false, true),
                ShrinkStep::new(4, false, true),
            ]
        );
        assert_eq!(report.simplest_failing_value(), &5);
    }
}
