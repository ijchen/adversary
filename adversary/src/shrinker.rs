use crate::report::Observation;

// TODO: make Shrinker generic over Seed instead of having an associated type
// (to allow for things like NeverShrink to not need a PhantomData<T>)
pub trait Shrinker {
    type Seed;

    /// TODO
    ///
    /// NOTE: make it clear that implementors must ensure any value returned in
    /// Some(...) is simpler than any previous attempt that passed
    fn current_attempt(&self) -> Option<Self::Seed>;

    /// TODO
    ///
    /// NOTE: make it clear implementors may panic if the current attempt is
    /// None
    fn update(&mut self, current_attempt_passed: bool);

    /// TODO
    fn into_observations(self) -> Vec<Observation>;
}
