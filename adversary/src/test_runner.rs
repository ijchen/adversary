use std::any::Any;
use std::cell::RefCell;
use std::panic::{RefUnwindSafe, UnwindSafe};

use crate::input_generator::InputWithShrinkable;
use crate::rand::Rng;

use crate::IntoInputGenerator;
use crate::{input_generator::NextAttempt, report::ShrinkStep, InputGenerator, Report};

struct FailingInputReport<T, I> {
    pub failing_input: T,
    pub failing_shrinkable_input: I,
    pub passing_runs: u64,
}

fn find_failing_input<T, I>(
    test: &impl Fn(T) -> bool,
    generator: &mut impl InputGenerator<Input = T, InputSource = I>,
    rng: &mut impl Rng,
) -> Option<FailingInputReport<T, I>> {
    // #[inline]
    // fn helper<T, I>(
    //     test: &impl Fn(T) -> bool,
    //     generator: &mut impl InputGenerator<Input = T, InputSource = I>,
    //     inputs: impl Iterator<Item = InputWithShrinkable<T, I>>,
    // ) -> Option<FailingInputReport<T, I>> {
    //     // TODO: maybe make this all functional and appease the lambda bros
    //     let mut passing_runs: u64 = 0;
    //     for InputWithShrinkable(input, shrinkable_input) in inputs {
    //         let test_passed = test(input);

    //         if !test_passed {
    //             return Some(FailingInputReport {
    //                 failing_input: generator.create_input(&shrinkable_input),
    //                 failing_shrinkable_input: shrinkable_input,
    //                 passing_runs,
    //             });
    //         }

    //         // TODO(ichen): maybe possibly consider handling overflow better
    //         // than saturating (although, FWIW, at 50 billion inputs per second,
    //         // it would take over 11 years to reach u64::MAX)
    //         passing_runs = passing_runs.saturating_add(1);
    //     }

    //     None
    // }

    // TODO: allow customizing this
    const MAX_RUNS: usize = 1_000_000;

    // // TODO(ichen): consider the cost of triple-monomorphization here, and
    // // possible alternatives.
    // if generator
    //     .cardinality()
    //     .is_some_and(|cardinality| cardinality <= MAX_RUNS)
    // {
    //     helper(&test, generator, generator.exhaustive())
    // } else if generator
    //     .adversarial_count()
    //     .is_some_and(|adversarial_count| adversarial_count <= MAX_RUNS)
    // {
    //     helper(
    //         &test,
    //         generator,
    //         generator
    //             .adversarial()
    //             .chain(std::iter::repeat_with(|| generator.sample(rng)))
    //             .take(MAX_RUNS),
    //     )
    // } else {
    //     helper(
    //         &test,
    //         generator,
    //         std::iter::repeat_with(|| generator.sample(rng)).take(MAX_RUNS),
    //     )
    // }

    if generator
        .cardinality()
        .is_some_and(|cardinality| cardinality <= MAX_RUNS)
    {
        let mut passing_runs: u64 = 0;
        for InputWithShrinkable(input, shrinkable_input) in generator.exhaustive() {
            let test_passed = test(input);

            if !test_passed {
                return Some(FailingInputReport {
                    failing_input: generator.create_input(&shrinkable_input),
                    failing_shrinkable_input: shrinkable_input,
                    passing_runs,
                });
            }

            // TODO(ichen): maybe possibly consider handling overflow better
            // than saturating (although, FWIW, at 50 billion inputs per second,
            // it would take over 11 years to reach u64::MAX)
            passing_runs = passing_runs.saturating_add(1);
        }

        None
    } else if generator
        .adversarial_count()
        .is_some_and(|adversarial_count| adversarial_count <= MAX_RUNS)
    {
        let mut passing_runs: u64 = 0;
        for InputWithShrinkable(input, shrinkable_input) in generator
            .adversarial()
            .chain(std::iter::repeat_with(|| generator.sample(rng)))
            .take(MAX_RUNS)
        {
            let test_passed = test(input);

            if !test_passed {
                return Some(FailingInputReport {
                    failing_input: generator.create_input(&shrinkable_input),
                    failing_shrinkable_input: shrinkable_input,
                    passing_runs,
                });
            }

            // TODO(ichen): maybe possibly consider handling overflow better
            // than saturating (although, FWIW, at 50 billion inputs per second,
            // it would take over 11 years to reach u64::MAX)
            passing_runs = passing_runs.saturating_add(1);
        }

        None
    } else {
        let mut passing_runs: u64 = 0;
        for InputWithShrinkable(input, shrinkable_input) in
            std::iter::repeat_with(|| generator.sample(rng)).take(MAX_RUNS)
        {
            let test_passed = test(input);

            if !test_passed {
                return Some(FailingInputReport {
                    failing_input: generator.create_input(&shrinkable_input),
                    failing_shrinkable_input: shrinkable_input,
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
}

fn shrink_and_generate_report<T, I>(
    test: &impl Fn(T) -> bool,
    generator: &impl InputGenerator<Input = T, InputSource = I>,
    rng: &mut impl Rng,
    failing_input_report: FailingInputReport<T, I>,
) -> Report<T> {
    let mut history = generator.new_history();

    generator.update_history(
        &mut history,
        failing_input_report.failing_shrinkable_input,
        false,
    );

    let mut shrink_steps = vec![ShrinkStep::new(
        failing_input_report.failing_input,
        false,
        false,
    )];
    // TODO(ichen): limit how many times this loop can run (to guard against
    // faulty InputGenerator::next_input impls)
    loop {
        let (InputWithShrinkable(input, shrinkable_input), info_gathering) =
            match generator.next_input(rng, &history) {
                NextAttempt::Done => break,
                NextAttempt::InfoGathering(input) => (input, true),
                NextAttempt::ShrinkAttempt(input) => (input, false),
            };

        let test_passed = test(input);

        let shrink_step = ShrinkStep::new(
            generator.create_input(&shrinkable_input),
            info_gathering,
            test_passed,
        );

        generator.update_history(&mut history, shrinkable_input, test_passed);

        shrink_steps.push(shrink_step);
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
    test: impl Fn(T) -> bool,
    generator: impl IntoInputGenerator<T>,
    rng: &mut impl Rng,
) -> Result<(), Report<T>> {
    let mut generator = generator.into_input_generator();

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

fn extract_panic_message(any: Box<dyn Any + Send + 'static>) -> Option<String> {
    // Try downcasting to a &str first
    let any = match any.downcast::<&str>() {
        Ok(s) => return Some(s.to_string()),
        Err(any) => any,
    };

    // Downcasting to a &str failed, try downcasting to a String
    any.downcast::<String>().ok().map(|s| *s)
}

pub fn run_test_panics<T: UnwindSafe>(
    test: impl Fn(T) + RefUnwindSafe,
    generator: impl IntoInputGenerator<T>,
    rng: &mut impl Rng,
) -> Result<(), Report<T>> {
    // TODO(ichen): see if we can avoid interior mutability hack here
    let panic_message = RefCell::new(None);

    run_test(
        |value| match std::panic::catch_unwind(|| test(value)) {
            Ok(()) => true,
            Err(e) => {
                *panic_message.borrow_mut() = extract_panic_message(e);
                false
            }
        },
        generator,
        rng,
    )
    .map_err(|mut report| {
        report.panic_message = panic_message.into_inner();

        report
    })
}
