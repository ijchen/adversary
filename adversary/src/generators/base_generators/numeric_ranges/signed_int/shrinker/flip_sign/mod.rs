use consecutive::Consecutive;
use spread_out::SpreadOut;

use super::RangeInclusiveShrinkerSigned;

mod consecutive;
mod spread_out;

/// Implementation of the "Flip sign" phase of signed integer shrinking.
///
/// This phase is split up into multiple distinct "sub-phases". The bulk of the
/// logic is actually contained within submodules for each sub-phase - this enum
/// really just organizes them all together.
///
/// # Sub-phases
/// More details on each sub-phase can be found in their respective modules, but
/// here's a high-level overview of each:
/// - [Spread out](spread_out) - TODO
/// - [Consecutive](consecutive) - TODO
/// - [Done](done) - Done with this phase
///
/// # Sub-phase transitions
/// When one sub-phase finishes, it will move on to another. Often this is the
/// next one in the sequence, but some phases will jump forward or backward
/// under certain circumstances.
///
/// For example, the "Spread out" sub-phase, if it finds a failing value, will
/// immediately jump out of the "Flip sign" phase backwards to the "Shrink
/// magnitude" outer phase, skipping the "Consecutive" sub-phase entirely.
pub enum FlipSign<I> {
    SpreadOut(SpreadOut<I>),
    Consecutive(Consecutive<I>),
}

macro_rules! flip_sign {
    ($($i:ty = $u:ty),+$(,)?) => {$(
        impl FlipSign<$i> {
            /// Helper to get the most complex value of the opposite sign that
            /// is still simpler than the given `n`, or None if there is no
            /// simpler value than `n` with the opposite sign.
            ///
            /// See the "How sign effects simplicity" section of the
            /// documentation on [`RangeInclusiveShrinkerSigned`] for details of
            /// how this is determined.
            ///
            /// # Panics
            /// if any of the following invariants are not true:
            /// - `min < 0 < max`
            /// - `min <= n <= max`
            fn most_complex_value_simpler_than(min: $i, max: $i, n: $i) -> Option<$i> {
                assert!(min < 0 && 0 < max);
                assert!(min <= n && n <= max);

                match std::cmp::Ord::cmp(&n, &0) {
                    // If n == 0, there is no simpler value of the opposite sign
                    std::cmp::Ordering::Equal => None,

                    // If n < 0, choose n^2 (clamped to max)
                    // Note: if this saturates, we'll clamp to max anyway
                    std::cmp::Ordering::Less => Some(n.saturating_pow(2).min(max)),

                    // If n > 0, choose isqrt(n - 1) (or None if n == 1)
                    // Note: isqrt(n - 1) will always be isqrt(n), except when n
                    // is a perfect square - in that case, we *don't* want to
                    // use sqrt(n), but instead use sqrt(n) - 1. isqrt(n - 1)
                    // gives us the right answer in both cases.
                    // TODO: prove this, I'm not 100% convinced it's true
                    std::cmp::Ordering::Greater => {
                        // TODO: use <$i>::isqrt once it's stabilized in 0.84.0
                        // Note: Both the subtraction and the isqrt can't
                        // overflow because we know n > 0 (by the match arm)
                        (n != 1).then(|| integer_sqrt::IntegerSquareRoot::integer_sqrt(&(n - 1)))
                    }
                }
            }

            /// Constructs a new [`FlipSign`] in the starting sub-phase.
            ///
            /// # Panics
            /// if any of the following invariants are not true:
            /// - `min < 0 < max`
            /// - `min <= simplest_known_failing <= max`
            pub fn new(min: $i, max: $i, simplest_known_failing: $i) -> Option<Self> {
                assert!(min < 0 && 0 < max);
                assert!(min <= simplest_known_failing && simplest_known_failing <= max);

                if let Some(spread_out) = SpreadOut::<$i>::new(min, max, simplest_known_failing) {
                    // TODO: invariants
                    Some(Self::SpreadOut(spread_out))
                }
                else if let Some(consecutive) = Consecutive::<$i>::new(simplest_known_failing, min, max) {
                    // TODO: invariants
                    Some(Self::Consecutive(consecutive))
                }
                else {
                    None
                }
            }

            pub fn current_attempt(&self) -> Option<$i> {
                match self {
                    Self::SpreadOut(phase) => phase.current_attempt(),
                    Self::Consecutive(phase) => phase.current_attempt(),
                }
            }

            pub fn next_phase(&self, current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<$i, $u> {
                match self {
                    Self::SpreadOut(phase) => phase.next_phase(current_attempt_passed),
                    Self::Consecutive(phase) => phase.next_phase(current_attempt_passed),
                }
            }
        }
    )+};
}

flip_sign! {
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize,
}
