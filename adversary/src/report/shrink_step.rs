#[cfg(test)]
use crate::report::{FailureCause, PanicData};

use super::TestOutcome;

#[derive(Debug)]
pub struct ShrinkStep<T> {
    pub value: T,
    pub just_informational: bool,
    pub outcome: TestOutcome,
}

impl<T> ShrinkStep<T> {
    pub fn new(value: T, just_informational: bool, outcome: TestOutcome) -> Self {
        Self {
            value,
            just_informational,
            outcome,
        }
    }

    // TODO(ichen): this is just a testing tool, but can we do it more cleanly?
    // I'm almost envisioning some crate level macro that asserts a given
    // Vec<ShrinkStep> matches some expected value... 🤔
    #[cfg(test)]
    pub(crate) fn try_eq(&self, other: &Self) -> Option<bool>
    where
        T: PartialEq,
    {
        if self.value != other.value {
            return Some(false);
        }

        if self.just_informational != other.just_informational {
            return Some(false);
        }

        use TestOutcome as T;
        let (self_cause, other_cause) = match (&self.outcome, &other.outcome) {
            (T::Passed, T::Passed) => return Some(true),
            (T::Failed { cause: s }, T::Failed { cause: o }) => (s, o),
            _ => return Some(false),
        };

        fn panic_data_eq(a: &PanicData, b: &PanicData) -> Option<bool> {
            if a.location != b.location {
                return Some(false);
            }

            if a.payload.type_id() != b.payload.type_id() {
                return Some(false);
            }

            if !a.payload.is::<&str>() && !a.payload.is::<String>() {
                return None;
            }

            Some(a.payload_as_string().unwrap() == b.payload_as_string().unwrap())
        }

        use FailureCause as F;
        match (self_cause, other_cause) {
            (F::NormalFailure, F::NormalFailure) => Some(true),
            (F::UnexpectedPanic { panic_data: s }, F::UnexpectedPanic { panic_data: o }) => {
                panic_data_eq(s, o)
            }
            (F::ExpectedPanic { panic_data: s }, F::ExpectedPanic { panic_data: o }) => {
                panic_data_eq(s, o)
            }
            (F::WrongPanicMessage { panic_data: s }, F::WrongPanicMessage { panic_data: o }) => {
                panic_data_eq(s, o)
            }
            (F::ResultErr { .. }, F::ResultErr { .. }) => None,
            _ => Some(false),
        }
    }
}
