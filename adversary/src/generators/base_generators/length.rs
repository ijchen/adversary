use std::marker::PhantomData;

use crate::{
    RangeAwareValueGen, ValueGen,
    generators::base_generators::numeric_ranges::RangeInclusiveShrinkerUnsigned,
    report::Observation, shrinker::Shrinker,
};

/// A [`ValueGen`] that produces lengths for a collection of `T`s.
//
// TODO(ijchen): more useful information
pub fn length_gen<T>() -> impl RangeAwareValueGen<Value = usize, Seed = usize> {
    LengthGen::<T>::new()
}

// See: https://doc.rust-lang.org/std/ptr/index.html#allocated-object
const MAX_COLLECTION_LEN: usize = isize::MAX as usize;

pub struct LengthGen<T> {
    _phantom: PhantomData<fn(T)>,
}

struct Bucket {
    pub min: usize,
    pub max: usize,
    pub chance: u8, // integer percent chance from 0 to 100
}

macro_rules! buckets {
    ($($min:literal ..= $max:literal @ $chance:literal %),+$(,)?) => {{
        const _: () = assert!(0 $(+ $chance)+ == 100);
        $(
            const _: () = assert!($min <= $max);
        )+

        &[$(Bucket {
            min: $min,
            max: $max,
            chance: $chance,
        }),+]
    }};
}

const fn buckets<T>() -> &'static [Bucket] {
    match size_of::<T>() {
        // Small types, 16 bytes (128 bits) or smaller
        //
        // NOTE(ichen): since these values are small, we could in theory go much
        // higher than a max of 1,000,000 elements. In practice, it's likely
        // that these elements are being looped through, or more generally that
        // some O(n) operation is happening on them - I don't want to slow down
        // tests too much as a result of this. In practice, very large
        // collections seem unlikely to be truly uniquely problematic cases for
        // the majority of code.
        0..=16 => buckets! {
            0..=0 @ 5%,
            1..=12 @ 50%,
            13..=100 @ 25%,
            101..=1_000 @ 12%,
            1_001..=10_000 @ 5%,
            10_001..=100_000 @ 2%,
            100_001..=1_000_000 @ 1%,
        },

        // Medium-sized types, 256 bytes or smaller
        17..=256 => buckets! {
            0..=0 @ 5%,
            1..=12 @ 50%,
            13..=100 @ 25%,
            101..=1_000 @ 12%,
            1_001..=10_000 @ 5%,
            10_001..=50_000 @ 3%,
        },

        // Large types, over 256 bytes
        //
        // For reference, at the time of writing (1.87.0), the largest stable,
        // non-generic type in std on linux is `std::process::Command`, coming
        // in at 208 bytes.
        257.. => buckets! {
            0..=0 @ 5%,
            1..=12 @ 69%,
            13..=100 @ 25%,
            101..=1_000 @ 1%,
        },
    }
}

impl<T> ValueGen for LengthGen<T> {
    type Value = usize;
    type Seed = usize;

    type Shrinker<'a>
        = LengthGenShrinker
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        Some(MAX_COLLECTION_LEN + 1)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        usize::MIN..=MAX_COLLECTION_LEN
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(1)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        std::iter::once(0)
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        let buckets = buckets::<T>();

        // To pick a bucket proportionally to its chance, we'll select a random
        // number in 0..chance_sum, and treat it like an "index" into a list of
        // copies of all the buckets, where each bucket has `chance` entries.
        let chance_sum = buckets.iter().map(|bucket| bucket.chance).sum();
        let mut index = rng.gen_range(0..chance_sum);
        let mut bucket_index = 0;

        while let Some(new_index) = index.checked_sub(buckets[bucket_index].chance) {
            index = new_index;
            bucket_index += 1;
        }

        // Sample from the bucket
        let bucket = &buckets[bucket_index];
        rng.gen_range(bucket.min..=bucket.max)
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        LengthGenShrinker {
            inner: RangeInclusiveShrinkerUnsigned::<usize>::new(0, failing_value_seed),
        }
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        seed
    }
}

impl<T> RangeAwareValueGen for LengthGen<T> {
    fn value_in_range(&self, value: &Self::Value) -> bool {
        *value <= MAX_COLLECTION_LEN
    }
}

impl<T> LengthGen<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

pub struct LengthGenShrinker {
    inner: RangeInclusiveShrinkerUnsigned<usize>,
}

impl Shrinker<usize> for LengthGenShrinker {
    fn current_attempt(&self) -> Option<usize> {
        self.inner.current_attempt()
    }

    fn update(&mut self, current_attempt_passed: bool) {
        self.inner.update(current_attempt_passed);
    }

    fn into_observations(self) -> Vec<Observation> {
        self.inner.into_observations()
    }
}
