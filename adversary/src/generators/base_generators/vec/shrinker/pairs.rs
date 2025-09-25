use crate::{
    RangeAwareValueGen, ValueGen,
    vec::shrinker::{Done, RemoveElems, ShrinkElements, Subsets, VecShrinker},
};

#[derive(Debug)]
pub struct Pairs<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> {
    value_gen: &'gens G,
    len_gen: &'gens L,                      // Invariant: 2 is in range for len_gen
    simplest_known_failing: Box<[G::Seed]>, // Invariant: 3 <= len <= MAX_LEN_BEFORE_SKIP
    first_index: usize, // Invariant: first_index < second_index < simplest_known_failing.len()
    second_index: usize, // Invariant: first_index < second_index < simplest_known_failing.len()
}

/// If the length is longer than this, we skip checking all pairs.
///
/// At length 20, we'll check up to 190 pairs.
const MAX_LEN_BEFORE_SKIP: usize = 20;

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> Pairs<'gens, G, L> {
    /// Returns a new [`Pairs`], or [`Err`] with `simplest_known_failing` if this step should be
    /// skipped.
    pub fn new(
        value_gen: &'gens G,
        len_gen: &'gens L,
        simplest_known_failing: Box<[G::Seed]>,
    ) -> Result<Self, Box<[G::Seed]>> {
        // If the seed is already length 0, 1, or 2, there's no point in this step
        if simplest_known_failing.len() < 2 {
            return Err(simplest_known_failing);
        }

        // If the seed is longer than the max length, there are too many pairs to check
        if simplest_known_failing.len() > MAX_LEN_BEFORE_SKIP {
            return Err(simplest_known_failing);
        }

        // If 2 is not a valid length, we have to skip this step
        if !len_gen.value_in_range(&2) {
            return Err(simplest_known_failing);
        }

        Ok(Self {
            value_gen,
            len_gen,
            simplest_known_failing,
            first_index: 0,
            second_index: 1,
        })
    }
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> Pairs<'gens, G, L> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        assert!(self.first_index < self.second_index);
        assert!(self.second_index < self.simplest_known_failing.len());

        let first = self.simplest_known_failing[self.first_index].clone();
        let second = self.simplest_known_failing[self.second_index].clone();

        Some(Box::new([first, second]))
    }

    pub fn update(mut self, current_attempt_passed: bool) -> VecShrinker<'gens, G, L> {
        // If the current attempt passed, update our indices and carry on
        if current_attempt_passed {
            self.second_index += 1;
            if self.second_index == self.simplest_known_failing.len() {
                self.first_index += 1;
                self.second_index = self.first_index + 1;

                // If we've exhausted all pairs, move on to the next step
                if self.second_index == self.simplest_known_failing.len() {
                    // Try `RemoveElems`
                    let simplest_known_failing = match RemoveElems::new(
                        self.value_gen,
                        self.len_gen,
                        self.simplest_known_failing,
                        true,
                    ) {
                        Ok(remove_elems) => return VecShrinker::RemoveElems(remove_elems),
                        Err(simplest_known_failing) => simplest_known_failing,
                    };

                    // `RemoveElems` had to be skipped, try `Subsets`
                    let simplest_known_failing = match Subsets::new(
                        self.value_gen,
                        self.len_gen,
                        simplest_known_failing,
                        true,
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
                    return VecShrinker::Done(Done::new());
                }
            }

            VecShrinker::Pairs(self)
        }
        // If the current attempt failed, we have a new simplest known failing
        else {
            // Skip to shrinking elements
            if let Ok(shrink_elements) =
                ShrinkElements::new(self.value_gen, self.len_gen, self.simplest_known_failing)
            {
                return VecShrinker::ShrinkElements(shrink_elements);
            }

            // `ShrinkElements` had to be skipped, we're done
            VecShrinker::Done(Done::new())
        }
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
