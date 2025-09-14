use crate::{
    ValueGen,
    vec::shrinker::{VecShrinker, shrink_elements::ShrinkElements, subsets::Subsets},
};

#[derive(Debug)]
pub struct RemoveElems<'value_gen, G: ValueGen> {
    value_gen: &'value_gen G,
    simplest_known_failing: Box<[G::Seed]>,
    index: usize,
    made_progress: bool,
}

impl<'value_gen, G: ValueGen> RemoveElems<'value_gen, G> {
    pub fn new(value_gen: &'value_gen G, simplest_known_failing: Box<[G::Seed]>) -> Self {
        Self {
            value_gen,
            simplest_known_failing,
            index: 0, // index == simplest_known_failing.len() indicates we're done shrinking
            made_progress: false,
        }
    }
}

impl<'value_gen, G: ValueGen> RemoveElems<'value_gen, G> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        Some(
            self.simplest_known_failing
                .iter()
                .enumerate()
                .filter_map(|(i, elem)| (i != self.index).then(|| elem.clone()))
                .collect(),
        )
    }

    pub fn update(mut self, current_attempt_passed: bool) -> VecShrinker<'value_gen, G> {
        // If the current attempt passed, update the index and carry on
        if current_attempt_passed {
            self.index += 1;
        }
        // If the current attempt failed, we have a new simplest known failing
        else {
            self.made_progress = true;
            self.simplest_known_failing = self
                .current_attempt()
                .expect("RemoveElems::update called while done shrinking");

            // No need to update index, since we just removed the element previously at self.index.
            // We do have to check and handle if that was the last element, and we're now out of
            // bounds. Fortunately, this logic is already handled below.
        }

        // If that was the last element, move on to the next step
        if self.index == self.simplest_known_failing.len() {
            // If we've removed some elements this iteration, keep trying to remove elements
            // until we can't anymore
            return if self.made_progress {
                VecShrinker::RemoveElems(Self::new(self.value_gen, self.simplest_known_failing))
            }
            // No elements could be removed, move on to the next step
            else {
                // TODO(ijchen): this clone can be avoided
                Subsets::new(self.value_gen, self.simplest_known_failing.clone())
                    .map(VecShrinker::Subsets)
                    .unwrap_or_else(move || {
                        VecShrinker::ShrinkElements(ShrinkElements::new(
                            self.value_gen,
                            self.simplest_known_failing,
                        ))
                    })
            };
        }

        // We have more elements to try removing - keep going
        VecShrinker::RemoveElems(self)
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
