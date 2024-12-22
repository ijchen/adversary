use std::marker::PhantomData;

use super::RangeInclusiveShrinkerUnsigned;

/// Implementation of the "Done" phase of unsigned integer shrinking.
///
/// # Description
/// This phase is the final phase, used when we're done shrinking. It will never
/// attempt to shrink.
///
/// # Next phase
/// This is the final phase, and will never change to any other phase.
pub struct Done<T>(PhantomData<T>);

impl<T> Done<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn current_attempt(&self) -> Option<T> {
        None
    }

    pub fn next_phase(&self, _current_attempt_passed: bool) -> RangeInclusiveShrinkerUnsigned<T> {
        RangeInclusiveShrinkerUnsigned::Done(Self::new())
    }
}
