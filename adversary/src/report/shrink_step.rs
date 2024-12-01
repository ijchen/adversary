#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShrinkStep<T> {
    pub value: T,
    pub just_informational: bool,
    pub test_passed: bool,
}

impl<T> ShrinkStep<T> {
    pub fn new(value: T, just_informational: bool, test_passed: bool) -> Self {
        Self {
            value,
            just_informational,
            test_passed,
        }
    }
}
