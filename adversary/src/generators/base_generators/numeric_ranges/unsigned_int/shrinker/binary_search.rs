use super::{
    consecutive::{Consecutive, CONSEC_COUNT},
    spread_out::{SpreadOut, SPREAD_COUNT},
    RangeInclusiveShrinkerUnsigned,
};

/// Implementation of the "Binary search" phase of unsigned integer shrinking.
///
/// # Description
/// This phase keeps track of a "high" and "low" value, where `high` is the
/// smallest value observed failing, and `low` is the largest value we haven't
/// yet ruled out as too low. It will repeatedly try a value halway between
/// `high` and `low`, updating either `high` or `low` as we gain new information
/// and zero in on some threshold between passing and failing values.
///
/// This phase makes the assumption that the input space looks generally like
/// "passing values up to some threshold, then failing values after that
/// threshold". That assumption won't always be correct, but it's very effective
/// at quickly reducing the smallest known failing value when it is. When it
/// isn't, no big deal - hopefully other phases will fare better.
///
/// # Next phase
/// Once there are no values between `low` and `high` to try, we will usually
/// move on to the "Spread out" phase. The exception to this is that if there's
/// not enough input space left to do both "Spread out" *and* "Consecutive"
/// without overlap, we skip "Spread out" altogether and go right on to
/// "Consecutive".
///
/// # Goal
/// The goal of this step is to quickly reduce the simplest known failing value
/// as much as possible, especially when we have a long run of failing values
/// just less than our current simplest known failing value.
//
// # Invariants
//
// ## The "Valid range" invariant
// `min <= self.low < self.high`
#[derive(Debug, Clone)]
pub struct BinarySearch<T> {
    /// The lower value in the binary search range, which hasn't yet been
    /// observed passing.
    low: T,

    /// The upper value in the binary search range, known to be failing.
    high: T,

    /// The minimum value in the shrinking range. Not needed for this phase
    /// itself, but necessary to know for future phases (and the transition to
    /// them).
    min: T,
}

macro_rules! binary_search {
    ($($t: ty),+$(,)?) => {$(
        impl BinarySearch<$t> {
            /// Constructs a new [`BinarySearch`].
            ///
            /// # Panics
            /// if the invariant `min <= low < high` is not true.
            pub fn new(low: $t, high: $t, min: $t) -> Self {
                assert!(min <= low);
                assert!(low < high);

                // Invariant: "Valid range" must be upheld by the caller, and is
                // checked with the assertions above.
                Self { low, high, min }
            }

            pub fn current_attempt(&self) -> Option<$t> {
                // `self.high - self.low` can't overflow because the "Valid
                // range" invariant guarantees `self.low < self.high`. The
                // addition can't overflow because the overall result cannot
                // exceed `self.high`, which clearly is not more than the max
                // representable value for $t
                Some((self.high - self.low) / 2 + self.low)
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerUnsigned<$t> {
                let current_attempt = self.current_attempt().expect("BinarySearch::current_attempt always returns Some");

                // Compute the new high and low boundary in our binary search
                let mut new_low = self.low;
                let mut new_high = self.high;
                if current_attempt_passed {
                    // The value returned by `current_attempt` is always less
                    // than `self.high`, so adding 1 will at most be `self.high`
                    // and therefore cannot overflow
                    new_low = current_attempt + 1;
                } else {
                    new_high = current_attempt;
                }

                // If we've finished binary searching, move on to the next phase
                if new_low == new_high {
                    // We go down to `min + 1` at lowest instead of `min`
                    // because this phase assumes the "Try simplest" phase has
                    // already run, and if `min` were failing we wouldn't be
                    // here.
                    //
                    // `self.min + 1` can't overflow because the "Valid range"
                    // invariant guarantees `self.min < self.high`, so adding 1
                    // will at most be `self.high`.
                    //
                    // Saturation subtraction is correct because we take the max
                    // with something greater than zero, so if it saturates we
                    // still always correctly use `self.min + 1`.
                    let consecutive_start = new_high.saturating_sub(CONSEC_COUNT.into()).max(self.min + 1);

                    // If there's not enough input space to do spread out *and*
                    // consecutive without overlap, skip spread out altogether
                    return if consecutive_start.saturating_sub(SPREAD_COUNT.into()) < self.min + 1 {
                        // Invariant: Consecutive::new requires `min <
                        // <$t>::MAX`. this is guaranteed by the "Valid range"
                        // invariant, since `self.min < self.high` implies
                        // `self.min < <$t>::MAX`.
                        RangeInclusiveShrinkerUnsigned::Consecutive(Consecutive::<$t>::new(new_high, self.min))
                    }
                    // Otherwise, continue on to spread out
                    else {
                        // Invariant: SpreadOut::new requires that `self.min +
                        // CONSEC_COUNT + SPREAD_COUNT < new_high` (after
                        // plugging in our arguments). Rearranged, that's:
                        // `self.min < new_high - CONSEC_COUNT - SPREAD_COUNT`,
                        // which is impossible since we're in the `else` branch
                        // of the condition (almost) `new_high - CONSEC_COUNT -
                        // SPREAD_COUNT <= self.min`. I say "almost" because
                        // there's actually some saturating math and a .max(...)
                        // thrown in there - I'll leave it as an exercise to the
                        // reader to figure out why those actually are not a
                        // problem :) (it will also make it clear why the below
                        // assert is necessary)
                        const { assert!(SPREAD_COUNT > 0) };
                        RangeInclusiveShrinkerUnsigned::SpreadOut(SpreadOut::<$t>::new(new_high, self.min))
                    };
                }

                // Invariant: the "Valid range" invariant is upheld because
                // `new_low` and `new_high` both cannot be outside the previous
                // `low` and `high` range, so are therefore both still greater
                // than or equal to `self.min`. Additionally, exactly one of the
                // following two possibilities has happened:
                // 1. `new_high` is the old `self.high`, and `new_low` is now
                // something less than or equal to `new_high`
                // 2. `new_low` is the old `self.low`, and `new_high` is now
                // something greater than or equal to `new_low`
                // In either case, if `new_high` and `new_low` are equal, we
                // would have diverged in the branch above. Therefore, we know
                // at this point that new_low < new_high, satisfying the other
                // requirement of the "Valid range" invariant.
                RangeInclusiveShrinkerUnsigned::BinarySearch(Self::new(new_low, new_high, self.min))
            }
        }
    )+};
}

binary_search! { u8, u16, u32, u64, u128, usize }
