use std::ops::RangeInclusive;

use crate::{IntoValueGen, ValueGen};

use super::{super::RangeInclusiveGen, shrinker::RangeInclusiveShrinkerUnsigned};

macro_rules! unsigned_range_inclusive {
    ($($t: ty),+$(,)?) => {$(
        impl IntoValueGen<$t> for RangeInclusive<$t> {
            // TODO: use ATPIT once stabilized
            type Gen = RangeInclusiveGen<$t>;

            fn into_value_gen(self) -> Self::Gen {
                let min = *self.start();
                let max = *self.end();
                assert!(min <= max);

                RangeInclusiveGen { min, max }
            }
        }

        impl ValueGen for RangeInclusiveGen<$t> {
            type Value = $t;
            type Seed = Self::Value;
            // TODO: use ATPIT once stabilized
            type Shrinker<'a>
                = RangeInclusiveShrinkerUnsigned<$t>
            where
                Self: 'a;

            fn cardinality(&self) -> Option<usize> {
                // For unsigned ints where min <= max, max - min can't overflow
                usize::try_from(self.max - self.min).ok().and_then(|cardinality| cardinality.checked_add(1))
            }

            fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
                self.min..=self.max
            }

            fn adversarial_count(&self) -> Option<usize> {
                // TODO: at some point, write a version of this that doesn't
                // need to call `adversarial` by using smart math and knowledge
                Some(self.adversarial().count())
            }

            // For unsigned ints, the potential adversarial values are:
            // - Range start and end
            // - Range start + 1 and end - 1
            // - The middle two or three numbers, whichever is symmetrical
            //
            // Note that each potential value is only included if it actually
            // falls within the range of allowed values. Also note that this
            // will always include 0 and 1 if possible - as long as they are
            // within the range, they will be either the range start, or the
            // range start + 1.
            //
            // TODO: at some point, consider an optimized version of this that
            // doesn't allocate and uses smart math
            fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
                let cardinality_minus_one = self.max - self.min;
                match cardinality_minus_one {
                    0..=6 => (self.min..=self.max).collect(),
                    cardinality_minus_one => {
                        let mut nums = Vec::with_capacity(7);
                        let half_cardinality = self.min + cardinality_minus_one / 2;

                        nums.push(self.min);
                        nums.push(self.max);
                        nums.push(self.min + 1);
                        nums.push(self.max - 1);

                        if cardinality_minus_one % 2 == 0 {
                            nums.push(half_cardinality - 1);
                        }
                        nums.push(half_cardinality);
                        nums.push(half_cardinality + 1);

                        nums
                    }
                }
                .into_iter()
            }

            fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
                rng.gen_range(self.min..=self.max)
            }

            fn new_shrinker(&self, seed: Self::Seed) -> Self::Shrinker<'_> {
                RangeInclusiveShrinkerUnsigned::<$t>::new(self.min, seed)
            }

            fn create_value(&self, seed: Self::Seed) -> Self::Value {
                seed
            }
        }
    )+};
}

unsigned_range_inclusive! { u8, u16, u32, u64, u128, usize }

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::prelude::*;

    #[test]
    fn test_shrinks_to_min() {
        assert_eq!(
            run_test_bool(
                |_| false,
                0..=6u8,
                &mut crate::rand::thread_rng(),
                Config::default()
            )
            .unwrap_report()
            .simplest_failing_value(),
            &0
        );

        assert_eq!(
            run_test_bool(
                |n| n < 123,
                45..=1000u32,
                &mut crate::rand::thread_rng(),
                Config::default()
            )
            .unwrap_report()
            .simplest_failing_value(),
            &123
        );

        assert_eq!(
            run_test_bool(
                |n| n < 643,
                45..=2000000u128,
                &mut crate::rand::thread_rng(),
                Config::default()
            )
            .unwrap_report()
            .simplest_failing_value(),
            &643
        );

        assert_eq!(
            run_test_bool(
                |n| n < 1234,
                532..=u128::MAX,
                &mut crate::rand::thread_rng(),
                Config::default()
            )
            .unwrap_report()
            .simplest_failing_value(),
            &1234
        );

        assert_eq!(
            run_test_bool(
                |n| n < 2500 || n % 71 != 0,
                any::<u128>(),
                &mut crate::rand::thread_rng(),
                Config::default()
            )
            .unwrap_report()
            .simplest_failing_value(),
            &((2500 as f64 / 71 as f64).ceil() as u128 * 71)
        );
    }

    #[test]
    fn adversary_sanity_check() {
        // 1 2 3 4 5 6 7
        // ^ ^ ^ ^ ^ ^ ^
        let value_gen = (1u32..=7).into_value_gen();
        assert_eq!(
            HashSet::from([1, 2, 3, 4, 5, 6, 7]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );

        // 1 2 3 4 5 6 7 8
        // ^ ^   ^ ^   ^ ^
        let value_gen = (1u32..=8).into_value_gen();
        assert_eq!(
            HashSet::from([1, 2, 4, 5, 7, 8]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );

        // 1 2 3 4 5 6 7 8 9
        // ^ ^   ^ ^ ^   ^ ^
        let value_gen = (1u32..=9).into_value_gen();
        assert_eq!(
            HashSet::from([1, 9, 2, 8, 4, 5, 6]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );

        // 32 33 34 35 36 37 38 39 40 41 42 43 44
        // ^^ ^^          ^^ ^^ ^^          ^^ ^^
        let value_gen = (32u32..=44).into_value_gen();
        assert_eq!(
            HashSet::from([32, 33, 37, 38, 39, 43, 44]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );

        // 32 33 34 35 36 37 38 39 40 41 42 43 44 45
        // ^^ ^^             ^^ ^^             ^^ ^^
        let value_gen = (32u32..=45).into_value_gen();
        assert_eq!(
            HashSet::from([32, 33, 38, 39, 44, 45]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );
    }
}
