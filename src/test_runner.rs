use std::panic::RefUnwindSafe;

use rand::Rng;

use crate::{input_generator::NextAttempt, InputGenerator, Report};

fn find_failing_input<T>(
    test: &impl Fn(&T) -> bool,
    generator: &mut impl InputGenerator<Input = T>,
    rng: &mut impl Rng,
) -> Option<T> {
    // TODO: allow customizing this
    const MAX_RUNS: usize = 1_000_000;

    #[inline]
    fn helper<T>(test: &impl Fn(&T) -> bool, mut inputs: impl Iterator<Item = T>) -> Option<T> {
        inputs.find(|input| !test(input))
    }

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
    failing_input: T,
) -> Report<T> {
    let mut history = generator.new_history();
    generator.update_history(&mut history, &failing_input, false);
    let mut minimal_failing_input = None;
    while let NextAttempt::InfoGathering(next_input) | NextAttempt::ShrinkAttempt(next_input) =
        generator.next_input(rng, &history)
    {
        let passed = test(&next_input);

        generator.update_history(&mut history, &next_input, passed);

        if !passed {
            minimal_failing_input = Some(next_input);
        }
    }

    // Report {
    //     original_failing_input: failing_input,
    //     shrunk_failing_input: minimal_failing_input,
    //     details: generator.generate_report(history),
    // }
    todo!()
}

// TODO: add ability to customize how we sample the generator
// TODO: handle generator impls that lie about their sizes
pub fn run_test<T>(
    test: impl Fn(&T) -> bool,
    mut generator: impl InputGenerator<Input = T>,
    rng: &mut impl Rng,
) -> Result<(), Report<T>> {
    // Find a failing input
    let Some(failing_input) = find_failing_input(&test, &mut generator, rng) else {
        return Ok(());
    };

    // Shrink the failing input and generate a report
    Err(shrink_and_generate_report(
        &test,
        &generator,
        rng,
        failing_input,
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
