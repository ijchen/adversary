use super::{
    binary_search::BinarySearch,
    consecutive::{Consecutive, CONSEC_COUNT},
    done::Done,
    RangeInclusiveShrinkerUnsigned,
};

/// The number of spread out values to try during the "Spread out" shrinking
/// phase.
pub const SPREAD_COUNT: u8 = 12;

/// Implementation of the "Spread out" phase of unsigned integer shrinking.
///
/// # Description
/// This phase selects a fixed number ([`SPREAD_COUNT`]) of integers,
/// approximately evenly spaced out between the minimum value in the range and
/// the current simplest known failing value. This range is divided into
/// equal-sized sections, and the center of that section is used as an input.
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
/// Once all [`SPREAD_COUNT`] values have been attempted (and none of them
/// failed), we will move on to the "Consecutive" phase. If any value fails, we
/// will immediately jump back to binary search again, unless the failing value
/// is `min` or `min + 1`. If one of those failed, there's no point binary
/// searching, we just jump right to "Done".
///
/// # Goal
/// The goal behind this phase is to sample many spread out points in the
/// remaining input space, with the hope that we will catch any "pockets" of
/// failing values that binary search missed.
//
// # Invariants
//
// ## The "No overlap" invariant
// `self.min + CONSEC_COUNT + SPREAD_COUNT < self.simplest_known_failing`
//
// ## The "Valid index" invariant
// `index < SPREAD_COUNT`
#[derive(Debug, Clone)]
pub struct SpreadOut<T> {
    /// The zero-based index of which "chunk" of the input space we're currently
    /// searching. Will be between 0 (inclusive) and [`SPREAD_COUNT`]
    /// (exclusive).
    index: u8,

    /// The current simplest known failing value.
    simplest_known_failing: T,

    /// The minimum value in the shrinking range.
    min: T,
}

