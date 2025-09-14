use crate::{
    ValueGen,
    vec::shrinker::{SingleElems, VecShrinker, done::Done},
};

#[derive(Debug)]
pub struct TryEmpty<'value_gen, G: ValueGen> {
    value_gen: &'value_gen G,
    simplest_known_failing: Box<[G::Seed]>,
}

impl<'value_gen, G: ValueGen> TryEmpty<'value_gen, G> {
    pub fn new(value_gen: &'value_gen G, simplest_known_failing: Box<[G::Seed]>) -> Self {
        Self {
            value_gen,
            simplest_known_failing,
        }
    }
}

impl<'value_gen, G: ValueGen> TryEmpty<'value_gen, G> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        Some(Box::new([]))
    }

    pub fn update(self, current_attempt_passed: bool) -> VecShrinker<'value_gen, G> {
        // If the attempt failed, we're done shrinking
        if !current_attempt_passed {
            return VecShrinker::Done(Done::new());
        }

        // The attempt passed, continue on to single element attempts
        VecShrinker::SingleElems(SingleElems::new(
            self.value_gen,
            self.simplest_known_failing,
        ))
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
