use pair::Pair;

use crate::{IntoValueGen, ValueGen, shrinker::Shrinker};

use super::shrinker_owner::ShrinkerOwner;

pub struct ShrinkChild<P: ValueGen, C: ValueGen> {
    parent_seed: P::Seed,
    child_shrinker: Pair<ShrinkerOwner<C>>,
}

impl<P: ValueGen, C: ValueGen> ShrinkChild<P, C>
where
    P::Value: IntoValueGen<C::Value, Gen = C>,
{
    pub fn new(parent: &P, simplest_known_failing: (P::Seed, C::Seed)) -> Self {
        Self {
            parent_seed: simplest_known_failing.0.clone(),
            child_shrinker: Pair::new_with_context(
                ShrinkerOwner(
                    parent
                        .create_value(simplest_known_failing.0)
                        .into_value_gen(),
                ),
                simplest_known_failing.1,
            ),
        }
    }

    pub fn current_attempt(&self) -> Option<(P::Seed, C::Seed)> {
        self.child_shrinker.with_dependent(|child_shrinker| {
            child_shrinker
                .current_attempt()
                .map(|child_seed| (self.parent_seed.clone(), child_seed))
        })
    }

    pub fn update(&mut self, current_attempt_passed: bool) {
        self.child_shrinker
            .with_dependent_mut(|child_shrinker| child_shrinker.update(current_attempt_passed))
    }
}
