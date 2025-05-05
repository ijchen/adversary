use std::error::Error;

use find_failing_value::{FindFailingValueReport, find_failing_value};
use shrink::shrink;
use test::Test;

use crate::{IntoValueGen, ValueGen, rand::Rng, report::Report};

mod config;
mod find_failing_value;
mod shrink;
mod test;
mod test_result;

pub use config::Config;
pub use test_result::TestResult;

pub fn run_test_bool<T>(
    test: fn(T) -> bool,
    generator: impl IntoValueGen<T>,
    rng: &mut (impl Rng + ?Sized),
    config: Config,
) -> TestResult<T> {
    run_test_inner(test, generator.into_value_gen(), rng, config)
}

pub fn run_test_panic<T>(
    test: fn(T),
    generator: impl IntoValueGen<T>,
    rng: &mut (impl Rng + ?Sized),
    config: Config,
) -> TestResult<T> {
    run_test_inner(test, generator.into_value_gen(), rng, config)
}

pub fn run_test_result<T, E: Into<Box<dyn Error>>>(
    test: fn(T) -> Result<(), E>,
    generator: impl IntoValueGen<T>,
    rng: &mut (impl Rng + ?Sized),
    config: Config,
) -> TestResult<T> {
    run_test_inner(test, generator.into_value_gen(), rng, config)
}

pub fn run_test_should_panic<T>(
    test: fn(T),
    generator: impl IntoValueGen<T>,
    rng: &mut (impl Rng + ?Sized),
    config: Config,
) -> TestResult<T> {
    run_test_inner(
        test::should_panic(test),
        generator.into_value_gen(),
        rng,
        config,
    )
}

pub fn run_test_should_panic_with_message<T>(
    test: fn(T),
    message: impl ToString,
    generator: impl IntoValueGen<T>,
    rng: &mut (impl Rng + ?Sized),
    config: Config,
) -> TestResult<T> {
    run_test_inner(
        test::should_panic_with_message(test, message.to_string()),
        generator.into_value_gen(),
        rng,
        config,
    )
}

// TODO: add ability to customize how we sample the generator
// TODO: handle generator impls that lie about their sizes
fn run_test_inner<T>(
    test: impl Test<T>,
    generator: impl ValueGen<Value = T>,
    rng: &mut (impl Rng + ?Sized),
    config: Config,
) -> TestResult<T> {
    let mut generator = generator.into_value_gen();

    // Find a failing value
    let (failing_seed, passing_runs, cause) =
        match find_failing_value(&test, &mut generator, rng, &config) {
            FindFailingValueReport::NeverFailed => return TestResult::Passed,
            FindFailingValueReport::InvalidConfig(err) => return TestResult::InvalidConfig(err),
            FindFailingValueReport::Failed {
                failing_seed,
                passing_runs,
                cause,
            } => (failing_seed, passing_runs, cause),
        };

    // Shrink the failing value
    let (observations, shrink_steps) = shrink(&test, &generator, failing_seed, cause);

    // Generate a report
    TestResult::Failed(Report {
        test_name: config.test_name,
        passing_runs,
        observations,
        shrink_steps,
    })
}
