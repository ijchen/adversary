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
pub struct Done<I, U>(PhantomData<(I, U)>);

impl<I, U> Done<I, U> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn current_attempt(&self) -> Option<I> {
        None
    }

    pub fn next_phase(&self, _current_attempt_passed: bool) -> RangeInclusiveShrinkerSigned<I, U> {
        RangeInclusiveShrinkerSigned::Done(Self::new())
    }
}