macro_rules! spread_out {
    ($($t: ty),+$(,)?) => {$(
        impl SpreadOut<$t> {
            /// Constructs a new [`SpreadOut`].
            ///
            /// # Panics
            /// if the invariant `min + CONSEC_COUNT + SPREAD_COUNT <
            /// simplest_known_failing` is not true.
            pub fn new(simplest_known_failing: $t, min: $t) -> Self {
                assert!(
                    // Saturating add is fine, because if we saturate, the
                    // comparison is guaranteed to fail anyway (<$t>::MAX will
                    // not be less than any possible `simplest_known_failing`)
                    min
                        .saturating_add(<$t>::from(CONSEC_COUNT))
                        .saturating_add(<$t>::from(SPREAD_COUNT))
                    < simplest_known_failing
                );
                const { assert!(0 < SPREAD_COUNT) };

                // Invariant: "No overlap" must be upheld by the caller, and is
                // checked with the first assertion above. "Valid index" is
                // upheld by the above const-assert that `0 < SPREAD_COUNT`.
                Self::with_index(0, simplest_known_failing, min)
            }

            /// Constructs a new [`SpreadOut`] with the given `index`.
            ///
            /// # Panics
            /// if any of the following invariants are not true:
            /// - `min + CONSEC_COUNT + SPREAD_COUNT < simplest_known_failing`
            /// - `0 <= index < SPREAD_COUNT`
            fn with_index(index: u8, simplest_known_failing: $t, min: $t) -> Self {
                assert!(
                    // Saturating add is fine, because if we saturate, the
                    // comparison is guaranteed to fail anyway (<$t>::MAX will
                    // not be less than any possible `simplest_known_failing`)
                    min
                        .saturating_add(<$t>::from(CONSEC_COUNT))
                        .saturating_add(<$t>::from(SPREAD_COUNT))
                    < simplest_known_failing
                );
                assert!(index < SPREAD_COUNT);

                // Both invariants must be upheld by the caller, and are checked
                // with assertions above.
                Self { index, simplest_known_failing, min }
            }

            pub fn current_attempt(&self) -> Option<$t> {
                // Calculate the ends (both inclusive) of the range of values to
                // spread out within
                //
                // This addition can't overflow because the "No overlap"
                // invariant guarantees that `self.min` is at least 1 less than
                // `self.simplest_known_failing`
                let lowest = self.min + 1;
                // Neither of these subtractions can overflow, because the "No
                // overlap" invariant guarantees that
                // `self.simplest_known_failing - CONSEC_COUNT - 1 >= self.min +
                // SPREAD_COUNT`. This proves `highest` must be greater than or
                // equal to `self.min + SPREAD_COUNT` (clearly not less than 0).
                let highest = self.simplest_known_failing - <$t>::from(CONSEC_COUNT) - 1;
                // The subtraction can't overflow because we know `highest >=
                // self.min + SPREAD_COUNT` (see comment on `highest`), which
                // implies that `highest - self.min >= SPREAD_COUNT >= 0`.
                // Note that `highest - self.min` is just an alternative way to
                // write `highest - lowest + 1`, which is a little easier to
                // understand intuitively is the number of values in the range.
                let num_of_values_in_range = highest - self.min;

                const { assert!(SPREAD_COUNT * 2 <= u8::MAX) };
                let start = lowest + num_of_values_in_range / <$t>::from(SPREAD_COUNT * 2);
                // We know `offset` will never be 0, since we know that
                // `num_of_values_in_range >= SPREAD_COUNT` (see comment on
                // `num_of_values_in_range`). This is important because if
                // `offset` were rounded down to 0, we'd just try the same value
                // over and over again, `SPREAD_COUNT` times in a row.
                let offset = num_of_values_in_range / <$t>::from(SPREAD_COUNT) * <$t>::from(self.index);

                Some(start + offset)
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerUnsigned<$t> {
                // If we found a failing value, save it and move either back to
                // binary search, or to done
                if !current_attempt_passed {
                    let current_attempt = self.current_attempt().expect("SpreadOut::current_attempt always returns Some");

                    // If the failing value we found is <= min + 1, there's no
                    // point binary searching - we're done.
                    return if current_attempt <= self.min + 1 {
                        RangeInclusiveShrinkerUnsigned::Done(Done::new())
                    }
                    // Otherwise, there's more values to try between min and the
                    // new simplest failing input we just found, so start back
                    // at the binary search phase with our new information
                    else {
                        // Invariant: BinarySearch::new requires that `min <=
                        // low < high`. Plugging in our arguments, that's:
                        // `self.min <= self.min + 1 < current_attempt`
                        // `self.min <= self.min + 1` is trivially true.
                        // `self.min + 1 < current_attempt` is true because we
                        // are in an else branch for the condition
                        // `current_attempt <= self.min + 1`.
                        RangeInclusiveShrinkerUnsigned::BinarySearch(BinarySearch::<$t>::new(self.min + 1, current_attempt, self.min))
                    };
                }

                // If we've finished the spread out search, move on to the
                // consecutive phase
                if self.index + 1 == SPREAD_COUNT {
                    // Invariant: Consecutive::new requires `min < <$t>::MAX`.
                    // this is guaranteed by the "No overlap" invariant, since
                    // `self.min < self.simplest_known_failing` implies
                    // `self.min < <$t>::MAX`.
                    return RangeInclusiveShrinkerUnsigned::Consecutive(Consecutive::<$t>::new(
                        self.simplest_known_failing,
                        self.min,
                    ));
                }

                // Progress to the next value
                //
                // Invariant: "No overlap" is maintained from `self`, and "Valid
                // index" is upheld by diverging above when `self.index + 1 ==
                // SPREAD_COUNT`
                RangeInclusiveShrinkerUnsigned::SpreadOut(Self::with_index(self.index + 1, self.simplest_known_failing, self.min))
            }
        }
    )+};
}

spread_out! { u8, u16, u32, u64, u128, usize }
