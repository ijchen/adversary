use crate::{
    RangeAwareValueGen, ValueGen,
    shrinker::Shrinker,
    vec::shrinker::{Done, Pairs, RemoveElems, ShrinkElements, SingleElems, Subsets, TryEmpty},
};

// TODO(ijchen): better documentation on the phases, including specifically on each phase's struct.
// See the unsigned integer shrinker for an example of how I want this documentation to look.

pub enum VecShrinker<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> {
    // Try an empty vec
    TryEmpty(TryEmpty<'gens, G, L>),

    // Try each element by itself in a single-element vec
    SingleElems(SingleElems<'gens, G, L>),

    // Try all pairs of two elements
    Pairs(Pairs<'gens, G, L>),

    // Try removing single elements at a time
    RemoveElems(RemoveElems<'gens, G, L>),

    // Try subsets of the vec
    Subsets(Subsets<'gens, G, L>),

    // Shrink each element
    ShrinkElements(ShrinkElements<'gens, G, L>),

    // No more progress to be made - we're done
    Done(Done<G, L>),
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> VecShrinker<'gens, G, L> {
    pub fn new(value_gen: &'gens G, len_gen: &'gens L, failing_value_seed: Box<[G::Seed]>) -> Self {
        // If the seed is already empty, no need to shrink further
        if failing_value_seed.is_empty() {
            return Self::Done(Done::new());
        }

        // Start with the `TryEmpty` step, if possible
        let failing_value_seed = match TryEmpty::new(value_gen, len_gen, failing_value_seed) {
            Ok(try_empty) => return Self::TryEmpty(try_empty),
            Err(simplest_known_failing) => simplest_known_failing,
        };

        // `TryEmpty` had to be skipped, try `SingleElems`
        let failing_value_seed = match SingleElems::new(value_gen, len_gen, failing_value_seed) {
            Ok(single_elems) => return Self::SingleElems(single_elems),
            Err(simplest_known_failing) => simplest_known_failing,
        };

        // `SingleElems` had to be skipped, try `Pairs`
        let failing_value_seed = match Pairs::new(value_gen, len_gen, failing_value_seed) {
            Ok(pairs) => return Self::Pairs(pairs),
            Err(simplest_known_failing) => simplest_known_failing,
        };

        // `Pairs` had to be skipped, try `RemoveElems`
        let failing_value_seed =
            match RemoveElems::new(value_gen, len_gen, failing_value_seed, false) {
                Ok(remove_elems) => return Self::RemoveElems(remove_elems),
                Err(simplest_known_failing) => simplest_known_failing,
            };

        // `RemoveElems` had to be skipped, try `Subsets`
        let failing_value_seed = match Subsets::new(value_gen, len_gen, failing_value_seed, false) {
            Ok(subsets) => return Self::Subsets(subsets),
            Err(simplest_known_failing) => simplest_known_failing,
        };

        // `Subsets` had to be skipped, try `ShrinkElements`
        if let Ok(shrink_elements) = ShrinkElements::new(value_gen, len_gen, failing_value_seed) {
            return Self::ShrinkElements(shrink_elements);
        }

        // `ShrinkElements` had to be skipped, we're done
        Self::Done(Done::new())
    }
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> Shrinker<Box<[G::Seed]>>
    for VecShrinker<'gens, G, L>
{
    fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        match self {
            VecShrinker::TryEmpty(step) => step.current_attempt(),
            VecShrinker::SingleElems(step) => step.current_attempt(),
            VecShrinker::Pairs(step) => step.current_attempt(),
            VecShrinker::RemoveElems(step) => step.current_attempt(),
            VecShrinker::Subsets(step) => step.current_attempt(),
            VecShrinker::ShrinkElements(step) => step.current_attempt(),
            VecShrinker::Done(step) => step.current_attempt(),
        }
    }

    fn update(&mut self, current_attempt_passed: bool) {
        *self = match std::mem::replace(self, VecShrinker::Done(Done::new())) {
            VecShrinker::TryEmpty(step) => step.update(current_attempt_passed),
            VecShrinker::SingleElems(step) => step.update(current_attempt_passed),
            VecShrinker::Pairs(step) => step.update(current_attempt_passed),
            VecShrinker::RemoveElems(step) => step.update(current_attempt_passed),
            VecShrinker::Subsets(step) => step.update(current_attempt_passed),
            VecShrinker::ShrinkElements(step) => step.update(current_attempt_passed),
            VecShrinker::Done(step) => step.update(current_attempt_passed),
        };
    }

    fn into_observations(self) -> Vec<crate::report::Observation> {
        match self {
            VecShrinker::TryEmpty(step) => step.into_observations(),
            VecShrinker::SingleElems(step) => step.into_observations(),
            VecShrinker::Pairs(step) => step.into_observations(),
            VecShrinker::RemoveElems(step) => step.into_observations(),
            VecShrinker::Subsets(step) => step.into_observations(),
            VecShrinker::ShrinkElements(step) => step.into_observations(),
            VecShrinker::Done(step) => step.into_observations(),
        }
    }
}
