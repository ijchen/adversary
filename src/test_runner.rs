use std::panic::RefUnwindSafe;

use rand::Rng;

use crate::{input_generator::NextAttempt, report::ShrinkStep, InputGenerator, Report};

struct FailingInputReport<T> {
    pub failing_input: T,
    pub passing_runs: u64,
}

fn find_failing_input<T>(
    test: &impl Fn(&T) -> bool,
    generator: &mut impl InputGenerator<Input = T>,
    rng: &mut impl Rng,
) -> Option<FailingInputReport<T>> {
    #[inline]
    fn helper<T>(
        test: &impl Fn(&T) -> bool,
        inputs: impl Iterator<Item = T>,
    ) -> Option<FailingInputReport<T>> {
        // TODO: maybe make this all functional and appease the lambda bros
        let mut passing_runs: u64 = 0;
        for input in inputs {
            let test_passed = test(&input);

            if !test_passed {
                return Some(FailingInputReport {
                    failing_input: input,
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
        helper(&test, generator.exhaustive())
    } else if generator
        .adversarial_count()
        .is_some_and(|adversarial_count| adversarial_count <= MAX_RUNS)
    {
        helper(
            &test,
            generator
                .adversarial()
                .chain(std::iter::repeat_with(|| generator.sample(rng)))
                .take(MAX_RUNS),
        )
    } else {
        helper(
            &test,
            std::iter::repeat_with(|| generator.sample(rng)).take(MAX_RUNS),
        )
    }
}

#[allow(unused)] // TODO(ichen): some things are unused bc of refactoring
fn shrink_and_generate_report<T>(
    test: &impl Fn(&T) -> bool,
    generator: &impl InputGenerator<Input = T>,
    rng: &mut impl Rng,
    failing_input_report: FailingInputReport<T>,
) -> Report<T> {
    let mut history = generator.new_history();

    generator.update_history(&mut history, &failing_input_report.failing_input, false);

    let mut shrink_steps = vec![ShrinkStep::new(
        failing_input_report.failing_input,
        false,
        false,
    )];
    // TODO(ichen): limit how many times this loop can run (to guard against
    // faulty InputGenerator::next_input impls)
    loop {
        let (input, info_gathering) = match generator.next_input(rng, &history) {
            NextAttempt::Done => break,
            NextAttempt::InfoGathering(input) => (input, true),
            NextAttempt::ShrinkAttempt(input) => (input, false),
        };

        let test_passed = test(&input);

        generator.update_history(&mut history, &input, test_passed);

        shrink_steps.push(ShrinkStep::new(input, info_gathering, test_passed))
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
        panic_message: None,
        passing_runs: failing_input_report.passing_runs,
        observations: generator.generate_observations(history),
        shrink_steps,
        simplest_failing_input,
    }
}

// TODO: add ability to customize how we sample the generator
// TODO: handle generator impls that lie about their sizes
pub fn run_test<T>(
    test: impl Fn(&T) -> bool,
    mut generator: impl InputGenerator<Input = T>,
    rng: &mut impl Rng,
) -> Result<(), Report<T>> {
    // Find a failing input
    let Some(failing_input_report) = find_failing_input(&test, &mut generator, rng) else {
        return Ok(());
    };

    // Shrink the failing input and generate a report
    Err(shrink_and_generate_report(
        &test,
        &generator,
        rng,
        failing_input_report,
    ))
}

pub fn run_test_panics<T: RefUnwindSafe>(
    test: impl Fn(&T) + RefUnwindSafe,
    generator: impl InputGenerator<Input = T>,
    rng: &mut impl Rng,
) -> Result<(), Report<T>> {
    run_test(
        |value| std::panic::catch_unwind(|| test(value)).is_ok(),
        generator,
        rng,
    )
}
