use crate::{
    Chance, ValueGen,
    generators::base_generators::option::shrinker::{Done, ShrinkSome, TryNone},
    report::Observation,
    shrinker::Shrinker,
};

// TODO(ijchen): better documentation on the phases, including specifically on each phase's struct.
// See the unsigned integer shrinker for an example of how I want this documentation to look.

pub enum OptionShrinker<'value_gen, G: ValueGen> {
    // Try None
    TryNone(TryNone<'value_gen, G>),

    // Try Some values from the inner shrinker
    ShrinkSome(ShrinkSome<'value_gen, G>),

    // No more progress to be made - we're done
    Done(Done<G>),
}

impl<'value_gen, G: ValueGen> OptionShrinker<'value_gen, G> {
    pub fn new(
        value_gen: &'value_gen G,
        some_chance: Chance,
        failing_value_seed: Option<G::Seed>,
    ) -> Self {
        match some_chance {
            // If Some is impossible, there's nothing to shrink (failing_value_seed should be None)
            Chance::IMPOSSIBLE => Self::Done(Done::new()),

            // If Some is guaranteed, skip TryNone
            Chance::GUARANTEED => {
                // If Some is guaranteed, failing_value_seed should be Some
                let Some(failing_value_seed) = failing_value_seed else {
                    // If it's not, something weird has happened and we can't meaningfully shrink
                    return Self::Done(Done::new());
                };

                // Start with the `ShrinkSome` step, if possible
                if let Some(shrink_some) = ShrinkSome::new(value_gen, failing_value_seed) {
                    return Self::ShrinkSome(shrink_some);
                }

                // `ShrinkSome` had to be skipped, we're done
                Self::Done(Done::new())
            }

            // Both Some and None are possible - try both (starting with None)
            _ => {
                // If `failing_value_seed` is already None, we have no work to do
                let Some(failing_value_seed) = failing_value_seed else {
                    return Self::Done(Done::new());
                };

                // Start with the `TryNone` step
                Self::TryNone(TryNone::new(value_gen, failing_value_seed))
            }
        }
    }
}

impl<'value_gen, G: ValueGen> Shrinker<Option<G::Seed>> for OptionShrinker<'value_gen, G> {
    fn current_attempt(&self) -> Option<Option<G::Seed>> {
        match self {
            Self::TryNone(step) => step.current_attempt(),
            Self::ShrinkSome(step) => step.current_attempt(),
            Self::Done(step) => step.current_attempt(),
        }
    }

    fn update(&mut self, current_attempt_passed: bool) {
        *self = match std::mem::replace(self, Self::Done(Done::new())) {
            Self::TryNone(step) => step.update(current_attempt_passed),
            Self::ShrinkSome(step) => step.update(current_attempt_passed),
            Self::Done(step) => step.update(current_attempt_passed),
        };
    }

    fn into_observations(self) -> Vec<Observation> {
        match self {
            Self::TryNone(step) => step.into_observations(),
            Self::ShrinkSome(step) => step.into_observations(),
            Self::Done(step) => step.into_observations(),
        }
    }
}
