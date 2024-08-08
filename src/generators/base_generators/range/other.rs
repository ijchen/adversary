use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::IntoInputGenerator;

macro_rules! unsigned_others {
    ($($t: ty),+$(,)?) => {
        $(
            impl IntoInputGenerator<$t> for Range<$t> {
                type IntoInputGenerator = RangeInclusive<$t>;

                fn into_input_generator(self) -> Self::IntoInputGenerator {
                    self.start..=self.end - 1
                }
            }

            impl IntoInputGenerator<$t> for RangeTo<$t> {
                type IntoInputGenerator = RangeInclusive<$t>;

                fn into_input_generator(self) -> Self::IntoInputGenerator {
                    <$t>::MIN..=self.end - 1
                }
            }

            impl IntoInputGenerator<$t> for RangeToInclusive<$t> {
                type IntoInputGenerator = RangeInclusive<$t>;

                fn into_input_generator(self) -> Self::IntoInputGenerator {
                    <$t>::MIN..=self.end
                }
            }

            impl IntoInputGenerator<$t> for RangeFrom<$t> {
                type IntoInputGenerator = RangeInclusive<$t>;

                fn into_input_generator(self) -> Self::IntoInputGenerator {
                    self.start..=<$t>::MAX
                }
            }

            impl IntoInputGenerator<$t> for RangeFull {
                type IntoInputGenerator = RangeInclusive<$t>;

                fn into_input_generator(self) -> Self::IntoInputGenerator {
                    <$t>::MIN..=<$t>::MAX
                }
            }
        )+
    }
}

unsigned_others! { u8, u16, u32, u64, u128, usize }

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_shrinks_to_min() {
        assert_eq!(
            run_test(|_| false, 0u8..6, &mut crate::rand::thread_rng())
                .unwrap_err()
                .simplest_failing_input,
            0
        );

        assert_eq!(
            run_test(|&n| n < 123, 45..1000u16, &mut crate::rand::thread_rng())
                .unwrap_err()
                .simplest_failing_input,
            123
        );

        assert_eq!(
            run_test(
                |&n| n < 643,
                45..2000000usize,
                &mut crate::rand::thread_rng()
            )
            .unwrap_err()
            .simplest_failing_input,
            643
        );

        assert_eq!(
            run_test(
                |&n| n < 1234,
                532..u128::MAX,
                &mut crate::rand::thread_rng()
            )
            .unwrap_err()
            .simplest_failing_input,
            1234
        );
    }
}
