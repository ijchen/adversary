use super::{binary_search::BinarySearch, done::Done, RangeInclusiveShrinkerUnsigned};

/// The number of consecutive values in a row to try during the "Consecutive"
/// shrinking phase.
pub const CONSEC_COUNT: u8 = 100;

/// Implementation of the "Consecutive" phase of unsigned integer shrinking.
///
/// # Description
/// This phase attempts every value in a consecutive run of values immediately
/// under the current simplest known failing value. Typically, it will try
/// exactly [`CONSEC_COUNT`] values from `simplest_known_failing - CONSEC_COUNT`
/// up to (but not including `simplest_known_failing`. The exception to this is
/// that the run of values will never start before `min + 1`, where `min` is the
/// minimum value in the shrinking range.
///
/// # Next phase
/// Once all [`CONSEC_COUNT`] values have been attempted (and none of them
/// failed), we will finish shrinking and move on to the "Done" phase. If any
/// value fails, we will immediately jump back to binary search again, unless
/// this run of consecutive values started at `min + 1` - in that case, we've
/// already checked everything between our newest failing value and `min`, so we
/// just jump right to "Done".
///
/// # Goal
/// The goal behind this step is to try a large run of consecutive values just
/// less than the current simplest known failing value, with the hope that we
/// will be able to recognize and jump past any relatively small gaps of passing
/// inputs between the current simplest known failing value and simpler failing
/// values.
//
// # Invariants
//
// ## The "Min is not max" invariant
// `self.min < T::MAX`
//
// ## The "Within consecutive run" invariant
// `start <= self.current < self.simplest_known_failing`
// where `start = max(self.min + 1, self.simplest_known_failing - CONSEC_COUNT)`
#[derive(Debug, Clone)]
pub struct Consecutive<T> {
    /// The current value we're attempting.
    current: T,

    /// The current simplest known failing value.
    simplest_known_failing: T,

    /// The minimum value in the shrinking range. Not needed for this phase
    /// itself, but necessary to know for future phases (and the transition to
    /// them).
    min: T,
}

macro_rules! consecutive {
    ($($t: ty),+$(,)?) => {$(
        impl Consecutive<$t> {
            /// Computes the starting value of a consecutive run. This will be
            /// either `simplest_known_failing - CONSEC_COUNT` or `min + 1`,
            /// whichever is larger.
            ///
            /// # Panics
            /// if `min == <$t>::MAX`
            fn starting_value(simplest_known_failing: $t, min: $t) -> $t {
                assert_ne!(min, <$t>::MAX);

                <$t>::max(simplest_known_failing.saturating_sub(CONSEC_COUNT.into()), min + 1)
            }

            /// Constructs a new [`Consecutive`].
            ///
            /// # Panics
            /// if the invariant `min < <$t>::MAX` is not true.
            pub fn new(simplest_known_failing: $t, min: $t) -> Self {
                assert_ne!(min, <$t>::MAX);

                let current = Self::starting_value(simplest_known_failing, min);

                // Invariant: "Min is not max" must be upheld by the caller, and
                // is checked with the assertion above.
                Self { current, simplest_known_failing, min }
            }

            /// Constructs a new [`Consecutive`] with the given `current`.
            ///
            /// # Panics
            /// if any of the following invariants are not true:
            /// - `min < <$t>::MAX`
            /// - `start <= self.current < self.simplest_known_failing` where
            ///   `start = max(self.min + 1, self.simplest_known_failing -
            ///   CONSEC_COUNT)`
            fn with_current(current: $t, simplest_known_failing: $t, min: $t) -> Self {
                assert!(min < <$t>::MAX);
                assert!(Self::starting_value(simplest_known_failing, min) <= current);

                // Both invariants must be upheld by the caller, and are checked
                // with assertions above.
                Self { current, simplest_known_failing, min }
            }

            pub fn current_attempt(&self) -> Option<$t> {
                Some(self.current)
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerUnsigned<$t> {
                // If we found a failing value, save it and move either back to
                // binary search, or to done
                if !current_attempt_passed {
                    // If this consecutive run started at min + 1, that means we
                    // already checked everything between min and
                    // simplest_known_failing, so we're done.
                    //
                    // `self.min + 1` can't overflow because of the "Min is not
                    // max" invariant.
                    return if Self::starting_value(self.simplest_known_failing, self.min) == self.min + 1 {
                        RangeInclusiveShrinkerUnsigned::Done(Done::new())
                    }
                    // Otherwise, there's more values to try between min
                    // and the new simplest failing input we just found,
                    // so start back at the binary search phase with our
                    // new information
                    else {
                        // Invariant: BinarySearch::new requires us to ensure
                        // that `self.min <= self.min + 1 < self.current`.
                        // `self.min <= self.min + 1` is trivially true.
                        // `self.min + 1 < self.current` must be true because we
                        // are in the `else` branch of a condition that checks
                        // if the the consecutive run started at `self.min + 1`.
                        // If we didn't start at `self.min + 1`, we must have
                        // started at something greater than that, and therefore
                        // `self.current` must be greater than `self.min + 1`.
                        RangeInclusiveShrinkerUnsigned::BinarySearch(BinarySearch::<$t>::new(self.min + 1, self.current, self.min))
                    };
                }

                // If we've hit the simplest known failing input, we've
                // tried all values in our consecutive run - give up and
                // move to done
                //
                // `self.current + 1` can't overflow because the "Within
                // consecutive run" invariant guarantees that `self.current <
                // self.simplest_known_failing <= <$t>::MAX`
                if self.current + 1 == self.simplest_known_failing {
                    return RangeInclusiveShrinkerUnsigned::Done(Done::new());
                }

                // Continue through the consecutive range
                //
                // Invariant: "Min is not max" is maintained from `self`, and
                // "Within consecutive run" is upheld by diverging above when
                // `self.current + 1 == self.simplest_known_failing`.
                RangeInclusiveShrinkerUnsigned::Consecutive(Self::with_current(self.current + 1, self.simplest_known_failing, self.min))
            }
        }
    )+};
}

consecutive! { u8, u16, u32, u64, u128, usize }
