//! Implementation of the shrinking algorithm for signed integers.
//!
//! The shrinking is broken into multiple distinct "phases". Each phase is
//! implemented within its own submodule, for organization. The overall "main"
//! shrinker in this module just delegates to the submodules. For a high-level
//! overview of the phases, see the documentation on
//! [`RangeInclusiveShrinkerSigned`]. For more details on each individual phase,
//! see the documentation in their respective modules.

mod done;

use done::Done;

use crate::{report::Observation, shrinker::Shrinker};

/// The shrinker implementation for signed integer range
/// [`InputGenerator`](crate::InputGenerator)s.
///
/// Shrinking is split up into multiple distinct "phases". The bulk of the logic
/// is actually contained within submodules for each phase - this enum really
/// just organizes them all together.
///
/// # Phases
/// More details on each phase can be found in their respective modules, but
/// here's a high-level overview of each:
/// - TODO: the rest of them
/// - [Done](done) - Done shrinking
///
/// # Phase transitions
/// When one phase finishes, it will move on to another. Often this is the next
/// one in the sequence, but some phases will jump forward or backward under
/// certain circumstances.
///
/// For example, TODO: examples
// TODO: implement real phases
pub enum RangeInclusiveShrinkerSigned<T> {
    Done(Done<T>),
}

macro_rules! shrinker {
    ($($t: ty),+$(,)?) => {$(
        impl RangeInclusiveShrinkerSigned<$t> {
            pub fn new() -> Self {
                Self::Done(Done::new())
            }
        }

        impl Shrinker for RangeInclusiveShrinkerSigned<$t> {
            type InputSource = $t;

            fn current_attempt(&self) -> Option<Self::InputSource> {
                match self {
                    Self::Done(phase) => phase.current_attempt(),
                }
            }

            fn update(&mut self, current_attempt_passed: bool) {
                *self = match self {
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
