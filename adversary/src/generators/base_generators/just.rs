use crate::{input_generator::NextAttempt, InputGenerator};

#[repr(transparent)]
struct JustWith<F>(F);

impl<T, F: Fn() -> T> InputGenerator for JustWith<F> {
    type Input = T;

    type InputSource = ();

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        Some(1)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        std::iter::once(())
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(1)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        std::iter::once(())
    }

    fn sample(&self, _rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
        ()
    }

    fn new_history(&self, _failing_input: Self::InputSource) -> Self::History {
        ()
    }

    fn current_simplest_failing(&self, _history: &Self::History) -> Self::InputSource {
        ()
    }

    fn next_input(
        &self,
        _rng: &mut impl rand::Rng,
        _history: &Self::History,
    ) -> NextAttempt<Self::InputSource> {
        NextAttempt::Done
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _shrinkable_input: Self::InputSource,
        _test_passed: bool,
    ) {
        ()
    }

    fn generate_observations(&self, _history: Self::History) -> Vec<crate::report::Observation> {
        vec![]
    }

    fn create_input(&self, _input_source: Self::InputSource) -> Self::Input {
        (self.0)()
    }
}

/// An [`InputGenerator`] that always produces clones of the same value and
/// never simplifies.
pub fn just<T: Clone>(value: T) -> impl InputGenerator<Input = T, InputSource = ()> {
    // TODO(ichen): write unit tests to ensure the compiler optimizes this
    // closure away
    JustWith(move || value.clone())
}

/// An [`InputGenerator`] that computes a value from the provided closure and
/// never simplifies.
pub fn just_with<T>(f: impl Fn() -> T) -> impl InputGenerator<Input = T, InputSource = ()> {
    JustWith(f)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn just_does_just_things() {
        let strategy = adv::just(35);

        assert_eq!(strategy.cardinality(), Some(1));
        assert!(strategy
            .exhaustive()
            .map(|input_source| strategy.create_input(input_source))
            .eq([35]));

        assert_eq!(strategy.adversarial_count(), Some(1));
        assert!(strategy
            .adversarial()
            .map(|input_source| strategy.create_input(input_source))
            .eq([35]));

        let mut rng = crate::rand::thread_rng();
        for _ in 0..100 {
            assert_eq!(strategy.create_input(strategy.sample(&mut rng)), 35);
        }

        let history = strategy.new_history(());
        assert!(matches!(
            strategy.next_input(&mut rng, &history),
            NextAttempt::Done
        ));
    }

    #[test]
    fn just_with_does_just_with_things() {
        #[derive(Debug, PartialEq, Eq)]
        struct NotClone<T>(pub T);

        let strategy = adv::just_with(|| NotClone("hi"));

        assert_eq!(strategy.cardinality(), Some(1));
        assert!(strategy
            .exhaustive()
            .map(|input_source| strategy.create_input(input_source))
            .eq([NotClone("hi")]));

        assert_eq!(strategy.adversarial_count(), Some(1));
        assert!(strategy
            .adversarial()
            .map(|input_source| strategy.create_input(input_source))
            .eq([NotClone("hi")]));

        let mut rng = crate::rand::thread_rng();
        for _ in 0..100 {
            assert_eq!(
                strategy.create_input(strategy.sample(&mut rng)),
                NotClone("hi")
            );
        }

        let history = strategy.new_history(());
        assert!(matches!(
            strategy.next_input(&mut rng, &history),
            NextAttempt::Done
        ));
    }
}
