use crate::{Adversarial, Exhaustive, InputGenerator, Sample, Shrink};

use super::Canonical;

macro_rules! impl_input_generator_float {
    ($($f: ty [$u: ty]),+$(,)?) => {
        $(
            impl Exhaustive<$f> for Canonical {
                fn cardinality(&self) -> Option<usize> {
                    usize::checked_pow(2, <$u>::BITS)
                }

                fn exhaustive(&self) -> impl Iterator<Item = $f> {
                    (<$u>::MIN..=<$u>::MAX).map(<$f>::from_bits)
                }
            }

            impl Adversarial<$f> for Canonical {
                fn adversarial_count(&self) -> usize {
                    16
                }

                fn adversarial(&self) -> impl Iterator<Item = $f> {
                    const MANT_MASK: $u = (1 << (<$f>::MANTISSA_DIGITS - 1)) - 1;
                    [
                        // Basics
                        0.0,
                        -0.0,
                        1.0,
                        -1.0,

                        // Non-finite
                        <$f>::INFINITY,
                        <$f>::NEG_INFINITY,
                        <$f>::NAN,

                        // Subnormals
                        -<$f>::from_bits(MANT_MASK), // Minimum negative subnormal
                        -<$f>::from_bits(1), // Maximum negative subnormal
                        <$f>::from_bits(1), // Minimum positive subnormal
                        <$f>::from_bits(MANT_MASK), // Maximum positive subnormal

                        // Other weird numbers
                        <$f>::EPSILON,
                        <$f>::MIN,
                        <$f>::MAX,
                        <$f>::MIN_POSITIVE,
                        -<$f>::MIN_POSITIVE,
                    ].into_iter()
                }
            }

            impl Sample<$f> for Canonical {
                fn sample(&self, rng: &mut impl rand::Rng) -> $f {
                    <$f>::from_bits(rng.gen())
                }
            }

            impl Shrink<$f> for Canonical {
                type History = ();

                fn history_from_failure(&self, _failing_input: $f) -> Self::History {
                    todo!()
                }

                fn update_history(&self, _history: &mut Self::History, _input: $f, _test_passed: bool) {
                    todo!()
                }

                fn next_input(&self, _rng: &mut impl rand::Rng, _history: &Self::History) -> Option<$f> {
                    todo!()
                }
            }

            impl InputGenerator<$f> for Canonical {}
        )+
    }
}

impl_input_generator_float! { f32 [u32], f64 [u64] }
