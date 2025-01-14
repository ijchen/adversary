use std::marker::PhantomData;

use crate::{report::Observation, shrinker::Shrinker};

/// A shrinker for any type, which never performs any shrinking.
///
/// [`Shrinker::current_attempt`] always returns [`None`], [`Shrinker::update`]
/// always panics, and [`Shrinker::into_observations`] always returns an empty
/// [`Vec`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NeverShrink<T> {
    phantom_t: PhantomData<T>,
}

// Manual Default impl instead of #[derive(...)]'d because the derive macro adds
// an overly restrictive `T: Default` bound (we don't care if `T: Default`)
impl<T> Default for NeverShrink<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> NeverShrink<T> {
    pub fn new() -> Self {
        Self {
            phantom_t: PhantomData,
        }
    }
}

impl<T> Shrinker for NeverShrink<T> {
    type Seed = T;

    fn current_attempt(&self) -> Option<Self::Seed> {
        None
    }

    fn update(&mut self, _current_attempt_passed: bool) {
        panic!("`Shrinker::update` called on `NeverShrink`");
    }

    fn into_observations(self) -> Vec<Observation> {
        Vec::new()
    }
}
