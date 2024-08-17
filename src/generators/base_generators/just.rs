use crate::{
    input_generator::{InputWithShrinkable, NextAttempt},
    InputGenerator,
};

struct JustWith<F>(F);

impl<T, F: Fn() -> T> InputGenerator for JustWith<F> {
    type Input = T;

    type ShrinkableInput = ();

    type History = ();

    fn cardinality(&self) -> Option<usize> {
        Some(1)
    }

    fn exhaustive(
        &self,
    ) -> impl Iterator<
        Item = crate::input_generator::InputWithShrinkable<Self::Input, Self::ShrinkableInput>,
    > {
        std::iter::once(InputWithShrinkable((self.0)(), ()))
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(1)
    }

    fn adversarial(
        &self,
    ) -> impl Iterator<
        Item = crate::input_generator::InputWithShrinkable<Self::Input, Self::ShrinkableInput>,
    > {
        std::iter::once(InputWithShrinkable((self.0)(), ()))
    }

    fn sample(
        &self,
        _rng: &mut (impl rand::Rng + ?Sized),
    ) -> crate::input_generator::InputWithShrinkable<Self::Input, Self::ShrinkableInput> {
        InputWithShrinkable((self.0)(), ())
    }

    fn new_history(&self) -> Self::History {
        ()
    }

    fn next_input(
        &self,
        _rng: &mut impl rand::Rng,
        _history: &Self::History,
    ) -> crate::input_generator::NextAttempt<Self::Input, Self::ShrinkableInput> {
        NextAttempt::Done
    }

    fn update_history(
        &self,
        _history: &mut Self::History,
        _shrinkable_input: Self::ShrinkableInput,
        _test_passed: bool,
    ) {
        ()
    }

    fn generate_observations(&self, _history: Self::History) -> Vec<crate::report::Observation> {
        vec![]
    }
}

/// An [`InputGenerator`] that always produces clones of the same value and
/// never simplifies.
pub fn just<T: Clone>(value: T) -> impl InputGenerator<Input = T> {
    // TODO(ichen): make sure the compiler optimizes this closure away
    JustWith(move || value.clone())
}

/// An [`InputGenerator`] that computes a value from the provided closure and
/// never simplifies.
pub fn just_with<T>(f: impl Fn() -> T) -> impl InputGenerator<Input = T> {
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
            .map(|InputWithShrinkable(v, _)| v)
            .eq([35]));

        assert_eq!(strategy.adversarial_count(), Some(1));
        assert!(strategy
            .adversarial()
            .map(|InputWithShrinkable(v, _)| v)
            .eq([35]));

        let mut rng = crate::rand::thread_rng();
        for _ in 0..100 {
            assert_eq!(strategy.sample(&mut rng).0, 35);
        }

        let history = strategy.new_history();
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
            .map(|InputWithShrinkable(v, _)| v)
            .eq([NotClone("hi")]));

        assert_eq!(strategy.adversarial_count(), Some(1));
        assert!(strategy
            .adversarial()
            .map(|InputWithShrinkable(v, _)| v)
            .eq([NotClone("hi")]));

        let mut rng = crate::rand::thread_rng();
        for _ in 0..100 {
            assert_eq!(strategy.sample(&mut rng).0, NotClone("hi"));
        }

        let history = strategy.new_history();
        assert!(matches!(
            strategy.next_input(&mut rng, &history),
            NextAttempt::Done
        ));
    }
}
