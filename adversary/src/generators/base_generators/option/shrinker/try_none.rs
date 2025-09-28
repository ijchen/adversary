use crate::{
    ValueGen,
    generators::base_generators::option::shrinker::{Done, OptionShrinker, ShrinkSome},
};

#[derive(Debug)]
pub struct TryNone<'value_gen, G: ValueGen> {
    value_gen: &'value_gen G,
    simplest_known_failing: G::Seed,
}

impl<'value_gen, G: ValueGen> TryNone<'value_gen, G> {
    /// Returns a new [`TryNone`].
    pub fn new(value_gen: &'value_gen G, simplest_known_failing: G::Seed) -> Self {
        Self {
            value_gen,
            simplest_known_failing,
        }
    }
}

impl<'value_gen, G: ValueGen> TryNone<'value_gen, G> {
    pub fn current_attempt(&self) -> Option<Option<G::Seed>> {
        Some(None)
    }

    pub fn update(self, current_attempt_passed: bool) -> OptionShrinker<'value_gen, G> {
        // If the attempt failed, we're done shrinking
        if !current_attempt_passed {
            return OptionShrinker::Done(Done::new());
        }

        // The attempt passed, continue on to the next step

        // Start with the `ShrinkSome` step, if possible
        if let Some(shrink_some) = ShrinkSome::new(self.value_gen, self.simplest_known_failing) {
            return OptionShrinker::ShrinkSome(shrink_some);
        }

        // `ShrinkSome` had to be skipped, we're done
        OptionShrinker::Done(Done::new())
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
