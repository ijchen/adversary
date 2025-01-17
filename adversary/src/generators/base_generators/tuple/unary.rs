use crate::{IntoValueGen, ValueGen};

// TODO: once ATPIT is stabilized, we can just use this simpler implementation
// impl<T, IntoGen: IntoValueGen<T>> IntoValueGen<(T,)> for (IntoGen,) {
//     // TODO: use ATPIT once stabilized
//     type Gen = impl ValueGen<Value = (T,)>;

//     fn into_value_gen(self) -> Self::Gen {
//         self.0.into_value_gen().adv_map(|value| (value,))
//     }
// }

impl<T, IntoGen: IntoValueGen<T>> IntoValueGen<(T,)> for (IntoGen,) {
    // TODO: use ATPIT once stabilized
    type Gen = UnaryTupleValueGen<IntoGen::Gen>;

    fn into_value_gen(self) -> Self::Gen {
        UnaryTupleValueGen(self.0.into_value_gen())
    }
}

// TODO: this should not be pub, make private once ATPIT allows IntoValueGen
// impls to hide the concrete type of IntoValueGen::Gen
pub struct UnaryTupleValueGen<G>(G);

impl<G: ValueGen> ValueGen for UnaryTupleValueGen<G> {
    type Value = (G::Value,);
    type Seed = G::Seed;
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = G::Shrinker<'a>
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        self.0.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.0.adversarial_count()
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.adversarial()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        self.0.sample(rng)
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        self.0.new_shrinker(failing_value_seed)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        (self.0.create_value(seed),)
    }
}
