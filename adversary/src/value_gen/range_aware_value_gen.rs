use super::ValueGen;

pub trait RangeAwareValueGen: ValueGen {
    /// Returns whether or not a given value is in the range of possible values produced by this
    /// [`ValueGen`].
    ///
    /// This function should always be deterministic and correct - it is a logic error to implement
    /// this trait for a [`ValueGen`] that cannot reliably determine whether or not any given value
    /// is in its range.
    fn value_in_range(&self, value: &Self::Value) -> bool;
}
