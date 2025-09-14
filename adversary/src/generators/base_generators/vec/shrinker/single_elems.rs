use crate::{
    ValueGen,
    vec::shrinker::{VecShrinker, pairs::Pairs, shrink_elements::ShrinkElements},
};

#[derive(Debug)]
pub struct SingleElems<'value_gen, G: ValueGen> {
    value_gen: &'value_gen G,
    simplest_known_failing: Box<[G::Seed]>,
    index: usize, // index == simplest_known_failing.len() indicates we're done shrinking
}

impl<'value_gen, G: ValueGen> SingleElems<'value_gen, G> {
    pub fn new(value_gen: &'value_gen G, simplest_known_failing: Box<[G::Seed]>) -> Self {
        Self {
            value_gen,
            simplest_known_failing,
            index: 0,
        }
    }
}

impl<'value_gen, G: ValueGen> SingleElems<'value_gen, G> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        self.simplest_known_failing
            .get(self.index)
            .map(|seed| [seed.clone()].into())
    }

    pub fn update(self, current_attempt_passed: bool) -> VecShrinker<'value_gen, G> {
        // If the attempt failed, we're done shrinking size - try to shrink the
        // element
        if !current_attempt_passed {
            return VecShrinker::ShrinkElements(ShrinkElements::new(
                self.value_gen,
                self.current_attempt()
                    .expect("SingleElems::update called while done shrinking"),
            ));
        }

        // If this was the last element, move on to pairs
        let next_index = self.index + 1;
        if next_index >= self.simplest_known_failing.len() {
            return VecShrinker::Pairs(Pairs::new(self.value_gen, self.simplest_known_failing));
        }

        // Move on to the next element
        VecShrinker::SingleElems(Self {
            value_gen: self.value_gen,
            simplest_known_failing: self.simplest_known_failing,
            index: next_index,
        })
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
