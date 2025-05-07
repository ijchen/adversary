use super::ValueGen;

pub trait RangeAwareValueGen: ValueGen {
    /// Returns whether or not a given value is in the range of possible values
    /// produced by this [`ValueGen`].
    fn value_in_range(&self, value: &Self::Value) -> bool;
}
