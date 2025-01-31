//! Implementation of the shrinking algorithm for unsigned integers.
//!
//! The shrinking is broken into multiple distinct "phases". Each phase is
//! implemented within its own submodule, for organization. The overall "main"
//! shrinker in this module just delegates to the submodules. For a high-level
//! overview of the phases, see the documentation on
//! [`RangeInclusiveShrinkerUnsigned`]. For more details on each individual
//! phase, see the documentation in their respective modules.

mod binary_search;
mod consecutive;
mod done;
mod spread_out;
mod try_simplest;

use binary_search::BinarySearch;
use consecutive::Consecutive;
use done::Done;
use spread_out::SpreadOut;
use try_simplest::TrySimplest;

use crate::{
    generators::base_generators::numeric_ranges::RangeInclusiveGen, report::Observation,
    shrinker::Shrinker, ValueGen,
};
/// The shrinker implementation for unsigned integer range
/// [`ValueGen`](crate::ValueGen)s.
///
/// Shrinking is split up into multiple distinct "phases". The bulk of the logic
/// is actually contained within submodules for each phase - this enum really
/// just organizes them all together.
///
/// # Phases
/// More details on each phase can be found in their respective modules, but
/// here's a high-level overview of each:
/// - [Try simplest](try_simplest) - try the simplest value immediately
/// - [Binary search](binary_search) - binary search to a simpler value
/// - [Spread out](spread_out) - Try a spread out sampling of values between the
///   minimum and the simplest known failing value
/// - [Consecutive](consecutive) - Try a run of consecutive values just below
///   the simplest known failing value
/// - [Done](done) - Done shrinking
///
/// # Phase transitions
/// When one phase finishes, it will move on to another. Often this is the next
/// one in the sequence, but some phases will jump forward or backward under
/// certain circumstances.
///
/// For example, if "Try simplest" finds that the simplest value fails, it
/// immediately jumps to "Done", since there is no point trying larger values
/// when we know the simplest value is failing. As another example, both the
/// "Spread out" and "Consecutive" phases will jump *backwards* to "Binary
/// search" if they find failing values, with the idea being that we've
/// discovered values that binary search missed, so it maybe be worth trying
/// binary search again with a more refined range.

#[derive(Debug, Clone)]
pub enum RangeInclusiveShrinkerUnsigned<T> {
    TrySimplest(TrySimplest<T>),
    BinarySearch(BinarySearch<T>),
    SpreadOut(SpreadOut<T>),
    Consecutive(Consecutive<T>),
    Done(Done<T>),
}

macro_rules! shrinker {
    ($($t: ty),+$(,)?) => {$(
        impl RangeInclusiveShrinkerUnsigned<$t> {
            pub fn new(min: $t, simplest_known_failing: $t) -> Self {
                assert!(min <= simplest_known_failing);

                // If the simplest known failing value is the minimum possible
                // value, there's no point shrinking at all - we're done
                if simplest_known_failing == min {
                    Self::Done(Done::new())
                } else {
                    Self::TrySimplest(TrySimplest::<$t>::new(min, simplest_known_failing))
                }
            }
        }

        impl Shrinker<RangeInclusiveGen<$t>> for RangeInclusiveShrinkerUnsigned<$t> {
            fn current_attempt(&self) -> Option<<RangeInclusiveGen<$t> as ValueGen>::Value> {
                match self {
                    Self::TrySimplest(phase) => phase.current_attempt(),
                    Self::BinarySearch(phase) => phase.current_attempt(),
                    Self::SpreadOut(phase) => phase.current_attempt(),
                    Self::Consecutive(phase) => phase.current_attempt(),
                    Self::Done(phase) => phase.current_attempt(),
                }
            }

            fn update(&mut self, _generator: &RangeInclusiveGen<$t>, current_attempt_passed: bool) {
                *self = match self {
                    Self::TrySimplest(phase) => phase.next_phase(current_attempt_passed),
                    Self::BinarySearch(phase) => phase.next_phase(current_attempt_passed),
                    Self::SpreadOut(phase) => phase.next_phase(current_attempt_passed),
                    Self::Consecutive(phase) => phase.next_phase(current_attempt_passed),
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

shrinker! { u8, u16, u32, u64, u128, usize }
