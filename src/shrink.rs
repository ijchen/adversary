use rand::Rng;

pub trait Shrink<T: Clone> {
    type History;

    fn history_from_failure(&self, failing_input: T) -> Self::History;

    fn update_history(&self, history: &mut Self::History, input: T, test_passed: bool);

    /// TODO: better docs
    /// This API is especially likely to change as I figure out a decent way to
    /// do this.
    fn next_input(&self, rng: &mut impl Rng, history: &Self::History) -> Option<T>;
}
