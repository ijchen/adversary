use std::marker::PhantomData;

use crate::{IntoValueGen, ValueGen};

use super::{
    exhaustive_owner::{Kind, ValueGenIter},
    flatten_shrinker::FlattenShrinker,
};

pub struct Flatten<P, T>(P, PhantomData<T>);

impl<P: ValueGen, T> Flatten<P, T> {
    pub fn new<I: IntoValueGen<P::Value, Gen = P>>(parent_gen: I) -> Self {
        Self(parent_gen.into_value_gen(), PhantomData)
    }
}

impl<P: ValueGen, C: ValueGen> ValueGen for Flatten<P, C>
where
    P::Value: IntoValueGen<C::Value, Gen = C>,
{
    type Value = C::Value;
    type Seed = (P::Seed, C::Seed);
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = FlattenShrinker<'a, P, C>
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        const ITERATIONS_BAILOUT_COUNT: usize = 100_000_000;

        self.0
            .exhaustive()
            .enumerate()
            .map(|(i, parent_seed)| {
                (i < ITERATIONS_BAILOUT_COUNT)
                    .then(|| {
                        self.0
                            .create_value(parent_seed)
                            .into_value_gen()
                            .cardinality()
                    })
                    .flatten()
            })
            .try_fold(0usize, |total, next| total.checked_add(next?))
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.exhaustive().flat_map(|parent_seed| {
            ValueGenIter::new(
                self.0.create_value(parent_seed.clone()).into_value_gen(),
                Kind::Exhaustive,
            )
            .map(move |child_seed| (parent_seed.clone(), child_seed))
        })
    }

    fn adversarial_count(&self) -> Option<usize> {
        const ITERATIONS_BAILOUT_COUNT: usize = 100_000_000;

        self.0
            .adversarial()
            .enumerate()
            .map(|(i, parent_seed)| {
                (i < ITERATIONS_BAILOUT_COUNT)
                    .then(|| {
                        self.0
                            .create_value(parent_seed)
                            .into_value_gen()
                            .adversarial_count()
                    })
                    .flatten()
            })
            .try_fold(0usize, |total, next| total.checked_add(next?))
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.adversarial().flat_map(|parent_seed| {
            ValueGenIter::new(
                self.0.create_value(parent_seed.clone()).into_value_gen(),
                Kind::Adversarial,
            )
            .map(move |child_seed| (parent_seed.clone(), child_seed))
        })
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        let parent_seed = self.0.sample(rng);
        let child_seed = self
            .0
            .create_value(parent_seed.clone())
            .into_value_gen()
            .sample(rng);

        (parent_seed, child_seed)
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        FlattenShrinker::new(failing_value_seed, &self.0)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        self.0
            .create_value(seed.0)
            .into_value_gen()
            .create_value(seed.1)
    }
}
