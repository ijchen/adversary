use rand::{Rng, rngs::ThreadRng};

use crate::{
    ValueGen,
    vec::shrinker::{SingleElems, VecShrinker, done::Done, shrink_elements::ShrinkElements},
};

const MAX_ATTEMPTS_SINCE_PROGRESS: u32 = 100;

// TODO(ijchen): skip length 2 subsets if we didn't skip the all pairs step
#[derive(Debug)]
pub struct Subsets<'value_gen, G: ValueGen> {
    value_gen: &'value_gen G,
    simplest_known_failing: Box<[G::Seed]>, // Invariant: length is greater than 2
    current: Box<[G::Seed]>,
    attempts_since_progress: u32,
    // TODO(ijchen): provide an rng as argument to ValueGen::new_shrinker and Shrinker::update. Runs
    // into trouble naively passing &mut (impl Rng + ?Sized), so more consideration is needed.
    // That's probably also a good time to consider whether or not we want rand as a public
    // dependency, and the 0.8.5/0.9 issue.
    rng: ThreadRng,
}

impl<'value_gen, G: ValueGen> Subsets<'value_gen, G> {
    // NOTE: returns None if this step should be skipped
    pub fn new(value_gen: &'value_gen G, simplest_known_failing: Box<[G::Seed]>) -> Option<Self> {
        // There's no point trying subsets of length 0 or 1 - both are checked exhaustively by
        // earlier dedicated steps
        if simplest_known_failing.len() <= 2 {
            return None;
        }

        let mut rng = rand::thread_rng();

        let current = Self::new_subset(&simplest_known_failing, &mut rng);

        Some(Self {
            value_gen,
            simplest_known_failing,
            current,
            attempts_since_progress: 0,
            rng,
        })
    }

    fn new_subset(simplest_known_failing: &[G::Seed], rng: &mut ThreadRng) -> Box<[G::Seed]> {
        // NOTE(ijchen): skip subsets of length 0 and 1 - those are both are checked exhaustively by
        // earlier dedicated steps
        let len = rng.gen_range(2..simplest_known_failing.len());

        // Not using .choose_multiple(..) directly since we want to preserve order
        let keep_indices =
            rand::seq::index::sample(rng, simplest_known_failing.len(), len).into_vec();

        simplest_known_failing
            .iter()
            .enumerate()
            .filter_map(|(i, elem)| keep_indices.contains(&i).then(|| elem.clone()))
            .collect()
    }
}

impl<'value_gen, G: ValueGen> Subsets<'value_gen, G> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        (self.attempts_since_progress < MAX_ATTEMPTS_SINCE_PROGRESS).then(|| self.current.clone())
    }

    pub fn update(mut self, current_attempt_passed: bool) -> VecShrinker<'value_gen, G> {
        if current_attempt_passed {
            self.attempts_since_progress += 1;
            VecShrinker::Subsets(self)
        }
        // If the current attempt failed, we've made progress!
        else {
            // If we found a subset of length 3 or less, we're done shrinking length
            if self.current.len() <= 3 {
                return VecShrinker::ShrinkElements(ShrinkElements::new(
                    self.value_gen,
                    self.current,
                ));
            }

            let new_subset = Self::new_subset(&self.current, &mut self.rng);

            self.attempts_since_progress = 0;
            self.simplest_known_failing = std::mem::replace(&mut self.current, new_subset);

            VecShrinker::Subsets(self)
        }
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
