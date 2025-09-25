use crate::{
    RangeAwareValueGen, ValueGen,
    vec::shrinker::{Done, Pairs, ShrinkElements, Subsets, VecShrinker},
};

#[derive(Debug)]
pub struct RemoveElems<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> {
    value_gen: &'gens G,
    len_gen: &'gens L, // Invariant: `simplest_known_failing.len() - 1` is in range for len_gen
    simplest_known_failing: Box<[G::Seed]>, // Invariant: len >= 3
    ran_pairs_step: bool, // Whether or not we ran the Pairs step
    index: usize,      // index == simplest_known_failing.len() indicates we're done shrinking
    made_progress: bool, // Whether or not we've made progress in removing elements since index == 0
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> RemoveElems<'gens, G, L> {
    pub fn new(
        value_gen: &'gens G,
        len_gen: &'gens L,
        simplest_known_failing: Box<[G::Seed]>,
        ran_pairs_step: bool,
    ) -> Result<Self, Box<[G::Seed]>> {
        // If the seed is already length 0, 1, or 2, there's no point in this step
        if simplest_known_failing.len() < 3 {
            return Err(simplest_known_failing);
        }

        // If we ran the Pairs step and the seed is length 3, there's no point in this step, because
        // we'd just re-check pairs we've already checked
        if ran_pairs_step && simplest_known_failing.len() == 3 {
            return Err(simplest_known_failing);
        }

        // If `simplest_known_failing.len() - 1` is not a valid length, we have to skip this step
        if !len_gen.value_in_range(&(simplest_known_failing.len() - 1)) {
            return Err(simplest_known_failing);
        }

        Ok(Self {
            value_gen,
            len_gen,
            simplest_known_failing,
            ran_pairs_step,
            index: 0,
            made_progress: false,
        })
    }
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> RemoveElems<'gens, G, L> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        Some(
            self.simplest_known_failing
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != self.index)
                .map(|(_, elem)| elem.clone())
                .collect(),
        )
    }

    pub fn update(mut self, current_attempt_passed: bool) -> VecShrinker<'gens, G, L> {
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
            // If we've removed some elements this iteration, we want to loop back around to the
            // start of the vec and continue trying to remove elements (in case removing a later
            // element made it possible to remove an earlier element)
            let simplest_known_failing = if self.made_progress {
                match Self::new(
                    self.value_gen,
                    self.len_gen,
                    self.simplest_known_failing,
                    self.ran_pairs_step,
                ) {
                    Ok(remove_elems) => return VecShrinker::RemoveElems(remove_elems),
                    Err(simplest_known_failing) => simplest_known_failing,
                }
            } else {
                self.simplest_known_failing
            };

            // Removing single elements is done, move on to the next step

            // If we didn't try pairs but are able to now, go back and try pairs
            let simplest_known_failing = if !self.ran_pairs_step {
                match Pairs::new(self.value_gen, self.len_gen, simplest_known_failing) {
                    Ok(pairs) => return VecShrinker::Pairs(pairs),
                    Err(simplest_known_failing) => simplest_known_failing,
                }
            } else {
                simplest_known_failing
            };

            // Try `Subsets`
            let simplest_known_failing = match Subsets::new(
                self.value_gen,
                self.len_gen,
                simplest_known_failing,
                self.ran_pairs_step,
            ) {
                Ok(subsets) => return VecShrinker::Subsets(subsets),
                Err(simplest_known_failing) => simplest_known_failing,
            };

            // `Subsets` had to be skipped, try `ShrinkElements`
            if let Ok(shrink_elements) =
                ShrinkElements::new(self.value_gen, self.len_gen, simplest_known_failing)
            {
                return VecShrinker::ShrinkElements(shrink_elements);
            }

            // `ShrinkElements` had to be skipped, we're done
            VecShrinker::Done(Done::new())
        }
        // We have more elements to try removing - keep going
        else {
            VecShrinker::RemoveElems(self)
        }
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
