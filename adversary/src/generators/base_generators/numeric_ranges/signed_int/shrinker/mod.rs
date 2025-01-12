//! Implementation of the shrinking algorithm for signed integers.
//!
//! The shrinking is broken into multiple distinct "phases". Each phase is
//! implemented within its own submodule, for organization. The overall "main"
//! shrinker in this module just delegates to the submodules. For a high-level
//! overview of the phases, see the documentation on
//! [`RangeInclusiveShrinkerSigned`]. For more details on each individual phase,
//! see the documentation in their respective modules.

mod done;
mod try_simplest;

use done::Done;
use try_simplest::TrySimplest;

use crate::{report::Observation, shrinker::Shrinker};

/// The shrinker implementation for signed integer range
/// [`InputGenerator`](crate::InputGenerator)s.
///
/// Shrinking is split up into multiple distinct "phases". The bulk of the logic
/// is actually contained within submodules for each phase - this enum really
/// just organizes them all together.
///
/// # How sign effects simplicity
/// It's worth discussing how sign (positive vs. negative) effects how "simple"
/// a value is considered. Of course, simplicity is both subjective and
/// context-dependent. Fortunately, this isn't really a correctness issue - at
/// worst, poor shrinking will be less helpful to the developer debugging a
/// failing test. This simplicity metric, as with all others in this library,
/// are meant to be generally aligned with what will be most helpful most often
/// in a typical use case.
///
/// To me, it's pretty clear that all things being equal, a negative value is
/// more complicated than a positive value. It's pretty obvious to me that 5 is
/// simpler than -5. But I also don't think *all* negative values are more
/// complicated than *all* positive ones - It's equally obvious to me that -1 is
/// simpler than 4817326. That said, I don't think being negative should just be
/// a "tiebreaker" when the magnitude is the same - I would argue that 12923 is
/// simpler than -12921, even though the latter has a smaller absolute value.
///
/// For entirely made up reasons and because I feel like it's "about right", I
/// have decided that a negative value is around the same complexity as it's
/// square - that is, -5 is approximately as complicated as 25, and -12948 is
/// about as complicated as 167650704. Since it's useful to always be able to
/// pick a winner when comparing two values for simplicity, in the event of a
/// tie (like -5 and 25), I've decided the positive number is simpler (this is
/// useful because I certainly want 1 to be simpler than -1). In other words,
/// the magnitude of a negative number must be less than the square root of some
/// positive number in order to be considered simpler. So -5 is simpler than 26,
/// but not simpler than 25.
///
/// The reason I landed on square/sqrt is because it is the function that will
/// approximately double or halve the number of digits when going between
/// positive and negative values of equal complexity, which felt "about right"
/// to me (proving this uses some cute log rules, fun activity for the reader).
///
/// # Phases
/// More details on each phase can be found in their respective modules, but
/// here's a high-level overview of each:
/// - [Try simplest](try_simplest) - Try the simplest value immediately
/// - [Shrink magnitude](shrink_magnitude) - Shrink the magnitude (absolute
///   value) of the current simplest failing value, maintaining its sign.
/// - [Flip sign](flip_sign) - Attempt to find a simpler failing value with the
///   opposite sign of the current simplest failing value.
/// - [Done](done) - Done shrinking
///
/// # Phase transitions
/// When one phase finishes, it will move on to another. Often this is the next
/// one in the sequence, but some phases will jump forward or backward under
/// certain circumstances.
///
/// For example, if "Try simplest" finds that the simplest value fails, it
/// immediately jumps to "Done", since there is no point trying more complicated
/// values when we know the simplest value is failing. As another example, the
/// "Flip sign" phase will jump *backwards* to "Shrink magnitude" if it finds
/// a failing value, with the idea being that we've discovered failing values
/// within the opposite sign, so it's worth spending some time searching for an
/// even simpler value with this new sign.
pub enum RangeInclusiveShrinkerSigned<T> {
    TrySimplest(TrySimplest<T>),
    Done(Done<T>),
}

