use rand::{Rng, rngs::ThreadRng};

use crate::{
    RangeAwareValueGen, ValueGen,
    vec::shrinker::{Done, Pairs, ShrinkElements, VecShrinker},
};

const MAX_ATTEMPTS_SINCE_PROGRESS: u32 = 100;

#[derive(Debug)]
pub struct Subsets<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> {
    value_gen: &'gens G,
    len_gen: &'gens L,
    simplest_known_failing: Box<[G::Seed]>, // Invariant: length is greater than 2
    ran_pairs_step: bool,
    min_subset_size: usize,
    max_subset_size: usize,
    current: Box<[G::Seed]>,
    attempts_since_progress: u32,
    made_progress: bool,
    // TODO(ijchen): provide an rng as argument to ValueGen::new_shrinker and Shrinker::update. Runs
    // into trouble naively passing &mut (impl Rng + ?Sized), so more consideration is needed.
    // That's probably also a good time to consider whether or not we want rand as a public
    // dependency, and the 0.8.5/0.9 issue.
    rng: ThreadRng,
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> Subsets<'gens, G, L> {
    // NOTE: returns Err if this step should be skipped
    pub fn new(
        value_gen: &'gens G,
        len_gen: &'gens L,
        simplest_known_failing: Box<[G::Seed]>,
        ran_pairs_step: bool,
    ) -> Result<Self, Box<[G::Seed]>> {
        // Subsets of length 0 are checked by TryEmpty, length 1 by SingleElems, and length 2 by
        // Pairs (if that step ran)
        let mut min_subset_size = if ran_pairs_step { 3 } else { 2 };
        // Subsets of length `simplest_known_failing.len() - 1` are checked by RemoveElems
        let Some(mut max_subset_size) = simplest_known_failing.len().checked_sub(2) else {
            // let Some(mut max_subset_size) = simplest_known_failing.len().checked_sub(1) else {
            return Err(simplest_known_failing);
        };

        // TODO(ijchen): consider whether or not it's a concern that this might take a really long
        // time to terminate due to large runs of invalid lengths (ex. length 1000000000000 is the
        // first valid length from 0)
        // Bump up min_subset_size until it's valid for len_gen (or we run out of sizes)
        while min_subset_size <= max_subset_size && !len_gen.value_in_range(&min_subset_size) {
            min_subset_size += 1;
        }
        // Bump down max_subset_size until it's valid for len_gen (or we run out of sizes)
        while min_subset_size <= max_subset_size && !len_gen.value_in_range(&max_subset_size) {
            max_subset_size -= 1;
        }

        // If there are no valid subset sizes, we have to skip this step
        if (min_subset_size..=max_subset_size).is_empty() {
            return Err(simplest_known_failing);
        }

        let mut rng = rand::thread_rng();

        let current = Self::new_subset(
            &simplest_known_failing,
            min_subset_size,
            max_subset_size,
            len_gen,
            &mut rng,
        );

        Ok(Self {
            value_gen,
            len_gen,
            simplest_known_failing,
            ran_pairs_step,
            min_subset_size,
            max_subset_size,
            current,
            attempts_since_progress: 0,
            made_progress: false,
            rng,
        })
    }

    fn new_subset(
        simplest_known_failing: &[G::Seed],
        min_subset_size: usize,
        max_subset_size: usize,
        len_gen: &L,
        rng: &mut ThreadRng,
    ) -> Box<[G::Seed]> {
        // TODO(ijchen): handle situations where this takes a long time to terminate due to sparse
        // valid lengths (ex. when 3 is valid, 999999999 is valid, but nothing in between is valid)
        let len = loop {
            let len = rng.gen_range(min_subset_size..=max_subset_size);
            if len_gen.value_in_range(&len) {
                break len;
            }
        };

        // Not using .choose_multiple(..) directly since we want to preserve order
        let keep_indices =
            rand::seq::index::sample(rng, simplest_known_failing.len(), len).into_vec();

        simplest_known_failing
            .iter()
            .enumerate()
            .filter(|(i, _)| keep_indices.contains(i))
            .map(|(_, elem)| elem.clone())
            .collect()
    }

    fn new_with_progress(
        value_gen: &'gens G,
        len_gen: &'gens L,
        simplest_known_failing: Box<[G::Seed]>,
        ran_pairs_step: bool,
    ) -> Result<Self, Box<[G::Seed]>> {
        let mut subsets = Self::new(value_gen, len_gen, simplest_known_failing, ran_pairs_step)?;
        subsets.made_progress = true;
        Ok(subsets)
    }
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> Subsets<'gens, G, L> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        (self.attempts_since_progress < MAX_ATTEMPTS_SINCE_PROGRESS).then(|| self.current.clone())
    }

    pub fn update(mut self, current_attempt_passed: bool) -> VecShrinker<'gens, G, L> {
        if current_attempt_passed {
            let new_current = Self::new_subset(
                &self.simplest_known_failing,
                self.min_subset_size,
                self.max_subset_size,
                self.len_gen,
                &mut self.rng,
            );
            let new_attempts_since_progress = self.attempts_since_progress + 1;

            // If we haven't yet hit the max number of attempts since progress, keep trying subsets
            if new_attempts_since_progress < MAX_ATTEMPTS_SINCE_PROGRESS {
                VecShrinker::Subsets(Self {
                    current: new_current,
                    attempts_since_progress: new_attempts_since_progress,
                    ..self
                })
            }
            // We've hit the max number of attempts since progress - move on to the next step
            else {
                // If we didn't try pairs but are able to now, go back and try pairs
                let simplest_known_failing = if self.made_progress {
                    match Pairs::new(self.value_gen, self.len_gen, self.simplest_known_failing) {
                        Ok(pairs) => return VecShrinker::Pairs(pairs),
                        Err(simplest_known_failing) => simplest_known_failing,
                    }
                } else {
                    self.simplest_known_failing
                };

                // Try `ShrinkElements`
                if let Ok(shrink_elements) =
                    ShrinkElements::new(self.value_gen, self.len_gen, simplest_known_failing)
                {
                    return VecShrinker::ShrinkElements(shrink_elements);
                }

                // `ShrinkElements` had to be skipped, we're done
                VecShrinker::Done(Done::new())
            }
        }
        // If the current attempt failed, we've made progress!
        else {
            // We might be able to shrink even further with subsets - especially now that the space
            // of possible subsets is reduced
            let simplest_known_failing = match Self::new_with_progress(
                self.value_gen,
                self.len_gen,
                self.current,
                self.ran_pairs_step,
            ) {
                Ok(subsets) => return VecShrinker::Subsets(subsets),
                Err(simplest_known_failing) => simplest_known_failing,
            };

            // If we didn't try pairs but are able to now, go back and try pairs
            let simplest_known_failing = if !self.ran_pairs_step {
                match Pairs::new(self.value_gen, self.len_gen, simplest_known_failing) {
                    Ok(pairs) => return VecShrinker::Pairs(pairs),
                    Err(simplest_known_failing) => simplest_known_failing,
                }
            } else {
                simplest_known_failing
            };

            // `Try `ShrinkElements`
            if let Ok(shrink_elements) =
                ShrinkElements::new(self.value_gen, self.len_gen, simplest_known_failing)
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
