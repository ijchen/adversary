use std::marker::PhantomData;

use crate::{
    ValueGen, generators::base_generators::option::shrinker::OptionShrinker, report::Observation,
};

#[derive(Debug, Default)]
pub struct Done<G> {
    _phantom: PhantomData<fn(G)>,
}

impl<G: ValueGen> Done<G> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn current_attempt(&self) -> Option<Option<G::Seed>> {
        None
    }

    pub fn update<'a>(self, _current_attempt_passed: bool) -> OptionShrinker<'a, G> {
        panic!(
            "Done::update called (indicates OptionShrinker::update was called while done shrinking)"
        );
    }

    pub fn into_observations(self) -> Vec<Observation> {
        // TODO: observations
        vec![]
    }
}
