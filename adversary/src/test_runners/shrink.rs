use crate::{
    ValueGen,
    report::{FailureCause, Observation, ShrinkStep, TestOutcome},
    shrinker::Shrinker as _,
};

use super::test::Test;

pub fn shrink<T, S: Clone>(
    test: &impl Test<T>,
    generator: &impl ValueGen<Value = T, Seed = S>,
    failing_value_seed: S,
    cause: FailureCause,
) -> (Vec<Observation>, Vec<ShrinkStep<T>>) {
    let mut shrinker = generator.new_shrinker(failing_value_seed.clone());

    let mut shrink_steps = vec![ShrinkStep::new(
        generator.create_value(failing_value_seed),
        false,
        TestOutcome::Failed { cause },
    )];
    // TODO(ichen): limit how many times this loop can run (to guard against
    // faulty Shrinker impls)
    loop {
        // TODO: allow info-gathering attempts
        let Some(seed) = shrinker.current_attempt() else {
            break;
        };

        let outcome = test.test(generator.create_value(seed.clone()));

        shrinker.update(outcome.passed());

        shrink_steps.push(ShrinkStep::new(
            generator.create_value(seed),
            false,
            outcome,
        ));
    }

    (shrinker.into_observations(), shrink_steps)
}
