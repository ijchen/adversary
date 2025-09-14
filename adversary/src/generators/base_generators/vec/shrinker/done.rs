use std::marker::PhantomData;

use crate::{ValueGen, vec::shrinker::VecShrinker};

#[derive(Debug)]
pub struct Done<G> {
    _phantom: PhantomData<fn(G)>,
}

impl<G: ValueGen> Done<G> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        None
    }

    pub fn update<'a>(self, _current_attempt_passed: bool) -> VecShrinker<'a, G> {
        panic!(
            "Done::update called (indicates VecShrinker::update was called while done shrinking)"
        );
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        // TODO: observations
        vec![]
    }
}
