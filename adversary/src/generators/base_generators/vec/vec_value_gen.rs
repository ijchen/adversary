use crate::{IntoValueGen, RangeAwareValueGen, ValueGen, vec::shrinker::VecShrinker};

use super::{lazy_collected_iter::LazyCollectedIter, multi_radix_counter::MultiRadixCounter};
pub struct VecValueGen<G: ValueGen, L: RangeAwareValueGen<Value = usize>> {
    elem_gen: G,
    len_gen: L,
}

impl<G: ValueGen, L: RangeAwareValueGen<Value = usize>> VecValueGen<G, L> {
    pub fn new<IntoG: IntoValueGen<G::Value, Gen = G>, IntoL: IntoValueGen<usize, Gen = L>>(
        elem_gen: IntoG,
        len_gen: IntoL,
    ) -> Self {
        Self {
            elem_gen: elem_gen.into_value_gen(),
            len_gen: len_gen.into_value_gen(),
        }
    }
}

impl<G: ValueGen, L: RangeAwareValueGen<Value = usize>> ValueGen for VecValueGen<G, L> {
    type Value = Vec<G::Value>;
    type Seed = Box<[G::Seed]>;

    type Shrinker<'a>
        = VecShrinker<'a, G>
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        let elem_cardinality = self.elem_gen.cardinality()?;

        // for each possible length, there are elem_cardinality^length possible
        // unique vecs
        self.len_gen
            .exhaustive()
            .map(|len_seed| {
                let len: u32 = self.len_gen.create_value(len_seed).try_into().ok()?;
                elem_cardinality.checked_pow(len)
            })
            .try_fold(0, |accum, elem| usize::checked_add(accum, elem?))
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        let elem_cardinality = self.elem_gen.cardinality().unwrap();
        let mut elems = LazyCollectedIter::new(self.elem_gen.exhaustive());

        self.len_gen
            .exhaustive()
            .flat_map(move |len_seed| {
                MultiRadixCounter::new(elem_cardinality, self.len_gen.create_value(len_seed))
            })
            .map(move |indices| {
                indices
                    .into_iter()
                    .map(|index| elems.get(index).unwrap().clone())
                    .collect()
            })
    }

    fn adversarial_count(&self) -> Option<usize> {
        let elem_gen_adversarial_count = self.elem_gen.adversarial_count();

        let mut adversarial_count = 0;

        if self.len_gen.value_in_range(&0) {
            adversarial_count += 1;
        }

        if elem_gen_adversarial_count.is_some_and(|count| self.len_gen.value_in_range(&count)) {
            adversarial_count += 1;
        }

        if let Some(count) = elem_gen_adversarial_count
            && self.len_gen.value_in_range(&1)
        {
            adversarial_count += count;
        }

        Some(adversarial_count)
    }

    // Adversarial values:
    // - The empty vec (if 0 is a valid length)
    // - A vec containing all adversarial values of elem_gen (if that length is
    //   valid)
    // - Each adversarial value of elem_gen in a single-element vec (if 1 is a
    //   valid length AND elem_gen.adversarial_count().is_some())
    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        let elem_gen_adversarial_count = self.elem_gen.adversarial_count();

        let mut adversarial = Vec::with_capacity(self.adversarial_count().unwrap_or_default());

        // If 0 is a valid length, add the empty vector
        if self.len_gen.value_in_range(&0) {
            adversarial.push(vec![].into_boxed_slice());
        }

        // If the length of a vector containing all adversarial values together
        // is a valid length for the length gen, add that "all adversarial
        // values" vector
        if elem_gen_adversarial_count.is_some_and(|count| self.len_gen.value_in_range(&count)) {
            adversarial.push(self.elem_gen.adversarial().collect());
        }

        // If 1 is a valid length, add all `elem_gen` adversarial values as
        // single-element vectors
        if elem_gen_adversarial_count.is_some() && self.len_gen.value_in_range(&1) {
            adversarial.extend(
                self.elem_gen
                    .adversarial()
                    .map(|elem| vec![elem].into_boxed_slice()),
            );
        }

        adversarial.into_iter()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        let len = self.len_gen.create_value(self.len_gen.sample(rng));

        // TODO(ichen): a `std::iter::repeat_with_n` would be kinda nice
        std::iter::repeat_with(|| self.elem_gen.sample(rng))
            .take(len)
            .collect()
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        VecShrinker::new(&self.elem_gen, failing_value_seed)
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        seed.into_iter()
            .map(|seed| self.elem_gen.create_value(seed))
            .collect()
    }
}
