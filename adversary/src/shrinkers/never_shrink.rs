use crate::{report::Observation, shrinker::Shrinker};

/// A shrinker for any type, which never performs any shrinking.
///
/// [`Shrinker::current_attempt`] always returns [`None`], [`Shrinker::update`]
/// always panics, and [`Shrinker::into_observations`] always returns an empty
/// [`Vec`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NeverShrink;

impl NeverShrink {
    pub fn new() -> Self {
        Self
    }
}

impl<T> Shrinker<T> for NeverShrink {
    fn current_attempt(&self) -> Option<T> {
        None
    }

    fn update(&mut self, _current_attempt_passed: bool) {
        panic!("`Shrinker::update` called on `NeverShrink`");
    }

    fn into_observations(self) -> Vec<Observation> {
        Vec::new()
    }
}