macro_rules! shrinker {
    ($($t: ty),+$(,)?) => {$(
        impl RangeInclusiveShrinkerSigned<$t> {
            /// Constructs a new [`RangeInclusiveShrinkerSigned`].
            ///
            /// # Panics
            /// if the invariant `min <= simplest_known_failing <= max` is not
            /// true.
            pub fn new(simplest_known_failing: $t, (min, max): ($t, $t)) -> Self {
                debug_assert!(min <= simplest_known_failing && simplest_known_failing <= max);

                // If the simplest known failing value is the simplest possible
                // value, there's no point shrinking - we're done.
                if simplest_known_failing == Self::simplest_in_range(min, max) {
                    return Self::Done(Done::new());
                }

                // Invariant: upheld by our caller, and checked with the
                // assertion above.
                Self::TrySimplest(TrySimplest::<$t>::new(simplest_known_failing, (min, max)))
            }

            #[expect(unused, reason = "will be used by flip sign step")]
            /// Returns the most complex (furthest from 0, in this case) value
            /// with the opposite sign of `n` which is still simpler than `n`,
            /// or [`None`] if there is no simpler value of the opposite sign.
            ///
            /// For more information on what makes a value "simpler" despite
            /// different signs, see the "How sign effects simplicity" section
            /// on [`RangeInclusiveShrinkerSigned`].
            ///
            /// Note that the only three ways for there to be no simpler value
            /// with the opposite sign of `n` are:
            /// 1. `n == 0` (There is no "sign", nor is anything simpler than 0)
            /// 2. `n == 1` (No negative value is simpler than 1)
            /// 3. `min..=max` contains no values of the opposite sign
            ///
            /// # Panics
            /// if `min <= n <= max` is not true.
            fn flipped_simpler(n: $t, (min, max): ($t, $t)) -> Option<$t> {
                assert!(min <= n && n <= max);

                (n != 0 && n != 1 && min < 0 && max > 0).then(|| {
                    let unclamped = if n < 0 {
                        // For negative values, their square is simpler, but
                        // anything larger than that is more complex
                        n.saturating_pow(2)
                    }
                    else {
                        // For positive values, there are two cases: either they
                        // are a perfect square, or they are not. If they are a
                        // perfect square, then by the tiebreak rules their
                        // (negated) square root is barely not simpler, so we
                        // have to step down in magnitude by one. If the value
                        // is not a perfect square, then its (negated) integer
                        // square root is simpler (but maximally complicated).
                        // With the way the rounding works out, taking the
                        // integer square root of `n - 1` actually handles both
                        // cases correctly. No branching, yippie!
                        -(n - 1).isqrt()
                    };

                    // Clamp the unclamped value to `min..=max`
                    unclamped.clamp(min, max)
                })
            }

            /// Returns the median of three numbers.
            fn median_of_three(a: $t, b: $t, c: $t) -> $t {
                <$t>::max(<$t>::min(a, b), <$t>::min(<$t>::max(a, b), c))
            }

            /// Returns the simplest value within the given range.
            ///
            /// For more information on what makes a value "simpler" despite
            /// different signs, see the "How sign effects simplicity" section
            /// on [`RangeInclusiveShrinkerSigned`].
            fn simplest_in_range(min: $t, max: $t) -> $t {
                Self::median_of_three(min, max, 0)
            }
        }

        impl Shrinker for RangeInclusiveShrinkerSigned<$t> {
            type InputSource = $t;

            fn current_attempt(&self) -> Option<Self::InputSource> {
                match self {
                    Self::TrySimplest(phase) => phase.current_attempt(),
                    Self::Done(phase) => phase.current_attempt(),
                }
            }

            fn update(&mut self, current_attempt_passed: bool) {
                *self = match self {
                    Self::TrySimplest(phase) => phase.next_phase(current_attempt_passed),
                    Self::Done(phase) => phase.next_phase(current_attempt_passed),
                }
            }

            fn into_observations(self) -> Vec<Observation> {
                // TODO(ichen): useful observations
                Vec::new()
            }
        }
    )+};
}

shrinker! { i8, i16, i32, i64, i128, isize }
