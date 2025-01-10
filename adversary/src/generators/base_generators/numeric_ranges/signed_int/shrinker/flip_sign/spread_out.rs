use crate::generators::base_generators::numeric_ranges::common::shrinkers::StepBy;

use super::{
    super::{
        super::shrinker::{RangeInclusiveShrinkerSigned, ShrinkMagnitude},
        Done,
    },
    consecutive::Consecutive,
    FlipSign,
};

pub const FLIP_SIGN_SPREAD_COUNT: i8 = 12;
const _: () = assert!(FLIP_SIGN_SPREAD_COUNT >= 2);

/// Implementation of the "Spread out" sub-phase of the "Flip sign" phase of
/// signed integer shrinking.
///
/// # Description
/// This phase selects a fixed number ([`FLIP_SIGN_SPREAD_COUNT`]) of integers,
/// approximately evenly spaced out between zero and the most complex value of
/// the opposite sign that is still simpler than the current simplest known
/// failing value. This range is divided into equal-sized sections, and the
/// center of that section is used as an input.
///
/// Because of rounding that occurs with integer math, the values chosen may not
/// be the mathematically closest integers to the "true" most evenly spread out
/// real numbers (especially when the range is very small). This precision could
/// be achieved, but I don't think the implementation complexity is worth the
/// small gain in mathematical purity. After all, the point of shrinking is just
/// to aid in debugging for the developer, and all of these phases are really
/// just heuristics to shrink well in as many common cases as possible. It's
/// okay if the distribution is a little off sometimes.
///
/// # Next phase
/// Once all [`FLIP_SIGN_SPREAD_COUNT`] values have been attempted (and none of
/// them failed), we will move on to the "Consecutive" sub-phase. If any value
/// fails, we will immediately jump out of the "Flip sign" phase backwards to
/// the "Shrink magnitude" outer phase, unless the failing value is 0, -1, or 1.
/// If 0/-1/1 failed, there's no point shrinking magnitude, we just jump right
/// to "Done".
///
/// # Goal
/// The goal behind this phase is to sample many spread out points in the input
/// space of simpler values with the opposite sign, with the hope that we will
/// catch any "pockets" of failing values.
//
// # Invariants
//
// ## The "Valid range" invariant
// `self.min < 0 < self.max`
//
// ## The "Not done" invariant
// `self.step_by` is not "done" (it's .current() method is returning `Some`)
#[derive(Debug, Clone)]
pub struct SpreadOut<T> {
    step_by: StepBy<T>,

    /// The current simplest known failing value.
    simplest_known_failing: T,

    /// The minimum value in the shrinking range.
    min: T,

    /// The maximum value in the shrinking range.
    max: T,
}

macro_rules! spread_out {
    ($($i:ty = $u:ty),+$(,)?) => {$(
        impl SpreadOut<$i> {
            /// Returns a new [`SpreadOut`] in the initial state, or [`None`] if
            /// this step should be skipped (based on the input arguments).
            ///
            /// # Panics
            /// if any of the following invariants are not true:
            /// - `min < 0 < max`
            /// - `min <= simplest_known_failing <= max`
            /// - `simplest_known_failing != 0`
            pub fn new(min: $i, max: $i, simplest_known_failing: $i) -> Option<Self> {
                assert!(min < 0 && 0 < max);
                assert!(min <= simplest_known_failing && simplest_known_failing <= max);
                assert!(simplest_known_failing != 0);

                // This is the most complex value of the opposite sign simpler
                // than the current simplest known failing value. I am not
                // creative enough to give that a meaningful variable name that
                // is not a whole sentence.
                let magic_number = FlipSign::<$i>::most_complex_value_simpler_than(min, max, simplest_known_failing)?;

                // If there are fewer than FLIP_SIGN_SPREAD_COUNT values between
                // the most complex value of the opposite sign simpler than the
                // current simplest known failing value, this step should be
                // skipped.
                if magic_number.unsigned_abs() < <$i>::from(FLIP_SIGN_SPREAD_COUNT).unsigned_abs() {
                    return None;
                }

                // TODO: why can't this .unwrap() panic?
                let step_by = StepBy::<$i>::new(
                    magic_number,
                    0,
                    -(magic_number / <$i>::from(FLIP_SIGN_SPREAD_COUNT)),
                ).unwrap();

                step_by.current();

                // Invariant: TODO
                Some(Self { step_by, simplest_known_failing, min, max })
            }

            pub fn current_attempt(&self) -> Option<$i> {
                Some(self.step_by.current().unwrap())
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<$i, $u> {
                // If we found a failing value, save it and move either back to
                // shrink magnitude, or to done
                if !current_attempt_passed {
                    let current_attempt = self.current_attempt().expect("SpreadOut::current_attempt always returns Some");

                    // If the failing value we found is 0 or +/-1, there's no
                    // point shrinking magnitude - we're done.
                    return if current_attempt.unsigned_abs() <= 1 {
                        RangeInclusiveShrinkerSigned::Done(Done::new())
                    }
                    // Otherwise, there's more values to try between zero and
                    // the new simplest failing input we just found, so start
                    // back at the "Shrink magnitude" phase with our new
                    // information
                    else {
                        // TODO: Invariants
                        RangeInclusiveShrinkerSigned::ShrinkMagnitude(ShrinkMagnitude::<$i, $u>::new(current_attempt, self.min, self.max))
                    };
                }

                let next_step_by = self.step_by.next();

                // If we've finished the spread out search, move on to the
                // consecutive phase
                if next_step_by.done() {
                    if let Some(consecutive) = Consecutive::<$i>::new(
                        self.simplest_known_failing,
                        self.min,
                        self.max,
                    ) {
                        RangeInclusiveShrinkerSigned::FlipSign(FlipSign::Consecutive(consecutive))
                    } else {
                        RangeInclusiveShrinkerSigned::Done(Done::new())
                    }
                }
                else {
                    // Progress to the next value
                    // TODO: invariants
                    RangeInclusiveShrinkerSigned::FlipSign(FlipSign::SpreadOut(Self {
                        step_by: next_step_by,
                        simplest_known_failing: self.simplest_known_failing,
                        min: self.min,
                        max: self.max
                    }))
                }
            }
        }
    )+};
}

spread_out! {
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize,
}
