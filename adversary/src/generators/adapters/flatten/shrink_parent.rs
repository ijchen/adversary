use crate::{IntoValueGen, ValueGen, sample::sample, shrinker::Shrinker};

pub struct ShrinkParent<'parent, P: ValueGen, C: ValueGen> {
    simplest_known_failing: (P::Seed, C::Seed),
    parent: &'parent P,
    parent_shrinker: P::Shrinker<'parent>,
    // It's not strictly necessary to store the parent seed (since we could just
    // call `.current_attempt(..)` on `parent_shrinker`), but this guards
    // against impure implementations of `current_attempt`.
    parent_seed: P::Seed,
    child: C,
    // NOTE: stored in reverse order (next to try at the back) for efficiently
    // removing seeds without shifting every other seed down.
    child_seeds: Vec<C::Seed>,
}

impl<'parent, P: ValueGen, C: ValueGen> ShrinkParent<'parent, P, C>
where
    P::Value: IntoValueGen<C::Value, Gen = C>,
{
    pub fn new(
        parent: &'parent P,
        simplest_known_failing: (P::Seed, C::Seed),
    ) -> Result<Self, (P::Seed, C::Seed)> {
        let mut parent_shrinker = parent.new_shrinker(simplest_known_failing.0.clone());

        let Some((parent_seed, child, child_seeds)) =
            Self::find_child_with_seeds(parent, &mut parent_shrinker)
        else {
            return Err(simplest_known_failing);
        };

        Ok(Self {
            simplest_known_failing,
            parent,
            parent_shrinker,
            parent_seed,
            child,
            child_seeds,
        })
    }

    pub fn current_attempt(&self) -> Option<(P::Seed, C::Seed)> {
        Some((
            self.parent_seed.clone(),
            self.child_seeds.last().unwrap().clone(),
        ))
    }

    // Returns whether or not this ShrinkParent step is done and cannot make
    // further progress.
    pub fn update(&mut self, current_attempt_passed: bool) -> bool {
        // Split into two separate function for organization
        match current_attempt_passed {
            true => self.update_passed(),
            false => self.update_failed(),
        }
    }

    pub fn parent_and_simplest_known_failing(&self) -> (&'parent P, (P::Seed, C::Seed)) {
        (self.parent, self.simplest_known_failing.clone())
    }

    fn find_child_with_seeds(
        parent: &P,
        parent_shrinker: &mut P::Shrinker<'_>,
    ) -> Option<(P::Seed, C, Vec<C::Seed>)> {
        // Repeatedly construct a child ValueGen from the parent shrinker's
        // current attempt, until either we find a child with at least one seed
        // to try, or the parent shrinker can't make progress any more
        loop {
            // If the parent shrinker can't make progress, we're done here
            let parent_seed = parent_shrinker.current_attempt()?;

            // Create the child ValueGen from the parent shrinker's current
            // attempt
            let child = parent.create_value(parent_seed.clone()).into_value_gen();
            let mut child_seeds = get_child_seeds(&child);

            // If there are seeds we can try for this child shrinker, we can
            // stop looking for a child
            if !child_seeds.is_empty() {
                // NOTE: child seeds are stored in reverse order (next to try at
                // the back) for efficiently removing seeds without shifting
                // every other seed down.
                child_seeds.reverse();

                return Some((parent_seed, child, child_seeds));
            }

            // This child doesn't have any seeds to try - update the parent
            // shrinker and try another child
            parent_shrinker.update(true);
        }
    }

    /// Implementation detail of [`Self::update`].
    fn update_passed(&mut self) -> bool {
        // The test didn't fail - try the next child seed
        self.child_seeds.pop();

        // If there are no more seeds for this child, update the parent shrinker
        // and move on to the next child (or the next phase)
        if self.child_seeds.is_empty() {
            self.parent_shrinker.update(true);

            if let Some((parent_seed, child, child_seeds)) =
                Self::find_child_with_seeds(self.parent, &mut self.parent_shrinker)
            {
                self.parent_seed = parent_seed;
                self.child = child;
                self.child_seeds = child_seeds;
            } else {
                // The parent shrinker can't make any more progress - move on to
                // shrinking the child
                return true;
            }
        }

        false
    }

    /// Implementation detail of [`Self::update`].
    fn update_failed(&mut self) -> bool {
        // The test failed! Update our simplest known failing
        self.simplest_known_failing = (self.parent_seed.clone(), self.child_seeds.pop().unwrap());

        // Update the parent shrinker
        self.parent_shrinker.update(false);

        // Move on to the next child (or return true to move to the next step)
        if let Some((parent_seed, child, child_seeds)) =
            Self::find_child_with_seeds(self.parent, &mut self.parent_shrinker)
        {
            self.parent_seed = parent_seed;
            self.child = child;
            self.child_seeds = child_seeds;

            false
        } else {
            // The parent shrinker can't make any more progress - move on to
            // shrinking the child
            true
        }
    }
}

fn get_child_seeds<G: ValueGen>(value_gen: &G) -> Vec<G::Seed> {
    // TODO(ichen): tweak/optimize parameters, deal with combinatorial explosion
    // with chained flattens, and consider not using the thread RNG.
    sample(value_gen, 1000, 500, &mut rand::thread_rng()).collect()
}
