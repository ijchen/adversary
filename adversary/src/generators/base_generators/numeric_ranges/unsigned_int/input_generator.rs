use std::ops::RangeInclusive;

use crate::{shrinker::Shrinker, InputGenerator, IntoInputGenerator};

use super::{super::RangeInclusiveGen, shrinker::RangeInclusiveShrinkerUnsigned};

macro_rules! unsigned_range_inclusive {
    ($($t: ty),+$(,)?) => {$(
        impl IntoInputGenerator<$t> for RangeInclusive<$t> {
            fn into_input_generator(self) -> impl InputGenerator<Input = $t> {
                let min = *self.start();
                let max = *self.end();
                assert!(min <= max);

                RangeInclusiveGen { min, max }
            }
        }

        impl InputGenerator for RangeInclusiveGen<$t> {
            type Input = $t;

            type InputSource = Self::Input;

            fn cardinality(&self) -> Option<usize> {
                // For unsigned ints where min <= max, max - min can't overflow
                usize::try_from(self.max - self.min).ok().and_then(|cardinality| cardinality.checked_add(1))
            }

            fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
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
            fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
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

            fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
                rng.gen_range(self.min..=self.max)
            }

            fn new_shrinker(
                &self,
                failing_input: Self::InputSource,
            ) -> impl Shrinker<InputSource = Self::InputSource> {
                RangeInclusiveShrinkerUnsigned::<$t>::new(self.min, failing_input)
            }

            fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
                input_source
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
            run_test(|_| false, 0..=6u8, &mut crate::rand::thread_rng())
                .unwrap_err()
                .simplest_failing_input(),
            &0
        );

        assert_eq!(
            run_test(|n| n < 123, 45..=1000u32, &mut crate::rand::thread_rng())
                .unwrap_err()
                .simplest_failing_input(),
            &123
        );

        assert_eq!(
            run_test(
                |n| n < 643,
                45..=2000000u128,
                &mut crate::rand::thread_rng()
            )
            .unwrap_err()
            .simplest_failing_input(),
            &643
        );

        assert_eq!(
            run_test(
                |n| n < 1234,
                532..=u128::MAX,
                &mut crate::rand::thread_rng()
            )
            .unwrap_err()
            .simplest_failing_input(),
            &1234
        );

        assert_eq!(
            run_test(
                |n| n < 2500 || n % 71 != 0,
                any::<u128>(),
                &mut crate::rand::thread_rng()
            )
            .unwrap_err()
            .simplest_failing_input(),
            &((2500 as f64 / 71 as f64).ceil() as u128 * 71)
        );
    }

    #[test]
    fn adversary_sanity_check() {
        // 1 2 3 4 5 6 7
        // ^ ^ ^ ^ ^ ^ ^
        let gen = (1u32..=7).into_input_generator();
        assert_eq!(
            HashSet::from([1, 2, 3, 4, 5, 6, 7]),
            gen.adversarial()
                .map(|is| gen.create_input(is))
                .collect::<HashSet<_>>()
        );

        // 1 2 3 4 5 6 7 8
        // ^ ^   ^ ^   ^ ^
        let gen = (1u32..=8).into_input_generator();
        assert_eq!(
            HashSet::from([1, 2, 4, 5, 7, 8]),
            gen.adversarial()
                .map(|is| gen.create_input(is))
                .collect::<HashSet<_>>()
        );

        // 1 2 3 4 5 6 7 8 9
        // ^ ^   ^ ^ ^   ^ ^
        let gen = (1u32..=9).into_input_generator();
        assert_eq!(
            HashSet::from([1, 9, 2, 8, 4, 5, 6]),
            gen.adversarial()
                .map(|is| gen.create_input(is))
                .collect::<HashSet<_>>()
        );

        // 32 33 34 35 36 37 38 39 40 41 42 43 44
        // ^^ ^^          ^^ ^^ ^^          ^^ ^^
        let gen = (32u32..=44).into_input_generator();
        assert_eq!(
            HashSet::from([32, 33, 37, 38, 39, 43, 44]),
            gen.adversarial()
                .map(|is| gen.create_input(is))
                .collect::<HashSet<_>>()
        );

        // 32 33 34 35 36 37 38 39 40 41 42 43 44 45
        // ^^ ^^             ^^ ^^             ^^ ^^
        let gen = (32u32..=45).into_input_generator();
        assert_eq!(
            HashSet::from([32, 33, 38, 39, 44, 45]),
            gen.adversarial()
                .map(|is| gen.create_input(is))
                .collect::<HashSet<_>>()
        );
    }
}
