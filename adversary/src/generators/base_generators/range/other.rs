use std::ops::{Range, RangeFrom, RangeFull, RangeTo, RangeToInclusive};

use crate::{InputGenerator, IntoInputGenerator};

macro_rules! unsigned_others {
    ($($t: ty),+$(,)?) => {$(
        impl IntoInputGenerator<$t> for Range<$t> {
            fn into_input_generator(self) -> impl InputGenerator<Input = $t> {
                assert!(!self.is_empty());

                (self.start..=self.end - 1).into_input_generator()
            }
        }

        impl IntoInputGenerator<$t> for RangeTo<$t> {
            fn into_input_generator(self) -> impl InputGenerator<Input = $t> {
                assert!(self.end > 0);

                (0..=self.end - 1).into_input_generator()
            }
        }

        impl IntoInputGenerator<$t> for RangeToInclusive<$t> {
            fn into_input_generator(self) -> impl InputGenerator<Input = $t> {
                (0..=self.end).into_input_generator()
            }
        }

        impl IntoInputGenerator<$t> for RangeFrom<$t> {
            fn into_input_generator(self) -> impl InputGenerator<Input = $t> {
                (self.start..=<$t>::MAX).into_input_generator()
            }
        }

        impl IntoInputGenerator<$t> for RangeFull {
            fn into_input_generator(self) -> impl InputGenerator<Input = $t> {
                (0..=<$t>::MAX).into_input_generator()
            }
        }
    )+}
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
                .simplest_failing_input(),
            &0
        );

        assert_eq!(
            run_test(|n| n < 123, 45..1000u16, &mut crate::rand::thread_rng())
                .unwrap_err()
                .simplest_failing_input(),
            &123
        );

        assert_eq!(
            run_test(
                |n| n < 643,
                45..2000000usize,
                &mut crate::rand::thread_rng()
            )
            .unwrap_err()
            .simplest_failing_input(),
            &643
        );

        assert_eq!(
            run_test(|n| n < 1234, 532..u128::MAX, &mut crate::rand::thread_rng())
                .unwrap_err()
                .simplest_failing_input(),
            &1234
        );
    }
}
