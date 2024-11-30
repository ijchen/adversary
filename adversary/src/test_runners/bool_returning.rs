use crate::rand::Rng;

use crate::shrinker::Shrinker as _;
use crate::IntoInputGenerator;
use crate::{report::ShrinkStep, InputGenerator, Report};

struct FailingInputReport<T, I> {
    pub failing_input: T,
    pub failing_input_source: I,
    pub passing_runs: u64,
}

fn find_failing_input<T, I: Clone>(
    test: &impl Fn(T) -> bool,
    generator: &mut impl InputGenerator<Input = T, InputSource = I>,
    rng: &mut impl Rng,
) -> Option<FailingInputReport<T, I>> {
    #[inline]
    fn helper<T, I: Clone>(
        test: &impl Fn(T) -> bool,
        generator: &impl InputGenerator<Input = T, InputSource = I>,
        inputs: impl Iterator<Item = I>,
    ) -> Option<FailingInputReport<T, I>> {
        // TODO: maybe make this all functional and appease the lambda bros
        let mut passing_runs: u64 = 0;
        for input_source in inputs {
            let test_passed = test(generator.create_input(input_source.clone()));

            if !test_passed {
                return Some(FailingInputReport {
                    failing_input: generator.create_input(input_source.clone()),
                    failing_input_source: input_source,
                    passing_runs,
                });
            }

            // TODO(ichen): maybe possibly consider handling overflow better
            // than saturating (although, FWIW, at 50 billion inputs per second,
            // it would take over 11 years to reach u64::MAX)
            passing_runs = passing_runs.saturating_add(1);
        }

        None
    }

    // TODO: allow customizing this
    const MAX_RUNS: usize = 1_000_000;

    // TODO(ichen): consider the cost of triple-monomorphization here, and
    // possible alternatives.
    if generator
        .cardinality()
        .is_some_and(|cardinality| cardinality <= MAX_RUNS)
    {
        helper(&test, generator, generator.exhaustive())
    } else if generator
        .adversarial_count()
        .is_some_and(|adversarial_count| adversarial_count <= MAX_RUNS)
    {
        helper(
            &test,
            generator,
            generator
                .adversarial()
                .chain(std::iter::repeat_with(|| generator.sample(rng)))
                .take(MAX_RUNS),
        )
    } else {
        helper(
            &test,
            generator,
            std::iter::repeat_with(|| generator.sample(rng)).take(MAX_RUNS),
        )
    }
}

fn shrink_and_generate_report<T, I: Clone>(
    test: &impl Fn(T) -> bool,
    generator: &impl InputGenerator<Input = T, InputSource = I>,
    failing_input_report: FailingInputReport<T, I>,
) -> Report<T> {
    let mut shrinker = generator.new_shrinker(failing_input_report.failing_input_source);

    let mut shrink_steps = vec![ShrinkStep::new(
        failing_input_report.failing_input,
        false,
        false,
    )];
    // TODO(ichen): limit how many times this loop can run (to guard against
    // faulty Shrinker impls)
    loop {
        // TODO: allow info-gathering attempts
        let Some(input_source) = shrinker.current_attempt() else {
            break;
        };

        let test_passed = test(generator.create_input(input_source.clone()));

        shrinker.update(test_passed);

        shrink_steps.push(ShrinkStep::new(
            generator.create_input(input_source),
            false,
            test_passed,
        ));
    }

    let ShrinkStep {
        value: simplest_failing_input,
        just_informational: simplest_failing_input_just_informational,
        test_passed: simplest_failing_input_passed,
    } = shrink_steps.remove(
        shrink_steps
            .iter()
            .rposition(|step| !step.test_passed && !step.just_informational)
            .expect(
                "there should always be at least one failing non-informational step, element 0",
            ),
    );
    debug_assert!(!simplest_failing_input_passed && !simplest_failing_input_just_informational);

    Report {
        test_name: None,
        panic_info: None,
        passing_runs: failing_input_report.passing_runs,
        observations: shrinker.into_observations(),
        shrink_steps,
        simplest_failing_input,
    }
}

// TODO: add ability to customize how we sample the generator
// TODO: handle generator impls that lie about their sizes
pub fn run_test<T>(
    test: impl Fn(T) -> bool,
    generator: impl IntoInputGenerator<T>,
    rng: &mut impl Rng,
) -> Result<(), Box<Report<T>>> {
    let mut generator = generator.into_input_generator();

    // Find a failing input
    let Some(failing_input_report) = find_failing_input(&test, &mut generator, rng) else {
        return Ok(());
    };

    // Shrink the failing input and generate a report
    Err(Box::new(shrink_and_generate_report(
        &test,
        &generator,
        failing_input_report,
    )))
}
