use super::{binary_search::BinarySearch, done::Done, RangeInclusiveShrinkerUnsigned};

/// Implementation of the "Try simplest" phase of unsigned integer shrinking.
///
/// # Description
/// This phase tries the simplest value in the shrinking range immediately.
///
/// # Next phase
/// If the simplest value is found to be failing, shrinking ends immediately.
/// Otherwise, the simplest value isn't (always) failing, so we move on to the
/// next step, binary search.
///
/// # Goal
/// The goal behind this step is to waste no time trying more complicated values
/// if the simplest value will fail anyway.
//
// # Invariants
//
// ## The "Min" invariant
// `self.min < self.simplest_known_failing`
pub struct TrySimplest<T> {
    /// The minimum value in the shrinking range.
    min: T,

    /// The current simplest known failing value. Not needed for this phase
    /// itself, but necessary to know for future phases.
    simplest_known_failing: T,
}

macro_rules! try_simplest {
    ($($t: ty),+$(,)?) => {$(
        impl TrySimplest<$t> {
            /// Constructs a new [`TrySimplest`].
            ///
            /// # Panics
            /// if the invariant `min < simplest_known_failing` is not true.
            pub fn new(min: $t, simplest_known_failing: $t) -> Self {
                assert!(min < simplest_known_failing);

                // Invariant: "Min" is upheld by assertion above
                Self { min, simplest_known_failing }
            }

            pub fn current_attempt(&self) -> Option<$t> {
                Some(self.min)
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerUnsigned<$t> {
                // If the min value failed, we're done shrinking
                if !current_attempt_passed {
                    return RangeInclusiveShrinkerUnsigned::Done(Done::new());
                }

                // This addition can't overflow because the "Min" invariant
                // guarantees that `self.min` is at least 1 less than
                // `self.simplest_known_failing`
                let low = self.min + 1;
                let high = self.simplest_known_failing;

                // If there's nothing lower than the simplest known
                // failing value to try, we're done shrinking
                if low == high {
                    return RangeInclusiveShrinkerUnsigned::Done(Done::new());
                }

                // Otherwise, move on to binary search
                // Invariant: we must ensure that `min <= low < high` is true.
                // We know it will be, because low is `min + 1`, and the "Min"
                // invariant guarantees that `min < simplest_known_failing`,
                // which implies `low - 1 < high`, which implies `low <= high`,
                // and we've checked and diverged if `low == high`.
                RangeInclusiveShrinkerUnsigned::BinarySearch(BinarySearch::<$t>::new(low, high, self.min))
            }
        }
    )+};
}

try_simplest! { u8, u16, u32, u64, u128, usize }
