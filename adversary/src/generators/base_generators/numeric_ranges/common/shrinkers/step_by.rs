//! Home of the [`StepBy`] shrinker for numeric ranges.

/// A shrinker for numeric ranges which attempts every value from some starting
/// value up to (and potentially including) some stopping value, stepping by
/// some value.
//
// # Invariants
//
// ## The "terminating" invariant
// Adding `step_by` to `current` moves towards `stop_at`. More precisely:
// `current < stop_at && step_by > 0 || current > stop_at && step_by < 0`
#[derive(Debug, Clone)]
pub struct StepBy<T> {
    /// The current value to be attempted.
    current: T,

    /// The last value to attempt (inclusive).
    stop_at: T,

    /// How much to step by between values.
    step_by: T,

    /// Whether or not we're done yielding values.
    done: bool,
}

macro_rules! step_by {
    ($($t:ty),+$(,)?) => {$(
        impl StepBy<$t> {
            /// Constructs a new [`StepBy`], or returns [`None`] if the given
            /// parameters would give a [`StepBy`] that never terminates.
            ///
            /// In this context, "never terminates" means adding step_by to
            /// start will either move away from stop_at or not move at all,
            /// instead of moving towards it. Concretely, this means you must
            /// ensure that `start <= stop_at && step_by > 0 || start >= stop_at
            /// && step_by < 0`. If that condition is not true, this function
            /// returns [`None`].
            pub fn new(start: $t, stop_at: $t, step_by: $t) -> Option<Self> {
                // Invariant: "terminating" is ensured before producing a `Self`
                (start <= stop_at && step_by > 0 || start >= stop_at && step_by < 0).then(|| Self {
                    current: start,
                    stop_at,
                    step_by,
                    done: false,
                })
            }

            /// Returns whether or not we're done.
            pub fn done(&self) -> bool {
                self.done
            }

            /// Gets the current value to attempt, or [`None`] if we're done.
            pub fn current(&self) -> Option<$t> {
                (!self.done).then_some(self.current)
            }

            /// Gets a [`StepBy`] that is identical to self, but progressed to
            /// the next value (or put in a "done" state if we would have
            /// progressed past `self.stop_at`).
            pub fn next(&self) -> Self {
                let mut updated = self.clone();

                // If we're already done, no need to update anything
                if self.done {
                    return updated;
                }

                // Compute the next value (or if it would overflow, we're done)
                let Some(next) = self.current.checked_add(self.step_by) else {
                    updated.done = true;
                    return updated;
                };

                // If we've moved past the last value, we're done
                if self.step_by > 0 && next > self.stop_at || self.step_by < 0 && next < self.stop_at {
                    updated.done = true;
                    return updated;
                }

                // Invariant: we've just checked and diverged if "terminating"
                // wouldn't be true
                Self::new(next, self.stop_at, self.step_by).unwrap()
            }
        }
    )+};
}

step_by! {
    // TODO: figure out if I want to integrate this into unsigned, or just scrap
    // it and implement stuff manually in signed
    // u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
}
