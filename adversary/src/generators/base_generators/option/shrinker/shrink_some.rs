use crate::{
    ValueGen,
    generators::base_generators::option::shrinker::{Done, OptionShrinker},
    report::Observation,
    shrinker::Shrinker,
};

pub struct ShrinkSome<'value_gen, G: ValueGen + 'value_gen> {
    // Invariant: the shrinker is not done
    shrinker: G::Shrinker<'value_gen>,
}

impl<'value_gen, G: ValueGen + 'value_gen> ShrinkSome<'value_gen, G> {
    /// Returns a new [`ShrinkSome`], or [`None`] if this step should be skipped.
    pub fn new(value_gen: &'value_gen G, simplest_known_failing: G::Seed) -> Option<Self> {
        let shrinker = value_gen.new_shrinker(simplest_known_failing);

        // If the inner shrinker can't make progress, this step should be skipped
        if shrinker.current_attempt().is_none() {
            return None;
        }

        Some(Self { shrinker })
    }

    pub fn current_attempt(&self) -> Option<Option<G::Seed>> {
        self.shrinker.current_attempt().map(Some)
    }

    pub fn update(mut self, current_attempt_passed: bool) -> OptionShrinker<'value_gen, G> {
        self.shrinker.update(current_attempt_passed);

        // If the inner shrinker is done, this step is done
        if self.shrinker.current_attempt().is_none() {
            return OptionShrinker::Done(Done::new());
        }

        OptionShrinker::ShrinkSome(self)
    }

    pub fn into_observations(self) -> Vec<Observation> {
        self.shrinker.into_observations()
    }
}
