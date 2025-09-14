use crate::{
    ValueGen,
    shrinker::Shrinker,
    vec::shrinker::{
        done::Done, pairs::Pairs, remove_elems::RemoveElems, shrink_elements::ShrinkElements,
        single_elems::SingleElems, subsets::Subsets, try_empty::TryEmpty,
    },
};

// TODO phases:
// 1. Shrink the size
//   i. Try simple zero length
//     - If failing, we're done
//   ii. Try single-element vec for each elem
//     - If failing, jump to shrinking element
//   iii. If vec length is less than some threshold, try all pairs of elems (in
//        same order)
//   iv. Try removing single elements at a time (go through each elem and remove
//       it)
//   v. Try subsets of various lengths (TODO: how to determine lengths? How to
//      select elements?)
// 2. Shrink the elements
//   i. Go through each element and shrink it
// 3. Repeat until no progress is made (NOTE: don't jump to size shrink if we're
//    already at zero or one (or maybe even two?) length)

pub enum VecShrinker<'value_gen, G: ValueGen> {
    // Try an empty vec
    TryEmpty(TryEmpty<'value_gen, G>),

    // Try each element by itself in a single-element vec
    SingleElems(SingleElems<'value_gen, G>),

    // Try all pairs of two elements
    Pairs(Pairs<'value_gen, G>),

    // Try removing single elements at a time
    RemoveElems(RemoveElems<'value_gen, G>),

    // Try subsets of the vec
    Subsets(Subsets<'value_gen, G>),

    // Shrink each element
    ShrinkElements(ShrinkElements<'value_gen, G>),

    // No more progress to be made - we're done
    Done(Done<G>),
}

impl<'value_gen, G: ValueGen> VecShrinker<'value_gen, G> {
    pub fn new(value_gen: &'value_gen G, failing_value_seed: Box<[G::Seed]>) -> Self {
        // If the seed is already empty, no need to shrink further
        if failing_value_seed.is_empty() {
            return Self::Done(Done::new());
        }

        // If the seed only has one inner seed, jump straight to shrinking the element
        if failing_value_seed.len() == 1 {
            return Self::ShrinkElements(ShrinkElements::new(value_gen, failing_value_seed));
        }

        // Otherwise, start in the `TryEmpty` step
        Self::TryEmpty(TryEmpty::new(value_gen, failing_value_seed))
    }
}

impl<'value_gen, G: ValueGen> Shrinker<Box<[G::Seed]>> for VecShrinker<'value_gen, G> {
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
