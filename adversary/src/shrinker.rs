use crate::{report::Observation, ValueGen};

pub trait Shrinker<Gen: ValueGen + ?Sized> {
    /// TODO
    ///
    /// NOTE: make it clear that implementors must ensure any value returned in
    /// Some(...) is simpler than any previous attempt that passed
    fn current_attempt(&self) -> Option<Gen::Seed>;

    /// TODO
    ///
    /// NOTE: make it clear implementors may panic if the current attempt is
    /// None
    fn update(&mut self, generator: &Gen, current_attempt_passed: bool);

    /// TODO
    fn into_observations(self) -> Vec<Observation>;
}
