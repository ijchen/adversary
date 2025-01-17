use std::marker::PhantomData;

use super::RangeInclusiveShrinkerSigned;

/// Implementation of the "Done" phase of signed integer shrinking.
///
/// # Description
/// This phase is the final phase, used when we're done shrinking. It will never
/// attempt to shrink.
///
/// # Next phase
/// This is the final phase, and will never change to any other phase.
pub struct Done<T, U>(PhantomData<(T, U)>);

#[expect(
    clippy::new_without_default,
    reason = "this struct shouldn't even be public in the first place - and won't be once ATPIT is stabilized."
)]
impl<T, U> Done<T, U> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn current_attempt(&self) -> Option<T> {
        None
    }

    pub fn next_phase(&self, _current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<T, U> {
        RangeInclusiveShrinkerSigned::Done(Self::new())
    }
}
