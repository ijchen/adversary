use std::ops::RangeInclusive;

use crate::{
    generators::base_generators::numeric_ranges::RangeInclusiveGen, shrinker::Shrinker,
    InputGenerator, IntoInputGenerator,
};

use super::shrinker::RangeInclusiveShrinkerSigned;

macro_rules! signed_range_inclusive {
    ($($i:ty = $u:ty ),+$(,)?) => {$(
        const _: () = assert!(size_of::<$i>() == size_of::<$u>());

        impl IntoInputGenerator<$i> for RangeInclusive<$i> {
            fn into_input_generator(self) -> impl InputGenerator<Input = $i> {
                let min = *self.start();
                let max = *self.end();
                assert!(min <= max);

                RangeInclusiveGen { min, max }
            }
        }

        impl RangeInclusiveGen<$i> {
            fn cardinality_infallible(&self) -> $u {
                // This uses some pretty sexy two's complement modular
                // arithmetic tricks to avoid overflow issues
                // Sanity check: https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=f5e25ab4f97e57206163f8ece48d8aa6
                <$u>::wrapping_sub(self.max as $u, self.min as $u)
            }
        }

        impl InputGenerator for RangeInclusiveGen<$i> {
            type Input = $i;

            type InputSource = Self::Input;

            fn cardinality(&self) -> Option<usize> {
                usize::try_from(self.cardinality_infallible()).ok().and_then(|cardinality| cardinality.checked_add(1))
            }

            fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
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
            fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
                match self.cardinality_infallible() {
                    0..=7 => (self.min..=self.max).collect(),
                    cardinality => {
                        // Infallible - `uN::MAX / 2 <= iN::MAX` is always true
                        let half_cardinality = <$i>::try_from(cardinality / 2).unwrap();

                        let mut nums = Vec::with_capacity(10);

                        nums.push(self.min);
                        nums.push(self.max);
                        nums.push(self.min + 1);
                        nums.push(self.max - 1);

                        nums.push(self.min + half_cardinality - 1);
                        nums.push(self.min + half_cardinality);
                        if cardinality % 2 == 1 {
                            nums.push(self.min + half_cardinality + 1);
                        }

                        if !nums.contains(&-1) {
                            nums.push(-1)
                        }
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
                _failing_input: Self::InputSource,
            ) -> impl Shrinker<InputSource = Self::InputSource> {
                RangeInclusiveShrinkerSigned::<$i>::new()
            }

            fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
                input_source
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

// TODO(ichen): comment tests back in when shrinking is implemented
// #[cfg(test)]
// mod tests {
//     use crate::prelude::*;

//     // TODO: have a cooler name
//     #[test]
//     fn test_with_cool_name() {
//         assert_eq!(
//             run_test(|_| false, -42..=6, &mut crate::rand::thread_rng())
//                 .unwrap_err()
//                 .simplest_failing_input,
//             0
//         );

//         assert_eq!(
//             run_test(|n| n < 123, -45..=1000i64, &mut crate::rand::thread_rng())
//                 .unwrap_err()
//                 .simplest_failing_input,
//             123
//         );

//         assert_eq!(
//             run_test(
//                 |_| false,
//                 i128::MIN..=i128::MAX,
//                 &mut crate::rand::thread_rng()
//             )
//             .unwrap_err()
//             .simplest_failing_input,
//             0
//         );

//         assert_eq!(
//             run_test(|n| n > -100, -421..=-21i32, &mut crate::rand::thread_rng())
//                 .unwrap_err()
//                 .simplest_failing_input,
//             -100
//         );

//         assert_eq!(
//             run_test(
//                 |n| n < 643,
//                 45..=2000000i128,
//                 &mut crate::rand::thread_rng()
//             )
//             .unwrap_err()
//             .simplest_failing_input,
//             643
//         );

//         assert_eq!(
//             run_test(|n| n > -6, i16::MIN..=3, &mut crate::rand::thread_rng())
//                 .unwrap_err()
//                 .simplest_failing_input,
//             -6
//         );
//     }
// }
