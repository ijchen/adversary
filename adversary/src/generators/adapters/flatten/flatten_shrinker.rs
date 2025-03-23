use crate::{IntoValueGen, ValueGen, report::Observation, shrinker::Shrinker};

use super::{shrink_child::ShrinkChild, shrink_parent::ShrinkParent};

pub enum FlattenShrinker<'parent, P: ValueGen, C: ValueGen> {
    ShrinkParent(ShrinkParent<'parent, P, C>),
    ShrinkChild(ShrinkChild<P, C>),
}

impl<'parent, P: ValueGen, C: ValueGen> FlattenShrinker<'parent, P, C>
where
    P::Value: IntoValueGen<C::Value, Gen = C>,
{
    pub fn new(simplest_known_failing: (P::Seed, C::Seed), parent: &'parent P) -> Self {
        match ShrinkParent::new(parent, simplest_known_failing) {
            Ok(shrink_parent) => Self::ShrinkParent(shrink_parent),
            Err(simplest_known_failing) => {
                Self::ShrinkChild(ShrinkChild::new(parent, simplest_known_failing))
            }
        }
    }
}

impl<'parent, P: ValueGen, C: ValueGen> Shrinker<(P::Seed, C::Seed)>
    for FlattenShrinker<'parent, P, C>
where
    P::Value: IntoValueGen<C::Value, Gen = C>,
{
    fn current_attempt(&self) -> Option<(P::Seed, C::Seed)> {
        match self {
            FlattenShrinker::ShrinkParent(shrink_parent) => shrink_parent.current_attempt(),
            FlattenShrinker::ShrinkChild(shrink_child) => shrink_child.current_attempt(),
        }
    }

    fn update(&mut self, current_attempt_passed: bool) {
        match self {
            FlattenShrinker::ShrinkParent(shrink_parent) => {
                if shrink_parent.update(current_attempt_passed) {
                    let (parent, simplest_known_failing) =
                        shrink_parent.parent_and_simplest_known_failing();

                    *self = FlattenShrinker::ShrinkChild(ShrinkChild::new(
                        parent,
                        simplest_known_failing,
                    ))
                }
            }
            FlattenShrinker::ShrinkChild(shrink_child) => {
                shrink_child.update(current_attempt_passed);
            }
        }
    }

    fn into_observations(self) -> Vec<Observation> {
        // TODO(ichen): useful observations
        Vec::new()
    }
}
