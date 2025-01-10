use crate::generators::base_generators::numeric_ranges::{
    common::shrinkers::StepBy,
    signed_int::shrinker::{done::Done, shrink_magnitude::ShrinkMagnitude},
};

use super::{FlipSign, RangeInclusiveShrinkerSigned};

pub const FLIP_SIGN_CONSEC_COUNT: i8 = 100;
const _: () = assert!(FLIP_SIGN_CONSEC_COUNT >= 1);

/// Implementation of the "Consecutive" sub-phase of the "Flip sign" phase of
/// signed integer shrinking.
///
/// # Description
/// TODO
///
/// # Next phase
/// TODO
///
/// # Goal
/// TODO
//
// # Invariants
//
// TODO
#[derive(Debug, Clone)]
pub struct Consecutive<T> {
    step_by: StepBy<T>,

    /// The minimum value in the shrinking range.
    min: T,

    /// The maximum value in the shrinking range.
    max: T,
}

macro_rules! consecutive {
    ($($i:ty = $u:ty),+$(,)?) => {$(
        impl Consecutive<$i> {
            pub fn new(simplest_known_failing: $i, min: $i, max: $i) -> Option<Self> {
                assert!(min < 0 && 0 < max);
                assert!(min <= simplest_known_failing && simplest_known_failing <= max);
                assert!(simplest_known_failing != 0);

                // This is the most complex value of the opposite sign simpler
                // than the current simplest known failing value. I am not
                // creative enough to give that a meaningful variable name that
                // is not a whole sentence.
                let magic_number = FlipSign::<$i>::most_complex_value_simpler_than(min, max, simplest_known_failing)?;

                let signum = simplest_known_failing.signum();
                let naive_start = magic_number + <$i>::from(FLIP_SIGN_CONSEC_COUNT) * signum;
                let start = if simplest_known_failing > 0 { naive_start.min(-1) } else { naive_start.max(1) };
                // TODO: why can't this .unwrap() panic?
                let step_by = StepBy::<$i>::new(
                    start,
                    magic_number + signum,
                    signum,
                ).unwrap();

                // Invariant: TODO
                Some(Self { step_by, min, max })
            }

            pub fn current_attempt(&self) -> Option<$i> {
                // TODO: why can't this .unwrap() fail?
                Some(self.step_by.current().unwrap())
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<$i, $u> {
                // If we found a failing value, save it and move either back to
                // shrink magnitude, or to done
                if !current_attempt_passed {
                    let current_attempt = self.current_attempt().expect("Consecutive::current_attempt always returns Some");

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

                // If we've finished the consecutive phase, we're done
                if next_step_by.done() {
                    RangeInclusiveShrinkerSigned::Done(Done::new())
                }
                else {
                    // Progress to the next value
                    // TODO: invariants
                    RangeInclusiveShrinkerSigned::FlipSign(FlipSign::Consecutive(Self {
                        step_by: next_step_by,
                        min: self.min,
                        max: self.max
                    }))
                }
            }
        }
    )+};
}

consecutive! {
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize,
}
