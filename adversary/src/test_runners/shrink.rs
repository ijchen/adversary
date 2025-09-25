use crate::{
    ValueGen,
    report::{FailureCause, Importance, Observation, ShrinkStep, TestOutcome},
    shrinker::Shrinker as _,
};

use super::test::Test;

pub fn shrink<T, S: Clone>(
    test: &impl Test<T>,
    generator: &impl ValueGen<Value = T, Seed = S>,
    failing_value_seed: S,
    cause: FailureCause,
    max_shrink_steps: usize,
) -> (Vec<Observation>, Vec<ShrinkStep<T>>) {
    let mut shrinker = generator.new_shrinker(failing_value_seed.clone());

    let mut shrink_steps = vec![ShrinkStep::new(
        generator.create_value(failing_value_seed),
        false,
        TestOutcome::Failed { cause },
    )];
    while shrink_steps.len() - 1 < max_shrink_steps {
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

    let mut observations = shrinker.into_observations();
    if shrink_steps.len() - 1 == max_shrink_steps {
        observations.push(Observation::new(
            "Shrinking might have been able to make more progress, but was ended early after reaching the maximum number of shrinking steps.".to_string(),
            Importance::MaybeRelevant,
        ));
    }

    (observations, shrink_steps)
}
