use crate::{
    ValueGen,
    vec::shrinker::{VecShrinker, remove_elems::RemoveElems, shrink_elements::ShrinkElements},
};

#[derive(Debug)]
pub struct Pairs<'value_gen, G: ValueGen> {
    value_gen: &'value_gen G,
    simplest_known_failing: Box<[G::Seed]>,
    first_index: usize, // Invariant: first_index < second_index < simplest_known_failing.len()
    second_index: usize, // Invariant: first_index < second_index < simplest_known_failing.len()
}

/// If the length is longer than this, we skip checking all pairs.
///
/// At length 20, we'll check up to 190 pairs.
pub const MAX_LEN_BEFORE_SKIP: usize = 20;

impl<'value_gen, G: ValueGen> Pairs<'value_gen, G> {
    pub fn new(value_gen: &'value_gen G, simplest_known_failing: Box<[G::Seed]>) -> Self {
        // TODO: document panic conditions
        assert!((2..=MAX_LEN_BEFORE_SKIP).contains(&simplest_known_failing.len()));

        Self {
            value_gen,
            simplest_known_failing,
            first_index: 0,
            second_index: 1,
        }
    }
}

impl<'value_gen, G: ValueGen> Pairs<'value_gen, G> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        assert!(self.first_index < self.second_index);
        assert!(self.second_index < self.simplest_known_failing.len());

        let first = self.simplest_known_failing[self.first_index].clone();
        let second = self.simplest_known_failing[self.second_index].clone();

        Some(Box::new([first, second]))
    }

    pub fn update(mut self, current_attempt_passed: bool) -> VecShrinker<'value_gen, G> {
        // If the current attempt passed, update our indices and carry on
        if current_attempt_passed {
            self.second_index += 1;
            if self.second_index == self.simplest_known_failing.len() {
                self.first_index += 1;
                self.second_index = self.first_index + 1;

                // If we've exhausted all pairs, move on to the next step
                if self.second_index == self.simplest_known_failing.len() {
                    return VecShrinker::RemoveElems(RemoveElems::new(
                        self.value_gen,
                        self.simplest_known_failing,
                    ));
                }
            }

            VecShrinker::Pairs(self)
        }
        // If the current attempt failed, we have a new simplest known failing
        else {
            // Skip to shrinking elements
            VecShrinker::ShrinkElements(ShrinkElements::new(
                self.value_gen,
                self.current_attempt()
                    .expect("Pairs::update called while done"),
            ))
        }
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
