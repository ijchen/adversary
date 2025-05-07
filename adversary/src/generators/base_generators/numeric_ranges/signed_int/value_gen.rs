use std::ops::RangeInclusive;

use crate::{
    IntoValueGen, RangeAwareValueGen, ValueGen,
    generators::base_generators::numeric_ranges::RangeInclusiveGen,
};

use super::shrinker::RangeInclusiveShrinkerSigned;

macro_rules! signed_range_inclusive {
    ($($i:ty = $u:ty),+$(,)?) => {$(
        const _: () = assert!(size_of::<$i>() == size_of::<$u>());

        impl IntoValueGen<$i> for RangeInclusive<$i> {
            // TODO: use ATPIT once stabilized
            type Gen = RangeInclusiveGen<$i>;

            fn into_value_gen(self) -> Self::Gen {
                let min = *self.start();
                let max = *self.end();
                assert!(min <= max);

                RangeInclusiveGen { min, max }
            }
        }

        impl ValueGen for RangeInclusiveGen<$i> {
            type Value = $i;
            type Seed = Self::Value;
            // TODO: use ATPIT once stabilized
            type Shrinker<'a>
                = RangeInclusiveShrinkerSigned<$i, $u>
            where
                Self: 'a;

            fn cardinality(&self) -> Option<usize> {
                usize::try_from(<$i>::abs_diff(self.min, self.max)).ok().and_then(|cardinality| cardinality.checked_add(1))
            }

            fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
                self.min..=self.max
            }

            fn adversarial_count(&self) -> Option<usize> {
                // TODO: at some point, write a version of this that doesn't
                // need to call `adversarial` by using smart math and knowledge
                Some(self.adversarial().count())
            }

            // For signed ints, adversarial values are:
            // - Range start and end
            // - Range start + 1 and end - 1
            // - The middle two or three numbers, whichever is symmetrical
            // - -1, 0, 1
            //
            // Note that each potential value is only included if it actually
            // falls within the range of allowed values.
            //
            // TODO: at some point, consider an optimized version of this that
            // doesn't allocate and uses smart math
            fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
                match <$i>::abs_diff(self.min, self.max) {
                    0..=6 => (self.min..=self.max).collect(),
                    cardinality_minus_one => {
                        // Infallible - `uN::MAX / 2 <= iN::MAX`
                        let half_cardinality = <$i>::try_from(cardinality_minus_one / 2).unwrap();

                        let mut nums = Vec::with_capacity(10);

                        nums.push(self.min);
                        nums.push(self.max);
                        nums.push(self.min + 1);
                        nums.push(self.max - 1);

                        if cardinality_minus_one % 2 == 0 {
                            nums.push(self.min + half_cardinality - 1);
                        }
                        nums.push(self.min + half_cardinality);
                        nums.push(self.min + half_cardinality + 1);

                        if (self.min..=self.max).contains(&-1) && !nums.contains(&-1) {
                            nums.push(-1)
                        }
                        if (self.min..=self.max).contains(&0) && !nums.contains(&0) {
                            nums.push(0)
                        }
                        if (self.min..=self.max).contains(&1) && !nums.contains(&1) {
                            nums.push(1)
                        }

                        nums
                    }
                }
                .into_iter()
            }

            fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
                rng.gen_range(self.min..=self.max)
            }

            fn new_shrinker(&self, seed: Self::Seed) -> Self::Shrinker<'_> {
                RangeInclusiveShrinkerSigned::<$i, $u>::new(seed, (self.min, self.max))
            }

            fn create_value(&self, seed: Self::Seed) -> Self::Value {
                seed
            }
        }

        impl RangeAwareValueGen for RangeInclusiveGen<$i> {
            fn value_in_range(&self, value: &Self::Value) -> bool {
                (self.min..=self.max).contains(value)
            }
        }
    )+};
}

signed_range_inclusive! {
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::prelude::*;

    // TODO: better name
    #[test]
    fn adversary_sanity_check() {
        // 1 2 3 4 5 6 7
        // ^ ^ ^ ^ ^ ^ ^
        let value_gen = (1i32..=7).into_value_gen();
        assert_eq!(
            HashSet::from([1, 2, 3, 4, 5, 6, 7]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );

        // 1 2 3 4 5 6 7 8
        // ^ ^   ^ ^   ^ ^
        let value_gen = (1i32..=8).into_value_gen();
        assert_eq!(
            HashSet::from([1, 2, 4, 5, 7, 8]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );

        // 1 2 3 4 5 6 7 8 9
        // ^ ^   ^ ^ ^   ^ ^
        let value_gen = (1i32..=9).into_value_gen();
        assert_eq!(
            HashSet::from([1, 9, 2, 8, 4, 5, 6]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );

        // 32 33 34 35 36 37 38 39 40 41 42 43 44
        // ^^ ^^          ^^ ^^ ^^          ^^ ^^
        let value_gen = (32i32..=44).into_value_gen();
        assert_eq!(
            HashSet::from([32, 33, 37, 38, 39, 43, 44]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );

        // 32 33 34 35 36 37 38 39 40 41 42 43 44 45
        // ^^ ^^             ^^ ^^             ^^ ^^
        let value_gen = (32i32..=45).into_value_gen();
        assert_eq!(
            HashSet::from([32, 33, 38, 39, 44, 45]),
            value_gen
                .adversarial()
                .map(|is| value_gen.create_value(is))
                .collect::<HashSet<_>>()
        );
    }

    // TODO(ichen): comment tests back in when shrinking is implemented
    // TODO: have a cooler name
    // #[test]
    // fn test_with_cool_name() {
    //     assert_eq!(
    //         run_test(|_| false, -42..=6, &mut crate::rand::thread_rng())
    //             .unwrap_err()
    //             .simplest_failing_value,
    //         0
    //     );

    //     assert_eq!(
    //         run_test(|n| n < 123, -45..=1000i64, &mut crate::rand::thread_rng())
    //             .unwrap_err()
    //             .simplest_failing_value,
    //         123
    //     );

    //     assert_eq!(
    //         run_test(
    //             |_| false,
    //             i128::MIN..=i128::MAX,
    //             &mut crate::rand::thread_rng()
    //         )
    //         .unwrap_err()
    //         .simplest_failing_value,
    //         0
    //     );

    //     assert_eq!(
    //         run_test(|n| n > -100, -421..=-21i32, &mut crate::rand::thread_rng())
    //             .unwrap_err()
    //             .simplest_failing_value,
    //         -100
    //     );

    //     assert_eq!(
    //         run_test(
    //             |n| n < 643,
    //             45..=2000000i128,
    //             &mut crate::rand::thread_rng()
    //         )
    //         .unwrap_err()
    //         .simplest_failing_value,
    //         643
    //     );

    //     assert_eq!(
    //         run_test(|n| n > -6, i16::MIN..=3, &mut crate::rand::thread_rng())
    //             .unwrap_err()
    //             .simplest_failing_value,
    //         -6
    //     );
    // }
}
