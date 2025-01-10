use super::{done::Done, shrink_magnitude::ShrinkMagnitude, RangeInclusiveShrinkerSigned};

/// Implementation of the "Try simplest" phase of signed integer shrinking.
///
/// # Description
/// This phase tries the simplest value in the shrinking range immediately.
///
/// # Next phase
/// If the simplest value is found to be failing, shrinking ends immediately.
/// Otherwise, the simplest value isn't (always) failing, so we move on to the
/// next step, "Shrink magnitude".
///
/// # Goal
/// The goal behind this step is to waste no time trying more complex values if
/// the simplest value will fail anyway.
//
// # Invariants
//
// ## The "Valid range" invariant
// `self.min < self.max`
//
// ## The "In range" invariant
// `self.min <= self.simplest_known_failing <= self.max`
pub struct TrySimplest<T> {
    /// The minimum value in the shrinking range.
    min: T,

    /// The maximum value in the shrinking range.
    max: T,

    /// The current simplest known failing value.
    simplest_known_failing: T,
}

macro_rules! try_simplest {
    ($($i:ty = $u:ty),+$(,)?) => {$(
        impl TrySimplest<$i> {
            /// Constructs a new [`TrySimplest`].
            ///
            /// # Panics
            /// if any of the following invariants are not true:
            /// - `min < max`
            /// - `min <= simplest_known_failing <= max`
            pub fn new(min: $i, max: $i, simplest_known_failing: $i) -> Self {
                assert!(min < max);
                assert!((min..=max).contains(&simplest_known_failing));

                // Both invariants must be upheld by the caller, and are checked
                // with assertions above.
                Self { min, max, simplest_known_failing }
            }

            pub fn current_attempt(&self) -> Option<$i> {
                Some(
                    // If zero is within the range of allowed values, that is the
                    // simplest.
                    if (self.min..=self.max).contains(&0) {
                        0
                    }
                    // The range is either entirely negative or entirely positive.
                    // Figure out which it is, and return the closest value to 0
                    else if self.simplest_known_failing < 0 {
                        // Range is entirely negative - max is closest to zero
                        self.max
                    }
                    else {
                        // Range is entirely positive - min is closest to zero
                        self.min
                    }
                )
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<$i, $u> {
                // If the simplest value failed, we're done shrinking
                if !current_attempt_passed {
                    RangeInclusiveShrinkerSigned::Done(Done::new())
                }
                else {
                    // Otherwise, move on to "Shrink magnitude"
                    // TODO: invariants
                    RangeInclusiveShrinkerSigned::ShrinkMagnitude(ShrinkMagnitude::<$i, $u>::new(self.simplest_known_failing, self.min, self.max))
                }
            }
        }
    )+};
}

try_simplest! {
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize,
}
