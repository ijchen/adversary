use crate::shrinker::Shrinker as _;

use super::{
    super::super::unsigned_int::RangeInclusiveShrinkerUnsigned, RangeInclusiveShrinkerSigned,
    flip_sign::FlipSign,
};

/// Implementation of the "Shrink magnitude" phase of signed integer shrinking.
///
/// # Description
/// This phase tries the shrink the magnitude (absolute value) of the current
/// simplest failing value while maintaining its sign.
///
/// # Next phase
/// When magnitude shrinking is complete, we move to the "Flip sign" phase to
/// try finding a simpler failing value with the opposite sign.
///
/// # Goal
/// The goal is to reduce the magnitude (absolute value) of the current simplest
/// failing value while maintaining its sign, making it simpler.
//
// # Invariants
//
// ## The "Valid ordering" invariant
// `self.min <= self.simplest_known_failing <= self.max`
//
// ## The "Magnitude shrinker not done" invariant
// `self.magnitude_shrinker.current_attempt().is_some()`
pub struct ShrinkMagnitude<I, U> {
    /// The minimum value in the shrinking range.
    min: I,

    /// The maximum value in the shrinking range.
    max: I,

    /// The current simplest known failing value.
    simplest_known_failing: I,

    /// A shrinker for the magnitude of the simplest known failing value.
    magnitude_shrinker: RangeInclusiveShrinkerUnsigned<U>,

    /// Whether or not we have found a simpler known failing value than the one
    /// we were given.
    have_made_progress: bool,
}

macro_rules! shrink_magnitude {
    ($($i:ty = $u:ty),+$(,)?) => {$(
        const _: () = assert!(size_of::<$i>() == size_of::<$u>());

        impl ShrinkMagnitude<$i, $u> {
            /// Constructs a new [`ShrinkMagnitude`].
            ///
            /// # Panics
            /// if any of the following invariants are not true:
            /// - `min <= simplest_known_failing <= max`
            /// - `simplest_known_failing` is not the simplest value in the
            ///   range min..=max
            pub fn new(simplest_known_failing: $i, (min, max): ($i, $i)) -> Self {
                assert!(min <= simplest_known_failing && simplest_known_failing <= max);

                let simplest_in_range = RangeInclusiveShrinkerSigned::<$i, $u>::simplest_in_range(min, max);
                assert_ne!(simplest_known_failing, simplest_in_range);

                let magnitude_shrinker = RangeInclusiveShrinkerUnsigned::<$u>::new(
                    simplest_in_range.unsigned_abs(),
                    simplest_known_failing.unsigned_abs()
                );
                assert!(magnitude_shrinker.current_attempt().is_some());

                // Invariant: "Valid ordering" must be upheld by the caller, and
                // is checked with assertions above. "Magnitude shrinker not
                // done" is true because `RangeInclusiveShrinkerUnsigned` will
                // only start out in a "Done" state if it's two arguments are
                // equal - which we've ensured is not the case.
                Self {
                    min,
                    max,
                    simplest_known_failing,
                    magnitude_shrinker,
                    have_made_progress: false,
                }
            }

            pub fn current_attempt(&self) -> Option<$i> {
                let magnitude = self.magnitude_shrinker.current_attempt().expect("`ShrinkMagnitude` 'Magnitude shrinker not done' invariant violated");
                Some(if self.simplest_known_failing >= 0 {
                    // This conversion cannot fail because magnitude is derived from
                    // the absolute value of a signed integer, so it's within the
                    // valid range for the signed type.
                    <$i>::try_from(magnitude).unwrap()
                } else {
                    // This conversion cannot fail because magnitude is derived from
                    // the absolute value of a signed integer, so it's within the
                    // valid range for the signed type. We use checked_sub_unsigned
                    // instead of casting and negating because <$i>::MIN.unsigned_abs() > <$i>::MAX
                    <$i>::checked_sub_unsigned(0, magnitude).unwrap()
                })
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<$i, $u> {
                // If the current attempt failed, we have a new simplest known
                // failing value
                let new_simplest_known_failing = if !current_attempt_passed {
                    self.current_attempt().expect("`ShrinkMagnitude::current_attempt` always returns `Some`")
                } else {
                    self.simplest_known_failing
                };

                // Update the magnitude shrinker
                let mut new_magnitude_shrinker = self.magnitude_shrinker.clone();
                new_magnitude_shrinker.update(current_attempt_passed);

                // If there's more magnitude shrinking to do, keep going
                if new_magnitude_shrinker.current_attempt().is_some() {
                    // All invariants are maintained: the new simplest_known_failing is
                    // either the original (if current attempt passed) or the current
                    // attempt (if it failed), both of which are within the valid range.
                    return RangeInclusiveShrinkerSigned::ShrinkMagnitude(Self {
                        min: self.min,
                        max: self.max,
                        simplest_known_failing: new_simplest_known_failing,
                        magnitude_shrinker: new_magnitude_shrinker,
                        have_made_progress: self.have_made_progress || !current_attempt_passed,
                    });
                }

                // Move to flip sign phase to try the opposite sign
                RangeInclusiveShrinkerSigned::FlipSign(FlipSign::<$i, $u>::new(
                    new_simplest_known_failing,
                    (self.min, self.max)
                ))
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
