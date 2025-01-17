use crate::{IntoValueGen, ValueGen};

pub fn flatten<P: IntoValueGen<C>, C: IntoValueGen<T>, T>(
    parent_gen: P,
) -> impl ValueGen<Value = T> {
    Flatten(parent_gen.into_value_gen(), std::marker::PhantomData)
}

// TODO: is there a way to express this that doesn't need the PhantomData hack?
struct Flatten<P, T>(P, std::marker::PhantomData<T>);

impl<P: ValueGen<Value = C>, C: IntoValueGen<T>, T> ValueGen for Flatten<P, T> {
    type Value = T;
    type Seed = (P::Seed, <C::Gen as ValueGen>::Seed);
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = crate::shrinkers::NeverShrink<Self::Seed>
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        self.0
            .exhaustive()
            .map(|parent_input_source| {
                self.0
                    .create_value(parent_input_source)
                    .into_value_gen()
                    .cardinality()
            })
            .try_fold(0usize, |total, next| total.checked_add(next?))
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.exhaustive().flat_map(|parent_input_source| {
            self.0
                .create_value(parent_input_source.clone())
                .into_value_gen()
                .exhaustive()
                .map(|child_input_source| (parent_input_source.clone(), child_input_source))
                // TODO: why is this collect necessary? Very undesirable
                .collect::<Vec<_>>()
        })
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.0
            .adversarial()
            .map(|parent_input_source| {
                self.0
                    .create_value(parent_input_source)
                    .into_value_gen()
                    .adversarial_count()
            })
            .try_fold(0usize, |total, next| total.checked_add(next?))
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.adversarial().flat_map(|parent_input_source| {
            self.0
                .create_value(parent_input_source.clone())
                .into_value_gen()
                .adversarial()
                .map(|child_input_source| (parent_input_source.clone(), child_input_source))
                // TODO: why is this collect necessary? Very undesirable
                .collect::<Vec<_>>()
        })
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        let parent_input_source = self.0.sample(rng);
        let child_input_source = self
            .0
            .create_value(parent_input_source.clone())
            .into_value_gen()
            .sample(rng);

        (parent_input_source, child_input_source)
    }

    fn new_shrinker(&self, _failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        crate::shrinkers::NeverShrink::new() // TODO: implement flatten shrinking
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        self.0
            .create_value(seed.0)
            .into_value_gen()
            .create_value(seed.1)
    }
}
