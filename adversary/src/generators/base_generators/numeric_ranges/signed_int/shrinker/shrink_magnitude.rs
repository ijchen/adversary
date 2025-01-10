use crate::shrinker::Shrinker;

use super::super::super::unsigned_int::RangeInclusiveShrinkerUnsigned;

use super::done::Done;
use super::flip_sign::FlipSign;
use super::RangeInclusiveShrinkerSigned;

/// Implementation of the "Shrink magnitude" phase of signed integer shrinking.
///
/// # Description
/// TODO: update this description
/// This phase attempts every value in a consecutive run of values immediately
/// under the current simplest known failing value. Typically, it will try
/// exactly [`CONSEC_COUNT`] values from `simplest_known_failing - CONSEC_COUNT`
/// up to (but not including `simplest_known_failing`. The exception to this is
/// that the run of values will never start before `min + 1`, where `min` is the
/// minimum value in the shrinking range.
///
/// # Next phase
/// TODO: update this description
/// Once all [`CONSEC_COUNT`] values have been attempted (and none of them
/// failed), we will finish shrinking and move on to the "Done" phase. If any
/// value fails, we will immediately jump back to binary search again, unless
/// this run of consecutive values started at `min + 1` - in that case, we've
/// already checked everything between our newest failing value and `min`, so we
/// just jump right to "Done".
///
/// # Goal
/// TODO: update this description
/// The goal behind this step is to try a large run of consecutive values just
/// less than the current simplest known failing value, with the hope that we
/// will be able to recognize and jump past any relatively small gaps of passing
/// inputs between the current simplest known failing value and simpler failing
/// values.
//
// # Invariants
//
// ## The "Valid range" invariant
// `self.min < self.max`
//
// ## The "In range" invariant
// `self.min <= self.simplest_known_failing <= self.max`
pub struct ShrinkMagnitude<I, U> {
    // TODO: field documentation
    magnitude_shrinker: RangeInclusiveShrinkerUnsigned<U>,

    /// The current simplest known failing value.
    simplest_known_failing: I,

    /// The minimum value in the shrinking range. Not needed for this phase
    /// itself, but necessary to know for future phases.
    min: I,

    /// The maximum value in the shrinking range. Not needed for this phase
    /// itself, but necessary to know for future phases.
    max: I,
}

macro_rules! shrink_magnitude {
    ($($i:ty = $u:ty),+$(,)?) => {$(
        const _: () = assert!(size_of::<$i>() == size_of::<$u>());

        impl ShrinkMagnitude<$i, $u> {
            /// Clever way to take the median of three numbers
            fn median_of_three(a: $i, b: $i, c: $i) -> $i {
                <$i>::max(<$i>::min(a, b), <$i>::min(<$i>::max(a, b), c))
            }

            /// Constructs a new [`ShrinkMagnitude`].
            ///
            /// # Panics
            /// if any of the following invariants are not true:
            /// - `min < max`
            /// - `min <= simplest_known_failing <= max`
            pub fn new(simplest_known_failing: $i, min: $i, max: $i) -> Self {
                assert!(min < max);
                assert!((min..=max).contains(&simplest_known_failing));

                let min_magnitude = Self::median_of_three(min, max, 0).unsigned_abs();

                // TODO: invariants
                let magnitude_shrinker = RangeInclusiveShrinkerUnsigned::<$u>::new(min_magnitude, simplest_known_failing.unsigned_abs());

                // Both invariants must be upheld by the caller, and are checked
                // with assertions above.
                Self { magnitude_shrinker, simplest_known_failing, min, max }
            }

            pub fn current_attempt(&self) -> Option<$i> {
                // TODO: closure relies on some invariant about the unsigned
                // shrinker representing only the magnitude, and the sign should
                // be made to match self.simplest_known_failing, and some stuff
                // about overflow and the try_from unwrap
                self.magnitude_shrinker.current_attempt().map(|n| <$i>::try_from(n).unwrap() * self.simplest_known_failing.signum())
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<$i, $u> {
                let mut next_shrinker = self.magnitude_shrinker.clone();

                next_shrinker.update(current_attempt_passed);

                // If the current attempt failed, it's a new simplest known
                // failing value
                let simplest_known_failing = if !current_attempt_passed {
                    self.current_attempt().unwrap()
                } else {
                    self.simplest_known_failing
                };

                // If the magnitude shrinker is done shrinking, we are done with
                // this phase
                if next_shrinker.current_attempt().is_none() {
                    return if let Some(flip_sign) = FlipSign::<$i>::new(self.min, self.max, simplest_known_failing) {
                        RangeInclusiveShrinkerSigned::FlipSign(flip_sign)
                    } else {
                        RangeInclusiveShrinkerSigned::Done(Done::new())
                    };
                }

                // TODO: invariants (and maybe make this a function call instead
                // of just literal construction)
                RangeInclusiveShrinkerSigned::ShrinkMagnitude(Self {
                    magnitude_shrinker: next_shrinker,
                    simplest_known_failing: self.simplest_known_failing,
                    min: self.min,
                    max: self.max,
                })
            }
        }
    )+};
}

shrink_magnitude! {
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize,
}
