use crate::{report::Observation, shrinker::Shrinker, InputGenerator, IntoInputGenerator};

impl<T: Clone, const N: usize> IntoInputGenerator<T> for [T; N] {
    fn into_input_generator(self) -> impl InputGenerator<Input = T> {
        ArrayInputGenerator(self)
    }
}

struct ArrayInputGenerator<T, const N: usize>([T; N]);

impl<T: Clone, const N: usize> InputGenerator for ArrayInputGenerator<T, N> {
    type Input = T;

    type InputSource = usize;

    fn cardinality(&self) -> Option<usize> {
        Some(N)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        0..N
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(0)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        std::iter::empty()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
        rng.gen_range(0..N)
    }

    fn new_shrinker(
        &self,
        failing_input: Self::InputSource,
    ) -> impl Shrinker<InputSource = Self::InputSource> {
        ArrayShrinker {
            next_index_to_try: 0,
            lowest_known_failing_index: failing_input,
        }
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        self.0[input_source].clone()
    }
}

// TODO(ichen): Combine this with SliceShrinker
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
        assert_eq!(report.simplest_failing_input(), &5);
    }
}
