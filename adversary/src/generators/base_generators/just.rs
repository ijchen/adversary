use crate::{ValueGen, report::Observation, shrinker::Shrinker};

#[repr(transparent)]
struct JustWith<F>(F);

impl<T, F: Fn() -> T> ValueGen for JustWith<F> {
    type Value = T;

    type Seed = ();

    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = JustShrinker
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        Some(1)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        std::iter::once(())
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(1)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        std::iter::once(())
    }

    #[expect(
        clippy::unused_unit,
        reason = "literally returning a unit value - made explicit for clarity"
    )]
    fn sample(&self, _rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        ()
    }

    fn new_shrinker(&self, (): Self::Seed) -> Self::Shrinker<'_> {
        JustShrinker
    }

    fn create_value(&self, (): Self::Seed) -> Self::Value {
        (self.0)()
    }
}

struct JustShrinker;

impl<T> Shrinker<T> for JustShrinker {
    fn current_attempt(&self) -> Option<T> {
        None
    }

    fn update(&mut self, _current_attempt_passed: bool) {}

    fn into_observations(self) -> Vec<Observation> {
        Vec::new()
    }
}

/// A [`ValueGen`] that always produces clones of the same value and never
/// shrinks.
pub fn just<T: Clone>(value: T) -> impl ValueGen<Value = T, Seed = ()> {
    // TODO(ichen): write unit tests to ensure the compiler optimizes this
    // closure away
    JustWith(move || value.clone())
}

/// A [`ValueGen`] that computes a value from the provided closure and never
/// shrinks.
pub fn just_with<T>(f: impl Fn() -> T) -> impl ValueGen<Value = T, Seed = ()> {
    JustWith(f)
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, shrinker::Shrinker};

    #[test]
    fn just_does_just_things() {
        let strategy = adv::just(35);

        assert_eq!(strategy.cardinality(), Some(1));
        assert!(
            strategy
                .exhaustive()
                .map(|seed| strategy.create_value(seed))
                .eq([35])
        );

        assert_eq!(strategy.adversarial_count(), Some(1));
        assert!(
            strategy
                .adversarial()
                .map(|seed| strategy.create_value(seed))
                .eq([35])
        );

        let mut rng = crate::rand::thread_rng();
        for _ in 0..100 {
            assert_eq!(strategy.create_value(strategy.sample(&mut rng)), 35);
        }

        let shrinker = strategy.new_shrinker(());
        assert!(shrinker.current_attempt().is_none());
    }

    #[test]
    fn just_with_does_just_with_things() {
        #[derive(Debug, PartialEq, Eq)]
        struct NotClone<T>(pub T);

        let strategy = adv::just_with(|| NotClone("hi"));

        assert_eq!(strategy.cardinality(), Some(1));
        assert!(
            strategy
                .exhaustive()
                .map(|seed| strategy.create_value(seed))
                .eq([NotClone("hi")])
        );

        assert_eq!(strategy.adversarial_count(), Some(1));
        assert!(
            strategy
                .adversarial()
                .map(|seed| strategy.create_value(seed))
                .eq([NotClone("hi")])
        );

        let mut rng = crate::rand::thread_rng();
        for _ in 0..100 {
            assert_eq!(
                strategy.create_value(strategy.sample(&mut rng)),
                NotClone("hi")
            );
        }

        let shrinker = strategy.new_shrinker(());
        assert!(shrinker.current_attempt().is_none());
    }
}
