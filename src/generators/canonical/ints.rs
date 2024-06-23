use crate::{Adversarial, Exhaustive, InputGenerator, Sample, Shrink};

use super::Canonical;

macro_rules! common {
    ($t: ty) => {
        impl Exhaustive<$t> for Canonical {
            fn cardinality(&self) -> Option<usize> {
                usize::checked_pow(2, <$t>::BITS)
            }

            fn exhaustive(&self) -> impl Iterator<Item = $t> {
                <$t>::MIN..=<$t>::MAX
            }
        }

        impl Sample<$t> for Canonical {
            fn sample(&self, rng: &mut impl rand::Rng) -> $t {
                rng.gen()
            }
        }

        impl InputGenerator<$t> for Canonical {}
    };
}

#[derive(Clone, Copy)]
pub struct IntHistory<T: Copy> {
    largest_passing: Option<T>,
    smallest_failing: T,
}

macro_rules! signed_specific {
    ($t: ty) => {
        impl Adversarial<$t> for Canonical {
            fn adversarial_count(&self) -> usize {
                7
            }

            fn adversarial(&self) -> impl Iterator<Item = $t> {
                [
                    <$t>::MIN,
                    <$t>::MIN + 1,
                    <$t>::saturating_sub(0, 1),
                    0,
                    1,
                    <$t>::MAX - 1,
                    <$t>::MAX,
                ]
                .into_iter()
            }
        }

        // TODO: do a much more thoughtful and improved implementation - this is just a
        // basic impl to get started with.
        impl Shrink<$t> for Canonical {
            type History = IntHistory<$t>;

            fn history_from_failure(&self, failing_input: $t) -> Self::History {
                IntHistory {
                    largest_passing: None,
                    smallest_failing: failing_input,
                }
            }

            fn update_history(&self, _history: &mut Self::History, _input: $t, _test_passed: bool) {
                todo!()
            }

            fn next_input(&self, _rng: &mut impl rand::Rng, history: &Self::History) -> Option<$t> {
                // If 0 is failing, we're done
                if history.smallest_failing == 0 {
                    return None;
                }

                // If we don't have any passing inputs, try 0
                let Some(_largest_passing) = history.largest_passing else {
                    return Some(0);
                };

                todo!();
            }
        }
    };
}

// #[derive(Clone, Copy)]
// struct SignedIntHistory<T: Copy> {
//     largest_passing: Option<T>,
//     smallest_failing: T,
// }

// // TODO: do a much more thoughtful and improved implementation - this is just a
// // basic impl to get started with.
// impl Shrink<i32> for Canonical {
//     type History = SignedIntHistory<i32>;

//     fn history_from_failure(&self, failing_input: i32) -> Self::History {
//         SignedIntHistory {
//             largest_passing: None,
//             smallest_failing: failing_input,
//         }
//     }

//     fn update_history(&self, history: &mut Self::History, input: i32, test_passed: bool) {
//         if test_passed {
//             match history.largest_passing {
//                 None => history.largest_passing = Some(input),
//                 Some(largest_passing) => {
//                     if input.unsigned_abs() > largest_passing.unsigned_abs() {
//                         history.largest_passing = Some(input);
//                     }
//                 }
//             };
//         } else {
//             if input.unsigned_abs() < history.smallest_failing.unsigned_abs() {
//                 history.smallest_failing = input;
//             }
//         }
//     }

//     fn next_input(&self, _rng: &mut impl rand::Rng, history: &Self::History) -> Option<i32> {
//         match history.largest_passing {
//             Some(largest_passing) => {
//                 // If our lower bound is larger than our upper bound, we don't
//                 // actually care about it - try to establish a better lower
//                 // bound with 0
//                 if history.smallest_failing.unsigned_abs() > largest_passing.unsigned_abs() {
//                     Some(0)
//                 }
//                 else {

//                 }
//             }

//             // No lower bound, try zero
//             None => Some(0),
//         }
//     }
// }

macro_rules! unsigned_specific {
    ($t: ty) => {
        impl Adversarial<$t> for Canonical {
            fn adversarial_count(&self) -> usize {
                4
            }

            fn adversarial(&self) -> impl Iterator<Item = $t> {
                [0, 1, <$t>::MAX - 1, <$t>::MAX].into_iter()
            }
        }

        // TODO: do a much more thoughtful and improved implementation - this is just a
        // basic impl to get started with.
        impl Shrink<$t> for Canonical {
            type History = IntHistory<$t>;

            fn history_from_failure(&self, failing_input: $t) -> Self::History {
                IntHistory {
                    largest_passing: None,
                    smallest_failing: failing_input,
                }
            }

            fn update_history(&self, history: &mut Self::History, input: $t, test_passed: bool) {
                match (test_passed, history.largest_passing) {
                    (true, None) => history.largest_passing = Some(input),
                    (true, Some(largest_passing)) => {
                        if input > largest_passing {
                            history.largest_passing = Some(input);
                        }
                    }
                    (false, _) => {
                        if input < history.smallest_failing {
                            history.smallest_failing = input;
                        }
                    }
                }
            }

            fn next_input(&self, _rng: &mut impl rand::Rng, history: &Self::History) -> Option<$t> {
                // If 0 is failing, we're done
                if history.smallest_failing == 0 {
                    return None;
                }

                // If we don't have any passing inputs, try 0
                let Some(largest_passing) = history.largest_passing else {
                    return Some(0);
                };

                assert!(largest_passing <= history.smallest_failing);

                // If there are no numbers between largest passing and smallest failing,
                // we're done searching
                if history.smallest_failing - largest_passing <= 1 {
                    return None;
                }

                // Binary search - try the middle
                Some((history.smallest_failing - largest_passing) / 2 + largest_passing)
            }
        }
    };
}

macro_rules! impl_input_generator_int {
    (
        u: { $($u: ty),+$(,)? }
        i: { $($i: ty),+$(,)? }
    ) => {
        $(
            unsigned_specific!($u);
            common!($u);
        )+
        $(
            signed_specific!($i);
            common!($i);
        )+
    }
}

impl_input_generator_int! {
    u: { usize, u8, u16, u32, u64, u128 }
    i: { isize, i8, i16, i32, i64, i128 }
}
