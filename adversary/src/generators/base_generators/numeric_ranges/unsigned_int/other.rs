use std::ops::{Range, RangeFrom, RangeFull, RangeTo, RangeToInclusive};

use crate::{IntoValueGen, generators::base_generators::numeric_ranges::RangeInclusiveGen};

macro_rules! others {
    ($($t: ty),+$(,)?) => {$(
        impl IntoValueGen<$t> for Range<$t> {
            // TODO: use ATPIT once stabilized
            type Gen = RangeInclusiveGen<$t>;

            fn into_value_gen(self) -> Self::Gen {
                assert!(!self.is_empty());

                (self.start..=self.end - 1).into_value_gen()
            }
        }

        impl IntoValueGen<$t> for RangeTo<$t> {
            // TODO: use ATPIT once stabilized
            type Gen = RangeInclusiveGen<$t>;

            fn into_value_gen(self) -> Self::Gen {
                assert!(self.end > <$t>::MIN);

                (<$t>::MIN..=self.end - 1).into_value_gen()
            }
        }

        impl IntoValueGen<$t> for RangeToInclusive<$t> {
            // TODO: use ATPIT once stabilized
            type Gen = RangeInclusiveGen<$t>;

            fn into_value_gen(self) -> Self::Gen {
                (<$t>::MIN..=self.end).into_value_gen()
            }
        }

        impl IntoValueGen<$t> for RangeFrom<$t> {
            // TODO: use ATPIT once stabilized
            type Gen = RangeInclusiveGen<$t>;

            fn into_value_gen(self) -> Self::Gen {
                (self.start..=<$t>::MAX).into_value_gen()
            }
        }

        impl IntoValueGen<$t> for RangeFull {
            // TODO: use ATPIT once stabilized
            type Gen = RangeInclusiveGen<$t>;

            fn into_value_gen(self) -> Self::Gen {
                (<$t>::MIN..=<$t>::MAX).into_value_gen()
            }
        }
    )+}
}

others! { u8, u16, u32, u64, u128, usize }

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_shrinks_to_min() {
        assert_eq!(
            run_test_bool(
                |_| false,
                0u8..6,
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
                45..1000u16,
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
                45..2000000usize,
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
                532..u128::MAX,
                &mut crate::rand::thread_rng(),
                Config::default()
            )
            .unwrap_report()
            .simplest_failing_value(),
            &1234
        );
    }
}
