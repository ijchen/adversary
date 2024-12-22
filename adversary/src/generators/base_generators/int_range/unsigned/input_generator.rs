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
            // - 0, 1
            //
            // Note that each potential value is only included if it actually
            // falls within the range of allowed values.
            //
            // TODO: at some point, consider an optimized version of this that
            // doesn't allocate and uses smart math
            fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
                let cardinality_minus_one = self.max - self.min;
                match cardinality_minus_one {
                    0..=6 => (self.min..=self.max).collect(),
                    cardinality_minus_one => {
                        let mut nums = Vec::with_capacity(9);

                        nums.push(self.min);
                        nums.push(self.max);
                        nums.push(self.min + 1);
                        nums.push(self.max - 1);

                        if cardinality_minus_one % 2 == 0 {
                            nums.push(self.min + cardinality_minus_one / 2 - 1);
                        }
                        nums.push(self.min + cardinality_minus_one / 2);
                        nums.push(self.min + cardinality_minus_one / 2 + 1);

                        if !nums.contains(&0) {
                            nums.push(0)
                        }
                        if !nums.contains(&1) {
                            nums.push(1)
                        }

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
}
