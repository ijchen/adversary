use crate::{
    RangeAwareValueGen, ValueGen,
    vec::shrinker::{Done, Pairs, RemoveElems, ShrinkElements, Subsets, VecShrinker},
};

#[derive(Debug)]
pub struct SingleElems<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> {
    value_gen: &'gens G,
    len_gen: &'gens L,                      // Invariant: 1 is in range for len_gen
    simplest_known_failing: Box<[G::Seed]>, // Invariant: length >= 2
    index: usize, // index == simplest_known_failing.len() indicates we're done shrinking
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> SingleElems<'gens, G, L> {
    /// Returns a new [`SingleElems`], or [`Err`] with `simplest_known_failing` if this step should
    /// be skipped.
    pub fn new(
        value_gen: &'gens G,
        len_gen: &'gens L,
        simplest_known_failing: Box<[G::Seed]>,
    ) -> Result<Self, Box<[G::Seed]>> {
        // If 1 is not a valid length, we have to skip this step
        if !len_gen.value_in_range(&1) {
            return Err(simplest_known_failing);
        }

        // If the seed is already length 0 or 1, there's no point in this step
        if simplest_known_failing.len() < 2 {
            return Err(simplest_known_failing);
        }

        Ok(Self {
            value_gen,
            len_gen,
            simplest_known_failing,
            index: 0,
        })
    }
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> SingleElems<'gens, G, L> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        self.simplest_known_failing
            .get(self.index)
            .map(|seed| [seed.clone()].into())
    }

    pub fn update(self, current_attempt_passed: bool) -> VecShrinker<'gens, G, L> {
        // If the attempt failed, we're done shrinking size - try to shrink the
        // element
        if !current_attempt_passed {
            // Try `ShrinkElements`
            if let Ok(shrink_elements) = ShrinkElements::new(
                self.value_gen,
                self.len_gen,
                self.current_attempt()
                    .expect("SingleElems::update called while done shrinking"),
            ) {
                return VecShrinker::ShrinkElements(shrink_elements);
            }

            // `ShrinkElements` had to be skipped, we're done
            return VecShrinker::Done(Done::new());
        }

        // If this was the last element, move on to the next step
        let next_index = self.index + 1;
        if next_index >= self.simplest_known_failing.len() {
            // Try `Pairs`
            let simplest_known_failing =
                match Pairs::new(self.value_gen, self.len_gen, self.simplest_known_failing) {
                    Ok(pairs) => return VecShrinker::Pairs(pairs),
                    Err(simplest_known_failing) => simplest_known_failing,
                };

            // `Pairs` had to be skipped, try `RemoveElems`
            let simplest_known_failing =
                match RemoveElems::new(self.value_gen, self.len_gen, simplest_known_failing, false)
                {
                    Ok(remove_elems) => return VecShrinker::RemoveElems(remove_elems),
                    Err(simplest_known_failing) => simplest_known_failing,
                };

            // `RemoveElems` had to be skipped, try `Subsets`
            let simplest_known_failing =
                match Subsets::new(self.value_gen, self.len_gen, simplest_known_failing, false) {
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

        // Move on to the next element
        VecShrinker::SingleElems(Self {
            value_gen: self.value_gen,
            len_gen: self.len_gen,
            simplest_known_failing: self.simplest_known_failing,
            index: next_index,
        })
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
