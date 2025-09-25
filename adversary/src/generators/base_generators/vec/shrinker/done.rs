use std::marker::PhantomData;

use crate::{RangeAwareValueGen, ValueGen, vec::shrinker::VecShrinker};

#[derive(Debug, Default)]
pub struct Done<G, L> {
    _phantom: PhantomData<fn(G, L)>,
}

impl<G: ValueGen, L: RangeAwareValueGen<Value = usize>> Done<G, L> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        None
    }

    pub fn update<'a>(self, _current_attempt_passed: bool) -> VecShrinker<'a, G, L> {
        panic!(
            "Done::update called (indicates VecShrinker::update was called while done shrinking)"
        );
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        // TODO: observations
        vec![]
    }
}
