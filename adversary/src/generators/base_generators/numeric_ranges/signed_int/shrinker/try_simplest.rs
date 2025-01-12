use super::{done::Done, RangeInclusiveShrinkerSigned};

/// Implementation of the "Try simplest" phase of signed integer shrinking.
///
/// # Description
/// This phase tries the simplest value in the shrinking range immediately.
///
/// # Next phase
/// If the simplest value is found to be failing, shrinking ends immediately.
/// Otherwise, the simplest value isn't (always) failing, so we move on to the
/// next step, shrink magnitude.
///
/// # Goal
/// The goal behind this step is to waste no time trying more complicated values
/// if the simplest value will fail anyway.
//
// # Invariants
//
// ## The "Valid ordering" invariant
// `self.min <= self.simplest_known_failing <= self.max`
pub struct TrySimplest<T> {
    /// The minimum value in the shrinking range.
    min: T,

    /// The maximum value in the shrinking range.
    max: T,

    /// The current simplest known failing value. Not needed for this phase
    /// itself, but necessary to know for future phases.
    #[expect(
        unused,
        reason = "will be used once transitions to other phases are implemented"
    )]
    simplest_known_failing: T,
}

macro_rules! try_simplest {
    ($($t: ty),+$(,)?) => {$(
        impl TrySimplest<$t> {
            /// Constructs a new [`TrySimplest`].
            ///
            /// # Panics
            /// if the invariant `min <= simplest_known_failing <= max` is not
            /// true.
            pub fn new(simplest_known_failing: $t, (min, max): ($t, $t)) -> Self {
                // TODO: sweep through `assert!`s and make most of them be
                // `debug_assert!`s (unless truly they could actually panic in
                // the absence of a library bug - in which case, it should for
                // sure have a corresponding message with it)
                debug_assert!(min <= simplest_known_failing && simplest_known_failing <= max);

                // Invariant: "Valid ordering" must be upheld by the caller, and
                // is checked with the assertions above.
                Self { min, max, simplest_known_failing }
            }

            pub fn current_attempt(&self) -> Option<$t> {
                Some(RangeInclusiveShrinkerSigned::<$t>::simplest_in_range(self.min, self.max))
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<$t> {
                // If the simplest value failed, we're done shrinking
                if !current_attempt_passed {
                    return RangeInclusiveShrinkerSigned::Done(Done::new());
                }

                todo!()
            }
        }
    )+};
}

try_simplest! { i8, i16, i32, i64, i128, isize }
