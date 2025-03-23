use crate::rand::Rng;

use crate::IntoValueGen;
use crate::sample::sample;
use crate::shrinker::Shrinker as _;
use crate::{ValueGen, report::Report, report::ShrinkStep};

struct FailingValueReport<T, I> {
    pub failing_value: T,
    pub failing_value_seed: I,
    pub passing_runs: u64,
}

fn find_failing_value<T, I: Clone>(
    test: &impl Fn(T) -> bool,
    generator: &mut impl ValueGen<Value = T, Seed = I>,
    rng: &mut impl Rng,
) -> Option<FailingValueReport<T, I>> {
    // TODO: allow customizing these
    const MAX_RUNS: usize = 1_000_000;
    const MIN_RANDOM_INPUTS: usize = MAX_RUNS / 5;

    const { assert!(MIN_RANDOM_INPUTS <= MAX_RUNS) }

    // TODO(ichen): consider the cost of enum dispatch here, and if it's worth
    // instead inlining (compiler might be smart enough to see through all this
    // anyway - measure, as always)
    let seeds = sample(generator, MAX_RUNS, MIN_RANDOM_INPUTS, rng);

    let mut passing_runs: u64 = 0;
    for seed in seeds {
        let test_passed = test(generator.create_value(seed.clone()));

        if !test_passed {
            return Some(FailingValueReport {
                failing_value: generator.create_value(seed.clone()),
                failing_value_seed: seed,
                passing_runs,
            });
        }

        // NOTE(ichen): Saturating add because the documentation on `Report`
        // indicates that:
        // > [`u64::MAX`] indicates that the test failed [`usize::MAX`]
        // > *or more* times.
        // FWIW, at 50 billion iterations per second, it would take over 11
        // years to reach u64::MAX
        passing_runs = passing_runs.saturating_add(1);
    }

    None
}

fn shrink_and_generate_report<T, I: Clone>(
    test: &impl Fn(T) -> bool,
    generator: &impl ValueGen<Value = T, Seed = I>,
    failing_value_report: FailingValueReport<T, I>,
) -> Report<T> {
    let mut shrinker = generator.new_shrinker(failing_value_report.failing_value_seed);

    let mut shrink_steps = vec![ShrinkStep::new(
        failing_value_report.failing_value,
        false,
        false,
    )];
    // TODO(ichen): limit how many times this loop can run (to guard against
    // faulty Shrinker impls)
    loop {
        // TODO: allow info-gathering attempts
        let Some(seed) = shrinker.current_attempt() else {
            break;
        };

        let test_passed = test(generator.create_value(seed.clone()));

        shrinker.update(test_passed);

        shrink_steps.push(ShrinkStep::new(
            generator.create_value(seed),
            false,
            test_passed,
        ));
    }

    Report {
        test_name: None,
        panic_info: None,
        passing_runs: failing_value_report.passing_runs,
        observations: shrinker.into_observations(),
        shrink_steps,
    }
}

// TODO: add ability to customize how we sample the generator
// TODO: handle generator impls that lie about their sizes
pub fn run_test<T>(
    test: impl Fn(T) -> bool,
    generator: impl IntoValueGen<T>,
    rng: &mut impl Rng,
) -> Result<(), Box<Report<T>>> {
    let mut generator = generator.into_value_gen();

    // Find a failing value
    let Some(failing_value_report) = find_failing_value(&test, &mut generator, rng) else {
        return Ok(());
    };

    // Shrink the failing value and generate a report
    Err(Box::new(shrink_and_generate_report(
        &test,
        &generator,
        failing_value_report,
    )))
}
