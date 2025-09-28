use std::marker::PhantomData;

use super::{RangeInclusiveShrinkerSigned, done::Done, shrink_magnitude::ShrinkMagnitude};

/// Implementation of the "Flip sign" phase of signed integer shrinking.
///
/// # Description
/// This phase attempts to find a simpler failing value with the opposite sign
/// of the current simplest failing value.
///
/// # Next phase
/// If a simpler failing value with the opposite sign is found, we move back to
/// the "Shrink magnitude" phase to continue shrinking with the new sign.
/// Otherwise, we move to the "Done" phase.
///
/// # Goal
/// The goal is to find the simplest possible failing value, which might have
/// a different sign than the current one.
pub struct FlipSign<I, U> {
    /// The minimum value in the shrinking range.
    min: I,

    /// The maximum value in the shrinking range.
    max: I,

    /// The current simplest known failing value.
    simplest_known_failing: I,

    /// Whether we have already tried the flipped value.
    have_tried_flipped: bool,

    /// Phantom data for the unused type parameter.
    _phantom: PhantomData<U>,
}

macro_rules! flip_sign {
    ($($i:ty = $u:ty),+$(,)?) => {$(
        const _: () = assert!(size_of::<$i>() == size_of::<$u>());

        impl FlipSign<$i, $u> {
            /// Constructs a new [`FlipSign`].
            ///
            /// # Panics
            /// if the invariant `min <= simplest_known_failing <= max` is not
            /// true.
            pub fn new(simplest_known_failing: $i, (min, max): ($i, $i)) -> Self {
                debug_assert!(min <= simplest_known_failing && simplest_known_failing <= max);

                Self {
                    min,
                    max,
                    simplest_known_failing,
                    have_tried_flipped: false,
                    _phantom: PhantomData,
                }
            }

            pub fn current_attempt(&self) -> Option<$i> {
                if self.have_tried_flipped {
                    None
                } else {
                    RangeInclusiveShrinkerSigned::<$i, $u>::flipped_simpler(
                        self.simplest_known_failing,
                        (self.min, self.max)
                    )
                }
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<$i, $u> {
                if self.have_tried_flipped {
                    // We've already tried the flipped value, so we're done
                    RangeInclusiveShrinkerSigned::Done(Done::new())
                } else if !current_attempt_passed {
                    // The flipped value failed, so it's our new simplest known failing value
                    let flipped_value = self.current_attempt().expect("current_attempt returns Some when have_tried_flipped is false");
                    RangeInclusiveShrinkerSigned::ShrinkMagnitude(ShrinkMagnitude::<$i, $u>::new(
                        flipped_value,
                        (self.min, self.max)
                    ))
                } else {
                    // The flipped value passed, so we're done
                    RangeInclusiveShrinkerSigned::Done(Done::new())
                }
            }
        }
    )+};
}

flip_sign! {
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize,
}
