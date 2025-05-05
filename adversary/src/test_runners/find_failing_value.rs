use crate::ValueGen;
use crate::rand::Rng;
use crate::report::{FailureCause, TestOutcome};
use crate::sample::sample;

use super::test::Test;
use super::test_config::TestConfig;

pub fn find_failing_value<T, S: Clone>(
    test: &impl Test<T>,
    generator: &impl ValueGen<Value = T, Seed = S>,
    rng: &mut (impl Rng + ?Sized),
    config: &TestConfig,
) -> FindFailingValueReport<S> {
    let TestConfig {
        max_attempts,
        min_randomized_attempts,
        ..
    } = config;

    // Ensure the provided configuration is valid
    if min_randomized_attempts > max_attempts {
        return FindFailingValueReport::InvalidConfig(format!(
            "can't sample at least {min_randomized_attempts} random seeds without going over a total of {max_attempts} seeds"
        ));
    }

    // TODO(ichen): consider the cost of enum dispatch here, and if it's worth
    // instead inlining (compiler might be smart enough to see through all this
    // anyway - measure, as always)
    let seeds = sample(generator, *max_attempts, *min_randomized_attempts, rng);

    let mut passing_runs: u64 = 0;
    for seed in seeds {
        let outcome = test.test(generator.create_value(seed.clone()));

        match outcome {
            TestOutcome::Passed => { /* Nothing to do here, carry on */ }
            TestOutcome::Failed { cause } => {
                // TODO: I'd like for the compiler to optimize for the path where
                // the test passes. My intuition says this will help with test
                // performance - but I have not done any real benchmarking yet.
                #[rustfmt::skip] #[inline(always)] #[cold] const fn cold() {}
                cold();

                return FindFailingValueReport::Failed {
                    failing_seed: seed,
                    passing_runs,
                    cause,
                };
            }
        }

        // NOTE(ichen): Saturating add because the documentation on `Report`
        // indicates that:
        //
        // > [`u64::MAX`] indicates that the test failed [`u64::MAX`] *or more*
        // > times. In other words, test runs over this value will saturate.
        //
        // FWIW, at 50 billion iterations per second, it would take over 11
        // years to reach u64::MAX
        passing_runs = passing_runs.saturating_add(1);
    }

    FindFailingValueReport::NeverFailed
}

pub enum FindFailingValueReport<T> {
    NeverFailed,
    InvalidConfig(String),
    Failed {
        failing_seed: T,
        passing_runs: u64,
        cause: FailureCause,
    },
}
