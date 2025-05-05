use crate::ValueGen;

pub fn map<G: ValueGen, U>(
    inner_generator: G,
    map_function: impl Fn(G::Value) -> U,
) -> impl ValueGen<Value = U, Seed = G::Seed> {
    Map {
        inner_generator,
        f: map_function,
    }
}

struct Map<G, F> {
    inner_generator: G,
    f: F,
}

impl<U, G: ValueGen, F: Fn(G::Value) -> U> ValueGen for Map<G, F> {
    type Value = U;
    type Seed = G::Seed;
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = G::Shrinker<'a>
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        self.inner_generator.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        self.inner_generator.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.inner_generator.adversarial_count()
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        self.inner_generator.adversarial()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        self.inner_generator.sample(rng)
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        self.inner_generator.new_shrinker(failing_value_seed)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        (self.f)(self.inner_generator.create_value(seed))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        report::{FailureCause, ShrinkStep, TestOutcome},
    };

    #[test]
    fn test_map_does_the_map_thing() {
        let report = run_test_bool(
            |n| n.len() == 1,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
                .into_value_gen()
                .adv_map(|t| t.to_string()),
            &mut crate::rand::thread_rng(),
            TestConfig::default(),
        )
        .unwrap_report();
        assert_eq!(report.passing_runs, 10);
        let expected = vec![
            ShrinkStep::new(
                "10".to_string(),
                false,
                TestOutcome::Failed {
                    cause: FailureCause::NormalFailure,
                },
            ),
            ShrinkStep::new("0".to_string(), false, TestOutcome::Passed),
            ShrinkStep::new("1".to_string(), false, TestOutcome::Passed),
            ShrinkStep::new("2".to_string(), false, TestOutcome::Passed),
            ShrinkStep::new("3".to_string(), false, TestOutcome::Passed),
            ShrinkStep::new("4".to_string(), false, TestOutcome::Passed),
            ShrinkStep::new("5".to_string(), false, TestOutcome::Passed),
            ShrinkStep::new("6".to_string(), false, TestOutcome::Passed),
            ShrinkStep::new("7".to_string(), false, TestOutcome::Passed),
            ShrinkStep::new("8".to_string(), false, TestOutcome::Passed),
            ShrinkStep::new("9".to_string(), false, TestOutcome::Passed),
        ];
        assert_eq!(report.shrink_steps.len(), expected.len());
        for (actual, expected) in report.shrink_steps.iter().zip(expected.iter()) {
            assert_eq!(actual.try_eq(expected), Some(true));
        }
        assert_eq!(report.simplest_failing_value(), "10");
    }
}
