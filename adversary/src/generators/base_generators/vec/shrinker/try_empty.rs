use crate::{
    RangeAwareValueGen, ValueGen,
    vec::shrinker::{Done, Pairs, RemoveElems, ShrinkElements, SingleElems, Subsets, VecShrinker},
};

#[derive(Debug)]
pub struct TryEmpty<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> {
    value_gen: &'gens G,
    len_gen: &'gens L,                      // Invariant: 0 is in range for len_gen
    simplest_known_failing: Box<[G::Seed]>, // Invariant: simplest_known_failing is not empty
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> TryEmpty<'gens, G, L> {
    /// Returns a new [`TryEmpty`], or [`Err`] with `simplest_known_failing` if this step should be
    /// skipped.
    pub fn new(
        value_gen: &'gens G,
        len_gen: &'gens L,
        simplest_known_failing: Box<[G::Seed]>,
    ) -> Result<Self, Box<[G::Seed]>> {
        // If 0 is not a valid length, we have to skip this step
        if !len_gen.value_in_range(&0) {
            return Err(simplest_known_failing);
        }

        // If the seed is already empty, there's no point in this step
        if simplest_known_failing.is_empty() {
            return Err(simplest_known_failing);
        }

        Ok(Self {
            value_gen,
            len_gen,
            simplest_known_failing,
        })
    }
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> TryEmpty<'gens, G, L> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        Some(Box::new([]))
    }

    pub fn update(self, current_attempt_passed: bool) -> VecShrinker<'gens, G, L> {
        // If the attempt failed, we're done shrinking
        if !current_attempt_passed {
            return VecShrinker::Done(Done::new());
        }

        // The attempt passed, continue on to the next step

        // Try `SingleElems`
        let failing_value_seed =
            match SingleElems::new(self.value_gen, self.len_gen, self.simplest_known_failing) {
                Ok(single_elems) => return VecShrinker::SingleElems(single_elems),
                Err(simplest_known_failing) => simplest_known_failing,
            };

        // `SingleElems` had to be skipped, try `Pairs`
        let failing_value_seed = match Pairs::new(self.value_gen, self.len_gen, failing_value_seed)
        {
            Ok(pairs) => return VecShrinker::Pairs(pairs),
            Err(simplest_known_failing) => simplest_known_failing,
        };

        // `Pairs` had to be skipped, try `RemoveElems`
        let failing_value_seed =
            match RemoveElems::new(self.value_gen, self.len_gen, failing_value_seed, false) {
                Ok(remove_elems) => return VecShrinker::RemoveElems(remove_elems),
                Err(simplest_known_failing) => simplest_known_failing,
            };

        // `RemoveElems` had to be skipped, try `Subsets`
        let failing_value_seed =
            match Subsets::new(self.value_gen, self.len_gen, failing_value_seed, false) {
                Ok(subsets) => return VecShrinker::Subsets(subsets),
                Err(simplest_known_failing) => simplest_known_failing,
            };

        // `Subsets` had to be skipped, try `ShrinkElements`
        if let Ok(shrink_elements) =
            ShrinkElements::new(self.value_gen, self.len_gen, failing_value_seed)
        {
            return VecShrinker::ShrinkElements(shrink_elements);
        }

        // `ShrinkElements` had to be skipped, we're done
        VecShrinker::Done(Done::new())

        // // The attempt passed, continue on to single element attempts
        // VecShrinker::SingleElems(SingleElems::new(
        //     self.value_gen,
        //     self.simplest_known_failing,
        // ))
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
